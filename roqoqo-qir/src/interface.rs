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

use lazy_static::lazy_static;
use qoqo_calculator::{CalculatorError, CalculatorFloat};
use roqoqo::{operations::*, Circuit, RoqoqoBackendError};
use std::{f64::consts::FRAC_PI_2, sync::Mutex};

lazy_static! {
    pub static ref NUMBER_LABEL: Mutex<u32> = Mutex::new(0);
    pub static ref NUMBER_VARS: Mutex<u32> = Mutex::new(0);
}

pub(crate) const NO_CALL_OPERATIONS: &[&str; 6] = &[
    "GateDefinition",
    "DefinitionFloat",
    "DefinitionUsize",
    "DefinitionBit",
    "DefinitionComplex",
    "Identity",
];

pub(crate) const NO_DECLARATION_OPERATIONS: &[&str; 30] = &[
    "Identity",
    "CallDefinedGate",
    "DefinitionFloat",
    "DefinitionUsize",
    "DefinitionBit",
    "DefinitionComplex",
    "PragmaConditional",
    "PragmaLoop",
    "XY",
    "SqrtPauliX",
    "InvSqrtPauliX",
    "Identity",
    "SWAP",
    "ISwap",
    "SqrtISwap",
    "InvSqrtISwap",
    "FSwap",
    "PMInteraction",
    "GivensRotation",
    "GivensRotationLittleEndian",
    "PhaseShiftedControlledZ",
    "PhaseShiftedControlledPhase",
    "PhaseShiftState1",
    "MolmerSorensenXX",
    "VariableMSXX",
    "ControlledPauliY",
    "ControlledPhaseShift",
    "RotateXY",
    "ControlledControlledPauliZ",
    "ControlledControlledPhaseShift",
];

pub(crate) fn format_arg(arg: &usize, arg_type: &str) -> String {
    format!("inttoptr (i64 {} to %{}*)", arg, arg_type)
}

fn format_calculator(calculator: &CalculatorFloat) -> String {
    match calculator {
        CalculatorFloat::Float(float_value) => {
            if float_value.fract() == 0.0 {
                format!("{:.1}", float_value)
            } else {
                format!("{}", float_value)
            }
        }
        CalculatorFloat::Str(str_value) => match str_value.as_str() {
            "pi" => std::f64::consts::PI.to_string(),
            "-pi" => format!("-{}", std::f64::consts::PI),
            "pi/2" => std::f64::consts::FRAC_PI_2.to_string(),
            "pi/4" => std::f64::consts::FRAC_PI_4.to_string(),
            "-pi/2" => format!("-{}", std::f64::consts::FRAC_PI_2),
            "-pi/4" => format!("-{}", std::f64::consts::FRAC_PI_4),
            _ => format!("%{}", str_value),
        },
    }
}

pub fn pre_process_circuit(circuit: &Circuit) -> Result<Circuit, RoqoqoBackendError> {
    let mut new_circuit = Circuit::new();
    for operation in circuit.iter() {
        match operation {
            Operation::XY(_) => {
                let mut circ = Circuit::new();
                circ.add_operation(RotateX::new(0, CalculatorFloat::FRAC_PI_2));
                circ.add_operation(CNOT::new(0, 1));
                circ.add_operation(RotateX::new(1, CalculatorFloat::from("theta")));
                circ.add_operation(CNOT::new(0, 1));
                circ.add_operation(RotateX::new(0, -CalculatorFloat::FRAC_PI_2));
                new_circuit.add_operation(Operation::from(GateDefinition::new(
                    circ,
                    "xy".to_owned(),
                    vec![0, 1],
                    vec!["theta".to_owned()],
                )));
                new_circuit.add_operation(operation.clone());
            }
            Operation::SWAP(_) => {
                let mut circ = Circuit::new();
                circ.add_operation(CNOT::new(0, 1));
                circ.add_operation(CNOT::new(1, 0));
                circ.add_operation(CNOT::new(0, 1));
                new_circuit.add_operation(Operation::from(GateDefinition::new(
                    circ,
                    "swap".to_owned(),
                    vec![0, 1],
                    vec![],
                )));
                new_circuit.add_operation(operation.clone());
            }
            Operation::ISwap(_) => {
                let mut circ = Circuit::new();
                circ.add_operation(RotateX::new(0, CalculatorFloat::FRAC_PI_2));
                circ.add_operation(CNOT::new(0, 1));
                circ.add_operation(RotateX::new(0, -CalculatorFloat::FRAC_PI_2));
                circ.add_operation(RotateY::new(1, -CalculatorFloat::FRAC_PI_2));
                circ.add_operation(CNOT::new(0, 1));
                circ.add_operation(RotateX::new(0, -CalculatorFloat::FRAC_PI_2));
                new_circuit.add_operation(Operation::from(GateDefinition::new(
                    circ,
                    "iswap".to_owned(),
                    vec![0, 1],
                    vec![],
                )));
                new_circuit.add_operation(operation.clone());
            }
            Operation::SqrtISwap(_) => {
                let mut circ = Circuit::new();
                circ.add_operation(RotateX::new(0, CalculatorFloat::FRAC_PI_2));
                circ.add_operation(CNOT::new(0, 1));
                circ.add_operation(RotateX::new(0, -CalculatorFloat::FRAC_PI_4));
                circ.add_operation(RotateY::new(1, -CalculatorFloat::FRAC_PI_4));
                circ.add_operation(CNOT::new(0, 1));
                circ.add_operation(RotateX::new(0, -CalculatorFloat::FRAC_PI_2));
                new_circuit.add_operation(Operation::from(GateDefinition::new(
                    circ,
                    "siswap".to_owned(),
                    vec![0, 1],
                    vec![],
                )));
                new_circuit.add_operation(operation.clone());
            }
            Operation::InvSqrtISwap(_) => {
                let mut circ = Circuit::new();
                circ.add_operation(RotateX::new(0, CalculatorFloat::FRAC_PI_2));
                circ.add_operation(CNOT::new(0, 1));
                circ.add_operation(RotateX::new(0, CalculatorFloat::FRAC_PI_4));
                circ.add_operation(RotateY::new(1, CalculatorFloat::FRAC_PI_4));
                circ.add_operation(CNOT::new(0, 1));
                circ.add_operation(RotateX::new(0, -CalculatorFloat::FRAC_PI_2));
                new_circuit.add_operation(Operation::from(GateDefinition::new(
                    circ,
                    "siswap_adj".to_owned(),
                    vec![0, 1],
                    vec![],
                )));
                new_circuit.add_operation(operation.clone());
            }
            Operation::FSwap(_) => {
                let mut circ = Circuit::new();
                circ.add_operation(RotateZ::new(0, -CalculatorFloat::FRAC_PI_2));
                circ.add_operation(RotateZ::new(1, -CalculatorFloat::FRAC_PI_2));
                circ.add_operation(RotateX::new(0, CalculatorFloat::FRAC_PI_2));
                circ.add_operation(CNOT::new(0, 1));
                circ.add_operation(RotateX::new(0, -CalculatorFloat::FRAC_PI_2));
                circ.add_operation(RotateY::new(1, -CalculatorFloat::FRAC_PI_2));
                circ.add_operation(CNOT::new(0, 1));
                circ.add_operation(RotateX::new(0, -CalculatorFloat::FRAC_PI_2));
                new_circuit.add_operation(Operation::from(GateDefinition::new(
                    circ,
                    "fswap".to_owned(),
                    vec![0, 1],
                    vec![],
                )));
                new_circuit.add_operation(operation.clone());
            }
            Operation::PMInteraction(_) => {
                let mut circ = Circuit::new();
                circ.add_operation(RotateX::new(0, CalculatorFloat::FRAC_PI_2));
                circ.add_operation(CNOT::new(0, 1));
                circ.add_operation(RotateX::new(0, CalculatorFloat::from("theta")));
                circ.add_operation(RotateY::new(1, CalculatorFloat::from("theta")));
                circ.add_operation(CNOT::new(0, 1));
                circ.add_operation(RotateX::new(0, -CalculatorFloat::FRAC_PI_2));
                new_circuit.add_operation(Operation::from(GateDefinition::new(
                    circ,
                    "pmint".to_owned(),
                    vec![0, 1],
                    vec!["theta".to_owned()],
                )));
                new_circuit.add_operation(operation.clone());
            }
            Operation::GivensRotation(_) => {
                let mut circ = Circuit::new();
                circ.add_operation(RotateZ::new(1, CalculatorFloat::from("phi_pi_over_2")));
                circ.add_operation(RotateX::new(0, CalculatorFloat::FRAC_PI_2));
                circ.add_operation(CNOT::new(0, 1));
                circ.add_operation(RotateX::new(0, CalculatorFloat::from("minus_theta")));
                circ.add_operation(RotateY::new(1, CalculatorFloat::from("minus_theta")));
                circ.add_operation(CNOT::new(0, 1));
                circ.add_operation(RotateX::new(0, -CalculatorFloat::FRAC_PI_2));
                circ.add_operation(RotateZ::new(1, -CalculatorFloat::FRAC_PI_2));
                new_circuit.add_operation(Operation::from(GateDefinition::new(
                    circ,
                    "gvnsrot".to_owned(),
                    vec![0, 1],
                    vec!["minus_theta".to_owned(), "phi_pi_over_2".to_owned()],
                )));
                new_circuit.add_operation(operation.clone());
            }
            Operation::GivensRotationLittleEndian(_) => {
                let mut circ = Circuit::new();
                circ.add_operation(RotateZ::new(0, -CalculatorFloat::FRAC_PI_2));
                circ.add_operation(RotateX::new(0, CalculatorFloat::FRAC_PI_2));
                circ.add_operation(CNOT::new(0, 1));
                circ.add_operation(RotateX::new(0, CalculatorFloat::from("minus_theta")));
                circ.add_operation(RotateY::new(1, CalculatorFloat::from("minus_theta")));
                circ.add_operation(CNOT::new(0, 1));
                circ.add_operation(RotateX::new(0, -CalculatorFloat::FRAC_PI_2));
                circ.add_operation(RotateZ::new(0, CalculatorFloat::from("phi_pi_over_2")));
                new_circuit.add_operation(Operation::from(GateDefinition::new(
                    circ,
                    "gvnsrotle".to_owned(),
                    vec![0, 1],
                    vec!["minus_theta".to_owned(), "phi_pi_over_2".to_owned()],
                )));
                new_circuit.add_operation(operation.clone());
            }
            Operation::PhaseShiftedControlledZ(_) => {
                let mut circ = Circuit::new();
                circ.add_operation(RotateZ::new(0, CalculatorFloat::FRAC_PI_2));
                circ.add_operation(RotateZ::new(1, CalculatorFloat::FRAC_PI_2));
                circ.add_operation(RotateY::new(1, CalculatorFloat::FRAC_PI_2));
                circ.add_operation(CNOT::new(0, 1));
                circ.add_operation(RotateX::new(1, -CalculatorFloat::FRAC_PI_2));
                circ.add_operation(RotateZ::new(0, -CalculatorFloat::FRAC_PI_2));
                circ.add_operation(RotateY::new(1, -CalculatorFloat::FRAC_PI_2));
                circ.add_operation(RotateZ::new(0, CalculatorFloat::from("phi")));
                circ.add_operation(RotateZ::new(1, CalculatorFloat::from("phi")));
                new_circuit.add_operation(Operation::from(GateDefinition::new(
                    circ,
                    "pscz".to_owned(),
                    vec![0, 1],
                    vec!["phi".to_owned()],
                )));
                new_circuit.add_operation(operation.clone());
            }
            Operation::PhaseShiftedControlledPhase(_) => {
                let mut circ = Circuit::new();
                circ.add_operation(RotateZ::new(0, CalculatorFloat::from("half_theta")));
                circ.add_operation(RotateZ::new(1, CalculatorFloat::from("half_theta")));
                circ.add_operation(CNOT::new(0, 1));
                circ.add_operation(RotateZ::new(1, CalculatorFloat::from("minus_half_theta")));
                circ.add_operation(CNOT::new(0, 1));
                circ.add_operation(RotateZ::new(0, CalculatorFloat::from("phi")));
                circ.add_operation(RotateZ::new(1, CalculatorFloat::from("phi")));
                new_circuit.add_operation(Operation::from(GateDefinition::new(
                    circ,
                    "pscp".to_owned(),
                    vec![0, 1],
                    vec![
                        "half_theta".to_owned(),
                        "minus_half_theta".to_owned(),
                        "phi".to_owned(),
                    ],
                )));
                new_circuit.add_operation(operation.clone());
            }
            Operation::MolmerSorensenXX(_) | Operation::VariableMSXX(_) => {
                let mut circ = Circuit::new();
                circ.add_operation(RotateX::new(0, CalculatorFloat::from("half_theta")));
                circ.add_operation(RotateX::new(1, CalculatorFloat::from("half_theta")));
                circ.add_operation(CNOT::new(0, 1));
                circ.add_operation(RotateX::new(0, CalculatorFloat::from("minus_half_theta")));
                circ.add_operation(RotateX::new(1, CalculatorFloat::from("minus_half_theta")));
                circ.add_operation(CNOT::new(0, 1));
                new_circuit.add_operation(Operation::from(GateDefinition::new(
                    circ,
                    "rxx".to_owned(),
                    vec![0, 1],
                    vec!["half_theta".to_owned(), "minus_half_theta".to_owned()],
                )));
                new_circuit.add_operation(operation.clone());
            }
            Operation::ControlledPhaseShift(_) => {
                let mut circ = Circuit::new();
                circ.add_operation(RotateZ::new(0, CalculatorFloat::from("half_theta")));
                circ.add_operation(CNOT::new(0, 1));
                circ.add_operation(RotateZ::new(1, CalculatorFloat::from("minus_half_theta")));
                circ.add_operation(CNOT::new(0, 1));
                circ.add_operation(RotateZ::new(1, CalculatorFloat::from("half_theta")));
                new_circuit.add_operation(Operation::from(GateDefinition::new(
                    circ,
                    "cp".to_owned(),
                    vec![0, 1],
                    vec!["half_theta".to_owned(), "minus_half_theta".to_owned()],
                )));
                new_circuit.add_operation(operation.clone());
            }
            Operation::RotateXY(_) => {
                let mut circ = Circuit::new();
                circ.add_operation(RotateZ::new(0, CalculatorFloat::from("minus_phi")));
                circ.add_operation(RotateX::new(0, CalculatorFloat::from("theta")));
                circ.add_operation(RotateZ::new(0, CalculatorFloat::from("phi")));
                new_circuit.add_operation(Operation::from(GateDefinition::new(
                    circ,
                    "rxy".to_owned(),
                    vec![0],
                    vec!["theta".to_owned(), "phi".to_owned(), "minus_phi".to_owned()],
                )));
                new_circuit.add_operation(operation.clone());
            }
            Operation::ControlledControlledPauliZ(_) => {
                let mut circ = Circuit::new();
                circ.add_operation(RotateZ::new(1, CalculatorFloat::FRAC_PI_4));
                circ.add_operation(CNOT::new(1, 2));
                circ.add_operation(RotateZ::new(2, -CalculatorFloat::FRAC_PI_4));
                circ.add_operation(CNOT::new(1, 2));
                circ.add_operation(RotateZ::new(2, CalculatorFloat::FRAC_PI_4));
                circ.add_operation(CNOT::new(0, 1));
                circ.add_operation(RotateZ::new(1, -CalculatorFloat::FRAC_PI_4));
                circ.add_operation(CNOT::new(1, 2));
                circ.add_operation(RotateZ::new(2, CalculatorFloat::FRAC_PI_4));
                circ.add_operation(CNOT::new(1, 2));
                circ.add_operation(RotateZ::new(2, -CalculatorFloat::FRAC_PI_4));
                circ.add_operation(CNOT::new(0, 1));
                circ.add_operation(RotateZ::new(0, CalculatorFloat::FRAC_PI_4));
                circ.add_operation(CNOT::new(0, 2));
                circ.add_operation(RotateZ::new(2, -CalculatorFloat::FRAC_PI_4));
                circ.add_operation(CNOT::new(0, 2));
                circ.add_operation(RotateZ::new(2, CalculatorFloat::FRAC_PI_4));
                new_circuit.add_operation(Operation::from(GateDefinition::new(
                    circ,
                    "ccz".to_owned(),
                    vec![0, 1, 2],
                    vec![],
                )));
                new_circuit.add_operation(operation.clone());
            }
            Operation::ControlledControlledPhaseShift(_) => {
                let mut circ = Circuit::new();
                circ.add_operation(RotateZ::new(1, CalculatorFloat::from("frac_theta_4")));
                circ.add_operation(CNOT::new(1, 2));
                circ.add_operation(RotateZ::new(2, CalculatorFloat::from("minus_frac_theta_4")));
                circ.add_operation(CNOT::new(1, 2));
                circ.add_operation(RotateZ::new(2, CalculatorFloat::from("frac_theta_4")));
                circ.add_operation(CNOT::new(0, 1));
                circ.add_operation(RotateZ::new(1, CalculatorFloat::from("minus_frac_theta_4")));
                circ.add_operation(CNOT::new(1, 2));
                circ.add_operation(RotateZ::new(2, CalculatorFloat::from("frac_theta_4")));
                circ.add_operation(CNOT::new(1, 2));
                circ.add_operation(RotateZ::new(2, CalculatorFloat::from("minus_frac_theta_4")));
                circ.add_operation(CNOT::new(0, 1));
                circ.add_operation(RotateZ::new(0, CalculatorFloat::from("frac_theta_4")));
                circ.add_operation(CNOT::new(0, 2));
                circ.add_operation(RotateZ::new(2, CalculatorFloat::from("minus_frac_theta_4")));
                circ.add_operation(CNOT::new(0, 2));
                circ.add_operation(RotateZ::new(2, CalculatorFloat::from("frac_theta_4")));
                new_circuit.add_operation(Operation::from(GateDefinition::new(
                    circ,
                    "ccp".to_owned(),
                    vec![0, 1, 2],
                    vec!["frac_theta_4".to_owned(), "minus_frac_theta_4".to_owned()],
                )));
                new_circuit.add_operation(operation.clone());
            }
            _ => new_circuit.add_operation(operation.clone()),
        }
    }
    Ok(new_circuit)
}

pub fn call_operation(operation: &Operation) -> Result<String, RoqoqoBackendError> {
    match operation {
        Operation::RotateX(op) => Ok(format!(
            "  call void @__quantum__qis__rx__body(double {}, %Qubit* {})",
            format_calculator(op.theta()),
            format_arg(op.qubit(), "Qubit")
        )),
        Operation::RotateY(op) => Ok(format!(
            "  call void @__quantum__qis__ry__body(double {}, %Qubit* {})",
            format_calculator(op.theta()),
            format_arg(op.qubit(), "Qubit")
        )),
        Operation::RotateZ(op) => Ok(format!(
            "  call void @__quantum__qis__rz__body(double {}, %Qubit* {})",
            format_calculator(op.theta()),
            format_arg(op.qubit(), "Qubit")
        )),
        Operation::PauliX(op) => Ok(format!(
            "  call void @__quantum__qis__x__body(%Qubit* {})",
            format_arg(op.qubit(), "Qubit")
        )),
        Operation::PauliY(op) => Ok(format!(
            "  call void @__quantum__qis__y__body(%Qubit* {})",
            format_arg(op.qubit(), "Qubit")
        )),
        Operation::PauliZ(op) => Ok(format!(
            "  call void @__quantum__qis__z__body(%Qubit* {})",
            format_arg(op.qubit(), "Qubit")
        )),
        Operation::Hadamard(op) => Ok(format!(
            "  call void @__quantum__qis__h__body(%Qubit* {})",
            format_arg(op.qubit(), "Qubit")
        )),
        Operation::SGate(op) => Ok(format!(
            "  call void @__quantum__qis__s__body(%Qubit* {})",
            format_arg(op.qubit(), "Qubit")
        )),
        Operation::TGate(op) => Ok(format!(
            "  call void @__quantum__qis__t__body(%Qubit* {})",
            format_arg(op.qubit(), "Qubit")
        )),
        Operation::CNOT(op) => Ok(format!(
            "  call void @__quantum__qis__cnot__body(%Qubit* {}, %Qubit* {})",
            format_arg(op.control(), "Qubit"),
            format_arg(op.target(), "Qubit"),
        )),
        Operation::ControlledPauliZ(op) => Ok(format!(
            "  call void @__quantum__qis__cz__body(%Qubit* {}, %Qubit* {})",
            format_arg(op.control(), "Qubit"),
            format_arg(op.target(), "Qubit"),
        )),
        Operation::SWAP(op) => Ok(format!(
            "  call void @swap(%Qubit* {}, %Qubit* {})",
            format_arg(op.control(), "Qubit"),
            format_arg(op.target(), "Qubit"),
        )),
        Operation::ISwap(op) => Ok(format!(
            "  call void @iswap(%Qubit* {}, %Qubit* {})",
            format_arg(op.control(), "Qubit"),
            format_arg(op.target(), "Qubit"),
        )),
        Operation::SqrtISwap(op) => Ok(format!(
            "  call void @siswap(%Qubit* {}, %Qubit* {})",
            format_arg(op.control(), "Qubit"),
            format_arg(op.target(), "Qubit"),
        )),
        Operation::InvSqrtISwap(op) => Ok(format!(
            "  call void @siswap_adj(%Qubit* {}, %Qubit* {})",
            format_arg(op.control(), "Qubit"),
            format_arg(op.target(), "Qubit"),
        )),
        Operation::FSwap(op) => Ok(format!(
            "  call void @fswap(%Qubit* {}, %Qubit* {})",
            format_arg(op.control(), "Qubit"),
            format_arg(op.target(), "Qubit"),
        )),
        Operation::Toffoli(op) => Ok(format!(
            "  call void @__quantum__qis__ccx__body(%Qubit* {}, %Qubit* {}, %Qubit* {})",
            format_arg(op.control_0(), "Qubit"),
            format_arg(op.control_1(), "Qubit"),
            format_arg(op.target(), "Qubit"),
        )),
        Operation::MeasureQubit(op) => Ok(format!(
            "  call void @__quantum__qis__mz__body(%Qubit* {}, %Result* {}) #1",
            format_arg(op.qubit(), "Qubit"),
            format_arg(op.readout_index(), "Result"),
        )),
        Operation::CallDefinedGate(op) => Ok(format!(
            "  call void @{}({}{}{})",
            op.gate_name(),
            op.free_parameters()
                .iter()
                .map(|param| format!("double {}", format_calculator(param)))
                .collect::<Vec<String>>()
                .join(", "),
            if !op.free_parameters().is_empty() && !op.qubits().is_empty() {
                ", "
            } else {
                ""
            },
            op.qubits()
                .iter()
                .map(|qubit| format!("%Qubit* {}", format_arg(qubit, "Qubit")))
                .collect::<Vec<String>>()
                .join(", ")
        )),
        Operation::PragmaConditional(op) => {
            let mut nb_vars = NUMBER_VARS.lock().unwrap();
            let mut nb_conditional = NUMBER_LABEL.lock().unwrap();
            let mut output_str = format!(
                "  %{} = call i1 @__quantum__qis__read_result__body(%Result* {})\n",
                *nb_vars,
                format_arg(op.condition_index(), "Result")
            );
            output_str.push_str(&format!(
                "  br i1 %{}, label %then{}, label %continue{}\n\nthen{}:\n",
                *nb_vars, *nb_conditional, *nb_conditional, *nb_conditional,
            ));
            for operation in op.circuit().iter() {
                output_str.push_str(&(call_operation(operation)? + "\n"));
            }
            output_str.push_str(&format!(
                "  br label %continue{}\n\ncontinue{}:",
                *nb_conditional, *nb_conditional,
            ));
            *nb_conditional += 1;
            *nb_vars += 1;
            Ok(output_str)
        }
        Operation::PragmaLoop(op) => match op.repetitions() {
            CalculatorFloat::Float(rep) => {
                let mut nb_loop = NUMBER_LABEL.lock().unwrap();
                let mut nb_var = NUMBER_VARS.lock().unwrap();
                let mut output_str =
                    format!("  br label %header{}\n\nheader{}:\n", *nb_loop, *nb_loop);
                output_str.push_str(&format!(
                    "  %{} = phi i64 [ 1, %{} ], [ %{}, %loop{} ]\n",
                    *nb_var,
                    if *nb_loop == 0 {
                        "entry".to_owned()
                    } else {
                        format!("continue{}", *nb_loop - 1)
                    },
                    *nb_var + 2,
                    *nb_loop
                ));
                output_str.push_str(&format!(
                        "  %{} = icmp slt i64 %{}, {}\n  br i1 %{}, label %loop{}, label %continue{}\n\nloop{}:\n",
                        *nb_var + 1,
                        *nb_var,
                        rep.floor() as i32 + 1,
                        *nb_var + 1,
                        *nb_loop,
                        *nb_loop,
                        *nb_loop,
                    ));
                for operation in op.circuit().iter() {
                    output_str.push_str(&(call_operation(operation)? + "\n"));
                }
                output_str.push_str(&format!(
                    "  %{} = add i64 %{}, 1\n  br label %header{}\n\ncontinue{}:",
                    *nb_var + 2,
                    *nb_var,
                    *nb_loop,
                    *nb_loop
                ));
                *nb_var += 3;
                *nb_loop += 1;
                Ok(output_str)
            }
            CalculatorFloat::Str(s) => Err(RoqoqoBackendError::GenericError {
                msg: format!("Used PragmaLoop with an unset parameter: {}", s),
            }),
        },
        Operation::MultiQubitZZ(op) => {
            if op.qubits().len() != 2 {
                Err(RoqoqoBackendError::GenericError {
                    msg: format!(
                        "MultiQubitZZ is only supported for 2 qubits, there are {}.",
                        op.qubits().len()
                    ),
                })
            } else {
                Ok(format!(
                    "  call void @__quantum__qis__rzz__body(double {}, %Qubit* {}, %Qubit* {})",
                    format_calculator(op.theta()),
                    format_arg(op.qubits().first().unwrap(), "Qubit"),
                    format_arg(op.qubits().get(1).unwrap(), "Qubit"),
                ))
            }
        }
        Operation::XY(op) => {
            let minus_half_theta = match CalculatorFloat::from(format_calculator(op.theta())) {
                CalculatorFloat::Float(theta) => CalculatorFloat::from(-0.5 * theta),
                CalculatorFloat::Str(s) => {
                    return Err(RoqoqoBackendError::CalculatorError(
                        CalculatorError::VariableNotSet { name: s.to_owned() },
                    ))
                }
            };
            Ok(format!(
                "  call void @xy(double {}, %Qubit* {}, %Qubit* {})",
                format_calculator(&minus_half_theta),
                format_arg(op.control(), "Qubit"),
                format_arg(op.target(), "Qubit")
            ))
        }
        Operation::SqrtPauliX(op) => Ok(format!(
            "  call void @__quantum__qis__rx__body(double {}, %Qubit* {})",
            format_calculator(&CalculatorFloat::FRAC_PI_2),
            format_arg(op.qubit(), "Qubit")
        )),
        Operation::InvSqrtPauliX(op) => Ok(format!(
            "  call void @__quantum__qis__rx__body(double {}, %Qubit* {})",
            format_calculator(&CalculatorFloat::from(-std::f64::consts::FRAC_PI_2)),
            format_arg(op.qubit(), "Qubit")
        )),
        Operation::PMInteraction(op) => Ok(format!(
            "  call void @pmint(double {}, %Qubit* {}, %Qubit* {})",
            format_calculator(op.t()),
            format_arg(op.control(), "Qubit"),
            format_arg(op.target(), "Qubit"),
        )),
        Operation::GivensRotation(op) => {
            let minus_theta = match CalculatorFloat::from(format_calculator(op.theta())) {
                CalculatorFloat::Float(theta) => CalculatorFloat::from(-theta),
                CalculatorFloat::Str(s) => {
                    return Err(RoqoqoBackendError::CalculatorError(
                        CalculatorError::VariableNotSet { name: s.to_owned() },
                    ))
                }
            };
            let phi_pi_over_2 = match CalculatorFloat::from(format_calculator(op.phi())) {
                CalculatorFloat::Float(phi) => CalculatorFloat::from(phi + FRAC_PI_2),
                CalculatorFloat::Str(s) => {
                    return Err(RoqoqoBackendError::CalculatorError(
                        CalculatorError::VariableNotSet { name: s.to_owned() },
                    ))
                }
            };
            Ok(format!(
                "  call void @gvnsrot(double {}, double {}, %Qubit* {}, %Qubit* {})",
                format_calculator(&minus_theta),
                format_calculator(&phi_pi_over_2),
                format_arg(op.control(), "Qubit"),
                format_arg(op.target(), "Qubit")
            ))
        }
        Operation::GivensRotationLittleEndian(op) => {
            let phi_pi_over_2 = match CalculatorFloat::from(format_calculator(op.phi())) {
                CalculatorFloat::Float(phi) => CalculatorFloat::from(phi + FRAC_PI_2),
                CalculatorFloat::Str(s) => {
                    return Err(RoqoqoBackendError::CalculatorError(
                        CalculatorError::VariableNotSet { name: s.to_owned() },
                    ))
                }
            };
            let minus_theta = match CalculatorFloat::from(format_calculator(op.theta())) {
                CalculatorFloat::Float(phi) => CalculatorFloat::from(-phi),
                CalculatorFloat::Str(s) => {
                    return Err(RoqoqoBackendError::CalculatorError(
                        CalculatorError::VariableNotSet { name: s.to_owned() },
                    ))
                }
            };
            Ok(format!(
                "  call void @gvnsrotle(double {}, double {}, %Qubit* {}, %Qubit* {})",
                format_calculator(&minus_theta),
                format_calculator(&phi_pi_over_2),
                format_arg(op.control(), "Qubit"),
                format_arg(op.target(), "Qubit")
            ))
        }
        Operation::PhaseShiftedControlledZ(op) => Ok(format!(
            "  call void @pscz(double {}, %Qubit* {}, %Qubit* {})",
            format_calculator(op.phi()),
            format_arg(op.control(), "Qubit"),
            format_arg(op.target(), "Qubit"),
        )),
        Operation::PhaseShiftedControlledPhase(op) => {
            let (minus_half_theta, half_theta) =
                match CalculatorFloat::from(format_calculator(op.theta())) {
                    CalculatorFloat::Float(theta) => (
                        CalculatorFloat::from(-0.5 * theta),
                        CalculatorFloat::from(0.5 * theta),
                    ),
                    CalculatorFloat::Str(s) => {
                        return Err(RoqoqoBackendError::CalculatorError(
                            CalculatorError::VariableNotSet { name: s.to_owned() },
                        ))
                    }
                };
            Ok(format!(
                "  call void @pscp(double {}, double {}, double {}, %Qubit* {}, %Qubit* {})",
                format_calculator(&half_theta),
                format_calculator(&minus_half_theta),
                format_calculator(op.phi()),
                format_arg(op.control(), "Qubit"),
                format_arg(op.target(), "Qubit"),
            ))
        }
        Operation::VariableMSXX(op) => {
            let half_theta = match CalculatorFloat::from(format_calculator(op.theta())) {
                CalculatorFloat::Float(theta) => CalculatorFloat::from(0.5 * theta),
                CalculatorFloat::Str(s) => {
                    return Err(RoqoqoBackendError::CalculatorError(
                        CalculatorError::VariableNotSet { name: s.to_owned() },
                    ))
                }
            };
            Ok(format!(
                "  call void @rxx(double {}, double {}, %Qubit* {}, %Qubit* {})",
                format_calculator(&half_theta),
                format_calculator(&-half_theta),
                format_arg(op.control(), "Qubit"),
                format_arg(op.target(), "Qubit"),
            ))
        }
        Operation::MolmerSorensenXX(op) => Ok(format!(
            "  call void @rxx(double {}, double {}, %Qubit* {}, %Qubit* {})",
            format_calculator(&CalculatorFloat::ZERO),
            format_calculator(&CalculatorFloat::ZERO),
            format_arg(op.control(), "Qubit"),
            format_arg(op.target(), "Qubit"),
        )),
        Operation::PhaseShiftState1(op) => Ok(format!(
            "  call void @__quantum__qis__rz__body(double {}, %Qubit* {})",
            format_calculator(op.theta()),
            format_arg(op.qubit(), "Qubit")
        )),
        Operation::ControlledPauliY(op) => Ok(format!(
            "  call void @cy(%Qubit* {}, %Qubit* {})",
            format_arg(op.control(), "Qubit"),
            format_arg(op.target(), "Qubit")
        )),
        Operation::ControlledPhaseShift(op) => {
            let (minus_half_theta, half_theta) =
                match CalculatorFloat::from(format_calculator(op.theta())) {
                    CalculatorFloat::Float(theta) => (
                        CalculatorFloat::from(-0.5 * theta),
                        CalculatorFloat::from(0.5 * theta),
                    ),
                    CalculatorFloat::Str(s) => {
                        return Err(RoqoqoBackendError::CalculatorError(
                            CalculatorError::VariableNotSet { name: s.to_owned() },
                        ))
                    }
                };
            Ok(format!(
                "  call void @cp(double {}, double {}, %Qubit* {}, %Qubit* {})",
                format_calculator(&half_theta),
                format_calculator(&minus_half_theta),
                format_arg(op.control(), "Qubit"),
                format_arg(op.target(), "Qubit"),
            ))
        }
        Operation::RotateXY(op) => {
            let minus_phi = match CalculatorFloat::from(format_calculator(op.phi())) {
                CalculatorFloat::Float(phi) => CalculatorFloat::from(-phi),
                CalculatorFloat::Str(s) => {
                    return Err(RoqoqoBackendError::CalculatorError(
                        CalculatorError::VariableNotSet { name: s.to_owned() },
                    ))
                }
            };
            Ok(format!(
                "  call void @rxy(double {}, double {}, double {}, %Qubit* {})",
                format_calculator(op.theta()),
                format_calculator(op.phi()),
                format_calculator(&minus_phi),
                format_arg(op.qubit(), "Qubit"),
            ))
        }
        Operation::ControlledControlledPauliZ(op) => Ok(format!(
            "  call void @ccz(%Qubit* {}, %Qubit* {}, %Qubit* {})",
            format_arg(op.control_0(), "Qubit"),
            format_arg(op.control_1(), "Qubit"),
            format_arg(op.target(), "Qubit"),
        )),
        Operation::ControlledControlledPhaseShift(op) => {
            let (minus_frac_theta_4, frac_theta_4) =
                match CalculatorFloat::from(format_calculator(op.theta())) {
                    CalculatorFloat::Float(theta) => (
                        CalculatorFloat::from(-0.25 * theta),
                        CalculatorFloat::from(0.25 * theta),
                    ),
                    CalculatorFloat::Str(s) => {
                        return Err(RoqoqoBackendError::CalculatorError(
                            CalculatorError::VariableNotSet { name: s.to_owned() },
                        ))
                    }
                };
            Ok(format!(
                "  call void @ccp(double {}, double {}, %Qubit* {}, %Qubit* {}, %Qubit* {})",
                format_calculator(&frac_theta_4),
                format_calculator(&minus_frac_theta_4),
                format_arg(op.control_0(), "Qubit"),
                format_arg(op.control_1(), "Qubit"),
                format_arg(op.target(), "Qubit"),
            ))
        }
        _ => {
            if NO_CALL_OPERATIONS.contains(&operation.hqslang()) {
                Ok("".to_owned())
            } else {
                Err(RoqoqoBackendError::OperationNotInBackend {
                    backend: "QirBackend",
                    hqslang: operation.hqslang(),
                })
            }
        }
    }
}

pub fn gate_declaration(operation: &Operation) -> Result<String, RoqoqoBackendError> {
    match operation {
        Operation::RotateX(_) => {
            Ok("declare void @__quantum__qis__rx__body(double, %Qubit*)".to_owned())
        }
        Operation::RotateY(_) => {
            Ok("declare void @__quantum__qis__ry__body(double, %Qubit*)".to_owned())
        }
        Operation::RotateZ(_) => {
            Ok("declare void @__quantum__qis__rz__body(double, %Qubit*)".to_owned())
        }
        Operation::PauliX(_) => Ok("declare void @__quantum__qis__x__body(%Qubit*)".to_owned()),
        Operation::PauliY(_) => Ok("declare void @__quantum__qis__y__body(%Qubit*)".to_owned()),
        Operation::PauliZ(_) => Ok("declare void @__quantum__qis__z__body(%Qubit*)".to_owned()),
        Operation::Hadamard(_) => Ok("declare void @__quantum__qis__h__body(%Qubit*)".to_owned()),
        Operation::SGate(_) => Ok("declare void @__quantum__qis__s__body(%Qubit*)".to_owned()),
        Operation::TGate(_) => Ok("declare void @__quantum__qis__t__body(%Qubit*)".to_owned()),
        Operation::CNOT(_) => {
            Ok("declare void @__quantum__qis__cnot__body(%Qubit*, %Qubit*)".to_owned())
        }
        Operation::Toffoli(_) => {
            Ok("declare void @__quantum__qis__ccx__body(%Qubit*, %Qubit*, %Qubit*)".to_owned())
        }
        Operation::ControlledPauliZ(_) => {
            Ok("declare void @__quantum__qis__cz__body(%Qubit*, %Qubit*)".to_owned())
        }
        Operation::MeasureQubit(_) => {
            Ok("declare void @__quantum__qis__mz__body(%Qubit*, %Result* writeonly) #1".to_owned())
        }
        Operation::GateDefinition(gate_definition) => {
            let mut definition_str = format!(
                "\ndefine void @{}({}{}{}) {}{{\nentry:\n",
                gate_definition.name(),
                gate_definition
                    .free_parameters()
                    .iter()
                    .map(|param| format!("double %{}", param))
                    .collect::<Vec<String>>()
                    .join(", "),
                if !gate_definition.free_parameters().is_empty() && !gate_definition.qubits().is_empty() { ", " } else { "" },
                gate_definition
                    .qubits()
                    .iter()
                    .map(|&qubit| format!("%Qubit* %qubit{}", qubit).to_owned())
                    .collect::<Vec<String>>()
                    .join(", "),
                if gate_definition
                    .circuit()
                    .iter()
                    .filter(|&op| matches!(op, Operation::MeasureQubit(_)))
                    .collect::<Vec<&Operation>>()
                    .is_empty() { "" } else { "#1 " }
            );
            for operation in gate_definition.circuit().iter() {
                definition_str.push_str(&call_operation(operation)?);
                definition_str.push('\n');
            }
            definition_str = definition_str.replace(
                "null",
                &format!(
                    "%qubit{}",
                    gate_definition.qubits().first().unwrap_or(&0_usize)
                ),
            );
            for (index, qubit) in gate_definition.qubits().iter().enumerate() {
                definition_str = definition_str.replace(
                    &format!("inttoptr (i64 {} to %Qubit*)", index),
                    &format!("%qubit{}", qubit),
                );
            }
            definition_str.push_str("  ret void\n}\n");
            Ok(definition_str)
        }
        Operation::MultiQubitZZ(op) => {
            if op.qubits().len() != 2 {
                Err(RoqoqoBackendError::GenericError {
                    msg: format!(
                        "MultiQubitZZ is only supported for 2 qubits, there are {}.",
                        op.qubits().len()
                    ),
                })
            } else {
                Ok("declare void @__quantum__qis__rzz__body(double, %Qubit*, %Qubit*)".to_owned())
            }
        }
        Operation::ControlledPauliY(_) => {
            Ok("\ndefine void @cy(%Qubit* qubit0, %Qubit* qubit1) {\nentry:\n  call void @__quantum__qis__s__adj(%Qubit* qubit1)\n  call void @__quantum__qis__cnot__body(%Qubit* qubit0, %Qubit* qubit1)\n  call void @__quantum__qis__s__body(%Qubit* qubit1)\n  ret void\n}\n".to_owned())
        }
        _ => if NO_DECLARATION_OPERATIONS
            .contains(&operation.hqslang()) { Ok("".to_owned()) } else { Err(RoqoqoBackendError::OperationNotInBackend {
                backend: "QirBackend",
                hqslang: operation.hqslang(),
            }) },
    }
}
