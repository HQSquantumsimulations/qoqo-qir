// Copyright © 2022-2024 HQS Quantum Simulations GmbH. All Rights Reserved.
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
//
//! Testing the qoqo-qir Backend

use std::{fs, path::Path};

use pyo3::{types::PyAnyMethods, Bound, Py, Python};
use qoqo::{operations::convert_operation_to_pyobject, CircuitWrapper};
use qoqo_calculator::CalculatorFloat;
use qoqo_calculator_pyo3::CalculatorFloatWrapper;
use qoqo_qir::QirBackendWrapper;
use roqoqo::{operations::*, Circuit};

// helper functions
fn circuitpy_from_circuitru(py: Python, circuit: Circuit) -> Bound<CircuitWrapper> {
    let circuit_type = py.get_type_bound::<CircuitWrapper>();
    let binding = circuit_type.call0().unwrap();
    let circuitpy = binding.downcast::<CircuitWrapper>().unwrap();
    for op in circuit {
        let new_op = convert_operation_to_pyobject(op).unwrap();
        circuitpy.call_method1("add", (new_op.clone(),)).unwrap();
    }
    circuitpy.to_owned()
}

fn new_qirbackend(
    py: Python,
    qir_profile: Option<String>,
    qir_version: Option<String>,
) -> Bound<QirBackendWrapper> {
    let backend_type = py.get_type_bound::<QirBackendWrapper>();
    backend_type
        .call1((qir_profile, qir_version))
        .unwrap()
        .downcast::<QirBackendWrapper>()
        .unwrap()
        .to_owned()
}

#[test]
fn test_backend_error() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backend_type = py.get_type_bound::<QirBackendWrapper>();
        assert!(backend_type.call1(("error", "0.1")).is_err());
        assert!(backend_type.call1(("base_profile", "error")).is_err());
    })
}

#[test]
fn test_simple_circuit() {
    let mut circuit = Circuit::new();
    circuit.add_operation(PauliX::new(0));

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backendpy = new_qirbackend(py, None, None);
        let circuitpy = circuitpy_from_circuitru(py, circuit);

        let result: String = backendpy
            .call_method1("circuit_to_qir_str", (circuitpy,))
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(result, "%Qubit = type opaque\n\ndefine void @main() #0 {\nentry:\n  call void @__quantum__qis__x__body(%Qubit* inttoptr (i64 0 to %Qubit*))\n  ret void\n}\n\ndeclare void @__quantum__qis__x__body(%Qubit*)\n\nattributes #0 = { \"entry_point\" \"required_num_qubits\"=\"1\" \"required_num_results\"=\"0\" \"output_labeling_schema\" \"qir_profiles\"=\"base_profile\" }\n\n!llvm.module.flags = !{!0, !1, !2, !3}\n\n!0 = !{i32 1, !\"qir_major_version\", i32 1}\n!1 = !{i32 7, !\"qir_minor_version\", i32 0}\n!2 = !{i32 1, !\"dynamic_qubit_management\", i1 false}\n!3 = !{i32 1, !\"dynamic_result_management\", i1 false}");
    });
}

#[test]
fn test_to_str_errors() {
    let mut circuit = Circuit::new();
    circuit.add_operation(PauliX::new(0));
    circuit.add_operation(QuantumRabi::new(0, 0, CalculatorFloat::ZERO));

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backendpy = new_qirbackend(py, None, None);
        let circuitpy = circuitpy_from_circuitru(py, circuit);
        let calc = Py::new(
            py,
            CalculatorFloatWrapper {
                internal: CalculatorFloat::from("0.0"),
            },
        )
        .unwrap();

        assert!(backendpy
            .call_method1("circuit_to_qir_str", (calc,))
            .is_err());
        assert!(backendpy
            .call_method1("circuit_to_qir_str", (circuitpy,))
            .is_err());
    });
}

#[test]
fn test_to_file_errors() {
    let mut circuit = Circuit::new();
    circuit.add_operation(PauliX::new(0));
    circuit.add_operation(QuantumRabi::new(0, 0, CalculatorFloat::ZERO));

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backendpy = new_qirbackend(py, None, None);
        let circuitpy = circuitpy_from_circuitru(py, circuit);
        let calc = Py::new(
            py,
            CalculatorFloatWrapper {
                internal: CalculatorFloat::from("0.0"),
            },
        )
        .unwrap();

        assert!(backendpy
            .call_method1("circuit_to_qir_file", (calc,))
            .is_err());
        assert!(backendpy
            .call_method1("circuit_to_qir_file", (circuitpy,))
            .is_err());
    });
}

#[test]
fn test_simple_circuit_file() {
    let mut circuit = Circuit::new();
    circuit.add_operation(PauliX::new(0));

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backendpy = new_qirbackend(py, None, None);
        let circuitpy = circuitpy_from_circuitru(py, circuit);

        let _result = backendpy
            .call_method1("circuit_to_qir_file", (circuitpy,))
            .unwrap();
        let read_in_path = Path::new("qir_output.ll");
        assert!(read_in_path.exists());
        fs::remove_file(read_in_path).unwrap();
    });
}

#[test]
fn test_circuit_with_measure() {
    let mut circuit = Circuit::new();
    circuit.add_operation(PauliX::new(0));
    circuit.add_operation(MeasureQubit::new(0, "ro".to_owned(), 0));
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backendpy = new_qirbackend(py, None, None);
        let circuitpy = circuitpy_from_circuitru(py, circuit);

        let result: String = backendpy
            .call_method1("circuit_to_qir_str", (circuitpy,))
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(result, "%Qubit = type opaque\n%Result = type opaque\n\ndefine void @main() #0 {\nentry:\n  call void @__quantum__qis__x__body(%Qubit* inttoptr (i64 0 to %Qubit*))\n  call void @__quantum__qis__mz__body(%Qubit* inttoptr (i64 0 to %Qubit*), %Result* inttoptr (i64 0 to %Result*)) #1\n  ret void\n}\n\ndeclare void @__quantum__qis__x__body(%Qubit*)\ndeclare void @__quantum__qis__mz__body(%Qubit*, %Result* writeonly) #1\n\nattributes #0 = { \"entry_point\" \"required_num_qubits\"=\"1\" \"required_num_results\"=\"1\" \"output_labeling_schema\" \"qir_profiles\"=\"base_profile\" \"irreversible\" }\nattributes #1 = { \"irreversible\" }\n\n!llvm.module.flags = !{!0, !1, !2, !3}\n\n!0 = !{i32 1, !\"qir_major_version\", i32 1}\n!1 = !{i32 7, !\"qir_minor_version\", i32 0}\n!2 = !{i32 1, !\"dynamic_qubit_management\", i1 false}\n!3 = !{i32 1, !\"dynamic_result_management\", i1 false}");
    });
}

#[test]
fn test_qir_example_circuit() {
    let mut circuit = Circuit::new();
    circuit.add_operation(Hadamard::new(0));
    circuit.add_operation(CNOT::new(0, 1));
    circuit.add_operation(MeasureQubit::new(0, "ro".to_owned(), 0));
    circuit.add_operation(MeasureQubit::new(1, "ro".to_owned(), 1));

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backendpy = new_qirbackend(py, None, None);
        let circuitpy = circuitpy_from_circuitru(py, circuit);

        let result: String = backendpy
            .call_method1("circuit_to_qir_str", (circuitpy,))
            .unwrap()
            .extract()
            .unwrap();
        assert_eq!(result, "%Qubit = type opaque\n%Result = type opaque\n\ndefine void @main() #0 {\nentry:\n  call void @__quantum__qis__h__body(%Qubit* inttoptr (i64 0 to %Qubit*))\n  call void @__quantum__qis__cnot__body(%Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  call void @__quantum__qis__mz__body(%Qubit* inttoptr (i64 0 to %Qubit*), %Result* inttoptr (i64 0 to %Result*)) #1\n  call void @__quantum__qis__mz__body(%Qubit* inttoptr (i64 1 to %Qubit*), %Result* inttoptr (i64 1 to %Result*)) #1\n  ret void\n}\n\ndeclare void @__quantum__qis__h__body(%Qubit*)\ndeclare void @__quantum__qis__cnot__body(%Qubit*, %Qubit*)\ndeclare void @__quantum__qis__mz__body(%Qubit*, %Result* writeonly) #1\n\nattributes #0 = { \"entry_point\" \"required_num_qubits\"=\"2\" \"required_num_results\"=\"2\" \"output_labeling_schema\" \"qir_profiles\"=\"base_profile\" \"irreversible\" }\nattributes #1 = { \"irreversible\" }\n\n!llvm.module.flags = !{!0, !1, !2, !3}\n\n!0 = !{i32 1, !\"qir_major_version\", i32 1}\n!1 = !{i32 7, !\"qir_minor_version\", i32 0}\n!2 = !{i32 1, !\"dynamic_qubit_management\", i1 false}\n!3 = !{i32 1, !\"dynamic_result_management\", i1 false}");
    });
}
