// Copyright © 2021-2024 HQS Quantum Simulations GmbH. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing permissions and
// limitations under the License.

use qoqo_calculator::CalculatorFloat;
use roqoqo::{operations::*, Circuit, RoqoqoBackendError};
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    str::FromStr,
    vec,
};

use crate::{
    call_operation, format_arg, gate_declaration, pre_process_circuit, NO_CALL_OPERATIONS,
    NO_DECLARATION_OPERATIONS, NUMBER_LABEL, NUMBER_VARS,
};

/// QIR backend to qoqo
///
/// This backend to roqoqo produces QIR output which can be exported.
///
/// This backend takes a roqoqo circuit and returns a QIR String or writes a QIR file
/// containing the translated circuit. The circuit itself is translated using the roqoqo-qir
/// interface. In this backend, the initialization sets up the relevant parameters and the run
/// function calls the QIR interface and writes the QIR file, which is saved to be used by the
/// user on whatever platform they see fit.
///
///

fn process_operation_circuit(
    circuit: &Circuit,
    already_seen_declarations: &mut Vec<String>,
    declarations: &mut String,
) -> Result<(), RoqoqoBackendError> {
    for operation in pre_process_circuit(circuit)?.iter() {
        if !already_seen_declarations.contains(&operation.hqslang().to_string()) {
            let mut continue_process = false;
            if let Operation::GateDefinition(gate_definition) = operation {
                if !already_seen_declarations.contains(gate_definition.name()) {
                    already_seen_declarations.push(gate_definition.name().to_owned());
                    continue_process = true;
                }
            } else {
                already_seen_declarations.push(operation.hqslang().to_string());
                continue_process = true;
            }

            if continue_process {
                match operation {
                    Operation::GateDefinition(gate_definition) => process_operation_circuit(
                        gate_definition.circuit(),
                        already_seen_declarations,
                        declarations,
                    )?,
                    Operation::PragmaConditional(pragma_conditional) => process_operation_circuit(
                        pragma_conditional.circuit(),
                        already_seen_declarations,
                        declarations,
                    )?,
                    Operation::PragmaLoop(pragma_loop) => process_operation_circuit(
                        pragma_loop.circuit(),
                        already_seen_declarations,
                        declarations,
                    )?,
                    Operation::SqrtPauliX(_) | Operation::InvSqrtPauliX(_) => {
                        process_operation_circuit(
                            &[Operation::from(RotateX::new(0, CalculatorFloat::ZERO))]
                                .into_iter()
                                .collect(),
                            already_seen_declarations,
                            declarations,
                        )?
                    }
                    Operation::PhaseShiftState1(_) => process_operation_circuit(
                        &[Operation::from(RotateZ::new(0, CalculatorFloat::ZERO))]
                            .into_iter()
                            .collect(),
                        already_seen_declarations,
                        declarations,
                    )?,
                    Operation::ControlledPauliY(_) => {
                        if !already_seen_declarations.contains(&"s_adj".to_owned()) {
                            already_seen_declarations.push("s_adj".to_owned());
                            declarations
                                .push_str("declare void @__quantum__qis__s__adj(%Qubit*)\n");
                        }
                        process_operation_circuit(
                            &[
                                Operation::from(CNOT::new(0, 1)),
                                Operation::from(SGate::new(0)),
                            ]
                            .into_iter()
                            .collect(),
                            already_seen_declarations,
                            declarations,
                        )?
                    }
                    _ => {}
                }
                declarations.push_str(&gate_declaration(operation)?);
                if !declarations.is_empty()
                    && !NO_DECLARATION_OPERATIONS.contains(&operation.hqslang())
                {
                    declarations.push('\n');
                }
            }
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Backend {
    /// Name of the profile to use.
    /// QIR profiles defines a subset of the QIR specification
    qir_profile: QirProfile,
    /// Which version of QIR to use
    qir_version: QirVersion,
}

impl Backend {
    /// Create a new QIR backend
    ///
    /// # Arguments
    ///
    /// * `qir_profile` - The name of the profile.
    /// * `qir_version` - The version of QIR.
    pub fn new(
        qir_profile: Option<String>,
        qir_version: Option<String>,
    ) -> Result<Self, RoqoqoBackendError> {
        Ok(Self {
            qir_profile: QirProfile::from_str(&qir_profile.unwrap_or("base_profile".to_owned()))?,
            qir_version: QirVersion::from_str(&qir_version.unwrap_or("0.1".to_owned()))?,
        })
    }

    /// Translates a Circuit to a valid QIR string.
    ///
    ///
    /// # Arguments
    ///
    /// * `circuit` - The Circuit items that is translated
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The valid QIR string
    /// * `RoqoqoBackendError::OperationNotInBackend` - An operation is not available on the backend
    pub fn circuit_to_qir_str(
        &self,
        circuit: &Circuit,
        measure_all: bool,
    ) -> Result<String, RoqoqoBackendError> {
        *NUMBER_VARS.lock().unwrap() = 0;
        *NUMBER_LABEL.lock().unwrap() = 0;
        let pre_processed_circuit = pre_process_circuit(circuit)?;
        match self.qir_profile {
            QirProfile::BaseProfile => {
                let mut has_measurements = false;
                let mut number_qubits_required = 0;
                let mut number_bits_required = 0;
                let mut definitions = "%Qubit = type opaque\n".to_owned();
                let mut already_seen_declarations: Vec<String> = vec![];
                let mut declarations = "".to_owned();
                let mut main = "define void @main() #0 {\nentry:\n".to_owned();

                for op in pre_processed_circuit.iter() {
                    // Taking note of the maximum number of qubits involved in the circuit for registers definition
                    if let InvolvedQubits::Set(involved_qubits) = op.involved_qubits() {
                        number_qubits_required =
                            number_qubits_required.max(match involved_qubits.iter().max() {
                                None => 0,
                                Some(n) => *n + 1,
                            })
                    }

                    if let Operation::MeasureQubit(measure_qubit) = op {
                        has_measurements = true;
                        number_bits_required =
                            number_bits_required.max(measure_qubit.readout_index().to_owned() + 1);
                    }
                    if let Operation::PragmaConditional(conditional) = op {
                        has_measurements = true;
                        number_bits_required =
                            number_bits_required.max(conditional.condition_index().to_owned() + 1);
                    }
                    // Appending gate declaration if not already seen before
                    if !already_seen_declarations.contains(&op.hqslang().to_string()) {
                        let mut continue_process = false;
                        if let Operation::GateDefinition(gate_definition) = op {
                            if !already_seen_declarations.contains(gate_definition.name()) {
                                already_seen_declarations.push(gate_definition.name().to_owned());
                                continue_process = true;
                            }
                        } else {
                            already_seen_declarations.push(op.hqslang().to_string());
                            continue_process = true;
                        }

                        if continue_process {
                            match op {
                                Operation::GateDefinition(gate_definition) => {
                                    process_operation_circuit(
                                        gate_definition.circuit(),
                                        &mut already_seen_declarations,
                                        &mut declarations,
                                    )?
                                }
                                Operation::PragmaConditional(pragma_conditional) => {
                                    if !already_seen_declarations
                                        .contains(&"read_result".to_owned())
                                    {
                                        already_seen_declarations.push("read_result".to_owned());
                                        declarations.push_str("declare i1 @__quantum__qis__read_result__body(%Result*)\n");
                                    }
                                    process_operation_circuit(
                                        pragma_conditional.circuit(),
                                        &mut already_seen_declarations,
                                        &mut declarations,
                                    )?
                                }
                                Operation::PragmaLoop(pragma_loop) => process_operation_circuit(
                                    pragma_loop.circuit(),
                                    &mut already_seen_declarations,
                                    &mut declarations,
                                )?,
                                Operation::SqrtPauliX(_) | Operation::InvSqrtPauliX(_) => {
                                    process_operation_circuit(
                                        &[Operation::from(RotateX::new(0, CalculatorFloat::ZERO))]
                                            .into_iter()
                                            .collect(),
                                        &mut already_seen_declarations,
                                        &mut declarations,
                                    )?
                                }
                                Operation::PhaseShiftState1(_) => process_operation_circuit(
                                    &[Operation::from(RotateZ::new(0, CalculatorFloat::ZERO))]
                                        .into_iter()
                                        .collect(),
                                    &mut already_seen_declarations,
                                    &mut declarations,
                                )?,
                                Operation::ControlledPauliY(_) => {
                                    if !already_seen_declarations.contains(&"s_adj".to_owned()) {
                                        already_seen_declarations.push("s_adj".to_owned());
                                        declarations.push_str(
                                            "declare void @__quantum__qis__s__adj(%Qubit*)\n",
                                        );
                                    }
                                    process_operation_circuit(
                                        &[
                                            Operation::from(CNOT::new(0, 1)),
                                            Operation::from(SGate::new(0)),
                                        ]
                                        .into_iter()
                                        .collect(),
                                        &mut already_seen_declarations,
                                        &mut declarations,
                                    )?
                                }
                                _ => {}
                            }
                            declarations.push_str(&gate_declaration(op)?);
                            if !declarations.is_empty()
                                && !NO_DECLARATION_OPERATIONS.contains(&op.hqslang())
                            {
                                declarations.push('\n');
                            }
                        }
                    }
                    main.push_str(&call_operation(op)?);
                    if !main.is_empty() && !NO_CALL_OPERATIONS.contains(&op.hqslang()) {
                        main.push('\n');
                    }
                }
                if measure_all {
                    has_measurements = true;
                    if !already_seen_declarations.contains(&"MeasureQubit".to_owned()) {
                        already_seen_declarations.push("MeasureQubit".to_owned());
                        declarations.push_str(
                            "declare void @__quantum__qis__mz__body(%Qubit*, %Result* writeonly) #1\n",
                        );
                    }
                    declarations.push_str("declare void @__quantum__rt__result_record_output(%Result*, i8*)\ndeclare void @__quantum__rt__array_record_output(i64, i8*)\n");
                    let mut record_output = "".to_owned();
                    for qubit in 0..number_qubits_required {
                        main.push_str(&format!(
                            "  call void @__quantum__qis__mz__body(%Qubit* {}, %Result* {}) #1\n",
                            format_arg(&qubit, "Qubit"),
                            format_arg(&qubit, "Result")
                        ));
                        record_output.push_str(&format!(
                            "  call void @__quantum__rt__result_record_output(%Result* {}, i8* null)\n",
                            format_arg(&qubit, "Result")
                        ));
                    }
                    main.push_str(&format!(
                        "  call void @__quantum__rt__array_record_output(i64 {}, i8* null)\n",
                        number_qubits_required
                    ));
                    main.push_str(&record_output);
                    number_bits_required = number_qubits_required.max(number_bits_required)
                }
                if has_measurements {
                    definitions.push_str("%Result = type opaque\n");
                }
                if declarations.ends_with("\n\n") {
                    declarations = declarations.strip_suffix('\n').map(str::to_owned).unwrap();
                }
                declarations = declarations.replace("\n\n\n", "\n\n");
                main.push_str("  ret void\n}\n\n");
                main.push_str(&declarations);
                let mut attributes = format!(
                    "attributes #0 = {{ \"entry_point\" \"required_num_qubits\"=\"{}\" \"required_num_results\"=\"{}\" \"output_labeling_schema\" \"qir_profiles\"=\"base_profile\"{} }}\n",
                    number_qubits_required,
                    number_bits_required,
                    has_measurements.then(|| " \"irreversible\"").unwrap_or("")
                );
                if has_measurements {
                    attributes.push_str("attributes #1 = { \"irreversible\" }\n");
                }
                let mut flags = "!llvm.module.flags = !{!0, !1, !2, !3}\n\n".to_owned();
                flags.push_str(
                    format!(
                        "!0 = !{{i32 1, !\"qir_major_version\", i32 {}}}\n",
                        major_version(self.qir_version)
                    )
                    .as_str(),
                );
                flags.push_str(
                    format!(
                        "!1 = !{{i32 7, !\"qir_minor_version\", i32 {}}}\n",
                        minor_version(self.qir_version)
                    )
                    .as_str(),
                );
                flags.push_str("!2 = !{i32 1, !\"dynamic_qubit_management\", i1 false}\n!3 = !{i32 1, !\"dynamic_result_management\", i1 false}");
                Ok([definitions, main, attributes, flags].join("\n"))
            }
        }
    }

    /// Translates a Circuit to a QIR file.
    ///
    /// # Arguments
    ///
    /// * `circuit` - The Circuit that is translated
    /// * `folder_name` - The name of the folder that is prepended to all filenames.
    /// * `filename` - The name of the file the QIR text is saved to.
    /// * `overwrite` - Whether to overwrite file if it already exists.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The QIR file was correctly written
    /// * `RoqoqoBackendError::FileAlreadyExists` - The file at this location already exists
    pub fn circuit_to_qir_file(
        &self,
        circuit: &Circuit,
        folder_name: &Path,
        filename: &Path,
        overwrite: bool,
        measure_all: bool,
    ) -> Result<(), RoqoqoBackendError> {
        let data: String = self.circuit_to_qir_str(circuit, measure_all)?;

        let output_path: PathBuf = folder_name.join(filename.with_extension("ll"));
        if output_path.is_file() && !overwrite {
            return Err(RoqoqoBackendError::FileAlreadyExists {
                path: output_path.to_str().unwrap().to_string(),
            });
        } else {
            let f = File::create(output_path).expect("Unable to create file");
            let mut f = BufWriter::new(f);
            f.write_all(data.as_bytes()).expect("Unable to write file")
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QirProfile {
    /// QIR base profile: https://github.com/qir-alliance/qir-spec/blob/main/specification/under_development/profiles/Base_Profile.md
    BaseProfile,
}

impl FromStr for QirProfile {
    type Err = RoqoqoBackendError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "base_profile" | "base" | "base profile" => Ok(QirProfile::BaseProfile),
            _ => Err(RoqoqoBackendError::GenericError {
                msg: format!("Profile '{}' not supported", s),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QirVersion {
    /// QIR 0.1
    V0point1,
}

impl FromStr for QirVersion {
    type Err = RoqoqoBackendError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0.1" => Ok(QirVersion::V0point1),
            _ => Err(RoqoqoBackendError::GenericError {
                msg: format!("Version '{}' not supported", s),
            }),
        }
    }
}

fn minor_version(version: QirVersion) -> usize {
    match version {
        QirVersion::V0point1 => 0,
    }
}

fn major_version(version: QirVersion) -> usize {
    match version {
        QirVersion::V0point1 => 1,
    }
}
