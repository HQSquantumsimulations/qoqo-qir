// Copyright © 2021-2023 HQS Quantum Simulations GmbH. All Rights Reserved.
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
//! Testing the roqoqo-qir Backend
//! run with `RUST_TEST_THREADS=1 cargo test`

use std::vec;

use qoqo_calculator::CalculatorFloat;
use roqoqo::{operations::*, Circuit};
use roqoqo_qir::{Backend, NUMBER_LABEL, NUMBER_VARS};
use serial_test::serial;

#[test]
#[serial]
fn test_simple_circuit() {
    let backend = Backend::new(None, None).unwrap();
    let mut circuit = Circuit::new();
    circuit.add_operation(PauliX::new(0));
    let qir_str = backend.circuit_to_qir_str(&circuit, false).unwrap();
    assert_eq!(qir_str, "%Qubit = type opaque\n\ndefine void @main() #0 {\nentry:\n  call void @__quantum__qis__x__body(%Qubit* inttoptr (i64 0 to %Qubit*))\n  ret void\n}\n\ndeclare void @__quantum__qis__x__body(%Qubit*)\n\nattributes #0 = { \"entry_point\" \"required_num_qubits\"=\"1\" \"required_num_results\"=\"0\" \"output_labeling_schema\" \"qir_profiles\"=\"base_profile\" }\n\n!llvm.module.flags = !{!0, !1, !2, !3}\n\n!0 = !{i32 1, !\"qir_major_version\", i32 1}\n!1 = !{i32 7, !\"qir_minor_version\", i32 0}\n!2 = !{i32 1, !\"dynamic_qubit_management\", i1 false}\n!3 = !{i32 1, !\"dynamic_result_management\", i1 false}");
}

#[test]
#[serial]
fn test_circuit_with_measure() {
    let backend = Backend::new(None, None).unwrap();
    let mut circuit = Circuit::new();
    circuit.add_operation(PauliX::new(0));
    circuit.add_operation(MeasureQubit::new(0, "ro".to_owned(), 0));
    let qir_str = backend.circuit_to_qir_str(&circuit, false).unwrap();
    assert_eq!(qir_str, "%Qubit = type opaque\n%Result = type opaque\n\ndefine void @main() #0 {\nentry:\n  call void @__quantum__qis__x__body(%Qubit* inttoptr (i64 0 to %Qubit*))\n  call void @__quantum__qis__mz__body(%Qubit* inttoptr (i64 0 to %Qubit*), %Result* inttoptr (i64 0 to %Result*)) #1\n  ret void\n}\n\ndeclare void @__quantum__qis__x__body(%Qubit*)\ndeclare void @__quantum__qis__mz__body(%Qubit*, %Result* writeonly) #1\n\nattributes #0 = { \"entry_point\" \"required_num_qubits\"=\"1\" \"required_num_results\"=\"1\" \"output_labeling_schema\" \"qir_profiles\"=\"base_profile\" \"irreversible\" }\nattributes #1 = { \"irreversible\" }\n\n!llvm.module.flags = !{!0, !1, !2, !3}\n\n!0 = !{i32 1, !\"qir_major_version\", i32 1}\n!1 = !{i32 7, !\"qir_minor_version\", i32 0}\n!2 = !{i32 1, !\"dynamic_qubit_management\", i1 false}\n!3 = !{i32 1, !\"dynamic_result_management\", i1 false}");
}

#[test]
#[serial]
fn test_qir_example_circuit() {
    let backend = Backend::new(None, None).unwrap();
    let mut circuit = Circuit::new();
    circuit.add_operation(Hadamard::new(0));
    circuit.add_operation(CNOT::new(0, 1));
    circuit.add_operation(MeasureQubit::new(0, "ro".to_owned(), 0));
    circuit.add_operation(MeasureQubit::new(1, "ro".to_owned(), 1));
    let qir_str = backend.circuit_to_qir_str(&circuit, false).unwrap();
    assert_eq!(qir_str, "%Qubit = type opaque\n%Result = type opaque\n\ndefine void @main() #0 {\nentry:\n  call void @__quantum__qis__h__body(%Qubit* inttoptr (i64 0 to %Qubit*))\n  call void @__quantum__qis__cnot__body(%Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  call void @__quantum__qis__mz__body(%Qubit* inttoptr (i64 0 to %Qubit*), %Result* inttoptr (i64 0 to %Result*)) #1\n  call void @__quantum__qis__mz__body(%Qubit* inttoptr (i64 1 to %Qubit*), %Result* inttoptr (i64 1 to %Result*)) #1\n  ret void\n}\n\ndeclare void @__quantum__qis__h__body(%Qubit*)\ndeclare void @__quantum__qis__cnot__body(%Qubit*, %Qubit*)\ndeclare void @__quantum__qis__mz__body(%Qubit*, %Result* writeonly) #1\n\nattributes #0 = { \"entry_point\" \"required_num_qubits\"=\"2\" \"required_num_results\"=\"2\" \"output_labeling_schema\" \"qir_profiles\"=\"base_profile\" \"irreversible\" }\nattributes #1 = { \"irreversible\" }\n\n!llvm.module.flags = !{!0, !1, !2, !3}\n\n!0 = !{i32 1, !\"qir_major_version\", i32 1}\n!1 = !{i32 7, !\"qir_minor_version\", i32 0}\n!2 = !{i32 1, !\"dynamic_qubit_management\", i1 false}\n!3 = !{i32 1, !\"dynamic_result_management\", i1 false}");
}

#[test]
#[serial]
fn test_qir_xy_circuit() {
    let backend = Backend::new(None, None).unwrap();
    let mut circuit = Circuit::new();
    circuit.add_operation(XY::new(0, 1, CalculatorFloat::FRAC_1_SQRT_2));
    circuit.add_operation(XY::new(2, 1, CalculatorFloat::PI));
    let qir_str = backend.circuit_to_qir_str(&circuit, false).unwrap();
    assert_eq!(qir_str, "%Qubit = type opaque\n\ndefine void @main() #0 {\nentry:\n  call void @xy(double -0.3535533905932738, %Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  call void @xy(double -1.5707963267948966, %Qubit* inttoptr (i64 2 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  ret void\n}\n\ndeclare void @__quantum__qis__rx__body(double, %Qubit*)\ndeclare void @__quantum__qis__cnot__body(%Qubit*, %Qubit*)\n\ndefine void @xy(double %theta, %Qubit* %qubit0, %Qubit* %qubit1) {\nentry:\n  call void @__quantum__qis__rx__body(double 1.5707963267948966, %Qubit* %qubit0)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__rx__body(double %theta, %Qubit* %qubit1)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__rx__body(double -1.5707963267948966, %Qubit* %qubit0)\n  ret void\n}\n\nattributes #0 = { \"entry_point\" \"required_num_qubits\"=\"3\" \"required_num_results\"=\"0\" \"output_labeling_schema\" \"qir_profiles\"=\"base_profile\" }\n\n!llvm.module.flags = !{!0, !1, !2, !3}\n\n!0 = !{i32 1, !\"qir_major_version\", i32 1}\n!1 = !{i32 7, !\"qir_minor_version\", i32 0}\n!2 = !{i32 1, !\"dynamic_qubit_management\", i1 false}\n!3 = !{i32 1, !\"dynamic_result_management\", i1 false}");
}

#[test]
#[serial]
fn test_qir_swap_circuit() {
    let backend = Backend::new(None, None).unwrap();
    let mut circuit = Circuit::new();
    circuit.add_operation(SWAP::new(0, 1));
    circuit.add_operation(SWAP::new(2, 1));
    let qir_str = backend.circuit_to_qir_str(&circuit, false).unwrap();
    assert_eq!(qir_str, "%Qubit = type opaque\n\ndefine void @main() #0 {\nentry:\n  call void @swap(%Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  call void @swap(%Qubit* inttoptr (i64 2 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  ret void\n}\n\ndeclare void @__quantum__qis__cnot__body(%Qubit*, %Qubit*)\n\ndefine void @swap(%Qubit* %qubit0, %Qubit* %qubit1) {\nentry:\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit1, %Qubit* %qubit0)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  ret void\n}\n\nattributes #0 = { \"entry_point\" \"required_num_qubits\"=\"3\" \"required_num_results\"=\"0\" \"output_labeling_schema\" \"qir_profiles\"=\"base_profile\" }\n\n!llvm.module.flags = !{!0, !1, !2, !3}\n\n!0 = !{i32 1, !\"qir_major_version\", i32 1}\n!1 = !{i32 7, !\"qir_minor_version\", i32 0}\n!2 = !{i32 1, !\"dynamic_qubit_management\", i1 false}\n!3 = !{i32 1, !\"dynamic_result_management\", i1 false}");
}

#[test]
#[serial]
fn test_qir_iswap_circuit() {
    let backend = Backend::new(None, None).unwrap();
    let mut circuit = Circuit::new();
    circuit.add_operation(ISwap::new(0, 1));
    circuit.add_operation(ISwap::new(2, 1));
    let qir_str = backend.circuit_to_qir_str(&circuit, false).unwrap();
    assert_eq!(qir_str, "%Qubit = type opaque\n\ndefine void @main() #0 {\nentry:\n  call void @iswap(%Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  call void @iswap(%Qubit* inttoptr (i64 2 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  ret void\n}\n\ndeclare void @__quantum__qis__rx__body(double, %Qubit*)\ndeclare void @__quantum__qis__cnot__body(%Qubit*, %Qubit*)\ndeclare void @__quantum__qis__ry__body(double, %Qubit*)\n\ndefine void @iswap(%Qubit* %qubit0, %Qubit* %qubit1) {\nentry:\n  call void @__quantum__qis__rx__body(double 1.5707963267948966, %Qubit* %qubit0)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__rx__body(double -1.5707963267948966, %Qubit* %qubit0)\n  call void @__quantum__qis__ry__body(double -1.5707963267948966, %Qubit* %qubit1)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__rx__body(double -1.5707963267948966, %Qubit* %qubit0)\n  ret void\n}\n\nattributes #0 = { \"entry_point\" \"required_num_qubits\"=\"3\" \"required_num_results\"=\"0\" \"output_labeling_schema\" \"qir_profiles\"=\"base_profile\" }\n\n!llvm.module.flags = !{!0, !1, !2, !3}\n\n!0 = !{i32 1, !\"qir_major_version\", i32 1}\n!1 = !{i32 7, !\"qir_minor_version\", i32 0}\n!2 = !{i32 1, !\"dynamic_qubit_management\", i1 false}\n!3 = !{i32 1, !\"dynamic_result_management\", i1 false}");
}

#[test]
#[serial]
fn test_qir_sqrtiswap_circuit() {
    let backend = Backend::new(None, None).unwrap();
    let mut circuit = Circuit::new();
    circuit.add_operation(SqrtISwap::new(0, 1));
    circuit.add_operation(SqrtISwap::new(2, 1));
    let qir_str = backend.circuit_to_qir_str(&circuit, false).unwrap();
    assert_eq!(qir_str, "%Qubit = type opaque\n\ndefine void @main() #0 {\nentry:\n  call void @siswap(%Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  call void @siswap(%Qubit* inttoptr (i64 2 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  ret void\n}\n\ndeclare void @__quantum__qis__rx__body(double, %Qubit*)\ndeclare void @__quantum__qis__cnot__body(%Qubit*, %Qubit*)\ndeclare void @__quantum__qis__ry__body(double, %Qubit*)\n\ndefine void @siswap(%Qubit* %qubit0, %Qubit* %qubit1) {\nentry:\n  call void @__quantum__qis__rx__body(double 1.5707963267948966, %Qubit* %qubit0)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__rx__body(double -0.7853981633974483, %Qubit* %qubit0)\n  call void @__quantum__qis__ry__body(double -0.7853981633974483, %Qubit* %qubit1)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__rx__body(double -1.5707963267948966, %Qubit* %qubit0)\n  ret void\n}\n\nattributes #0 = { \"entry_point\" \"required_num_qubits\"=\"3\" \"required_num_results\"=\"0\" \"output_labeling_schema\" \"qir_profiles\"=\"base_profile\" }\n\n!llvm.module.flags = !{!0, !1, !2, !3}\n\n!0 = !{i32 1, !\"qir_major_version\", i32 1}\n!1 = !{i32 7, !\"qir_minor_version\", i32 0}\n!2 = !{i32 1, !\"dynamic_qubit_management\", i1 false}\n!3 = !{i32 1, !\"dynamic_result_management\", i1 false}");
}

#[test]
#[serial]
fn test_qir_inv_sqrtiswap_circuit() {
    let backend = Backend::new(None, None).unwrap();
    let mut circuit = Circuit::new();
    circuit.add_operation(InvSqrtISwap::new(0, 1));
    circuit.add_operation(InvSqrtISwap::new(2, 1));
    let qir_str = backend.circuit_to_qir_str(&circuit, false).unwrap();
    assert_eq!(qir_str, "%Qubit = type opaque\n\ndefine void @main() #0 {\nentry:\n  call void @siswap_adj(%Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  call void @siswap_adj(%Qubit* inttoptr (i64 2 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  ret void\n}\n\ndeclare void @__quantum__qis__rx__body(double, %Qubit*)\ndeclare void @__quantum__qis__cnot__body(%Qubit*, %Qubit*)\ndeclare void @__quantum__qis__ry__body(double, %Qubit*)\n\ndefine void @siswap_adj(%Qubit* %qubit0, %Qubit* %qubit1) {\nentry:\n  call void @__quantum__qis__rx__body(double 1.5707963267948966, %Qubit* %qubit0)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__rx__body(double 0.7853981633974483, %Qubit* %qubit0)\n  call void @__quantum__qis__ry__body(double 0.7853981633974483, %Qubit* %qubit1)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__rx__body(double -1.5707963267948966, %Qubit* %qubit0)\n  ret void\n}\n\nattributes #0 = { \"entry_point\" \"required_num_qubits\"=\"3\" \"required_num_results\"=\"0\" \"output_labeling_schema\" \"qir_profiles\"=\"base_profile\" }\n\n!llvm.module.flags = !{!0, !1, !2, !3}\n\n!0 = !{i32 1, !\"qir_major_version\", i32 1}\n!1 = !{i32 7, !\"qir_minor_version\", i32 0}\n!2 = !{i32 1, !\"dynamic_qubit_management\", i1 false}\n!3 = !{i32 1, !\"dynamic_result_management\", i1 false}");
}

#[test]
#[serial]
fn test_qir_fswap_circuit() {
    let backend = Backend::new(None, None).unwrap();
    let mut circuit = Circuit::new();
    circuit.add_operation(FSwap::new(0, 1));
    circuit.add_operation(FSwap::new(2, 1));
    let qir_str = backend.circuit_to_qir_str(&circuit, false).unwrap();
    assert_eq!(qir_str, "%Qubit = type opaque\n\ndefine void @main() #0 {\nentry:\n  call void @fswap(%Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  call void @fswap(%Qubit* inttoptr (i64 2 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  ret void\n}\n\ndeclare void @__quantum__qis__rz__body(double, %Qubit*)\ndeclare void @__quantum__qis__rx__body(double, %Qubit*)\ndeclare void @__quantum__qis__cnot__body(%Qubit*, %Qubit*)\ndeclare void @__quantum__qis__ry__body(double, %Qubit*)\n\ndefine void @fswap(%Qubit* %qubit0, %Qubit* %qubit1) {\nentry:\n  call void @__quantum__qis__rz__body(double -1.5707963267948966, %Qubit* %qubit0)\n  call void @__quantum__qis__rz__body(double -1.5707963267948966, %Qubit* %qubit1)\n  call void @__quantum__qis__rx__body(double 1.5707963267948966, %Qubit* %qubit0)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__rx__body(double -1.5707963267948966, %Qubit* %qubit0)\n  call void @__quantum__qis__ry__body(double -1.5707963267948966, %Qubit* %qubit1)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__rx__body(double -1.5707963267948966, %Qubit* %qubit0)\n  ret void\n}\n\nattributes #0 = { \"entry_point\" \"required_num_qubits\"=\"3\" \"required_num_results\"=\"0\" \"output_labeling_schema\" \"qir_profiles\"=\"base_profile\" }\n\n!llvm.module.flags = !{!0, !1, !2, !3}\n\n!0 = !{i32 1, !\"qir_major_version\", i32 1}\n!1 = !{i32 7, !\"qir_minor_version\", i32 0}\n!2 = !{i32 1, !\"dynamic_qubit_management\", i1 false}\n!3 = !{i32 1, !\"dynamic_result_management\", i1 false}");
}

#[test]
#[serial]
fn test_qir_pm_interaction_circuit() {
    let backend = Backend::new(None, None).unwrap();
    let mut circuit = Circuit::new();
    circuit.add_operation(PMInteraction::new(0, 1, CalculatorFloat::PI));
    circuit.add_operation(PMInteraction::new(2, 1, CalculatorFloat::PI));
    let qir_str = backend.circuit_to_qir_str(&circuit, false).unwrap();
    assert_eq!(qir_str, "%Qubit = type opaque\n\ndefine void @main() #0 {\nentry:\n  call void @pmint(double 3.141592653589793, %Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  call void @pmint(double 3.141592653589793, %Qubit* inttoptr (i64 2 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  ret void\n}\n\ndeclare void @__quantum__qis__rx__body(double, %Qubit*)\ndeclare void @__quantum__qis__cnot__body(%Qubit*, %Qubit*)\ndeclare void @__quantum__qis__ry__body(double, %Qubit*)\n\ndefine void @pmint(double %theta, %Qubit* %qubit0, %Qubit* %qubit1) {\nentry:\n  call void @__quantum__qis__rx__body(double 1.5707963267948966, %Qubit* %qubit0)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__rx__body(double %theta, %Qubit* %qubit0)\n  call void @__quantum__qis__ry__body(double %theta, %Qubit* %qubit1)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__rx__body(double -1.5707963267948966, %Qubit* %qubit0)\n  ret void\n}\n\nattributes #0 = { \"entry_point\" \"required_num_qubits\"=\"3\" \"required_num_results\"=\"0\" \"output_labeling_schema\" \"qir_profiles\"=\"base_profile\" }\n\n!llvm.module.flags = !{!0, !1, !2, !3}\n\n!0 = !{i32 1, !\"qir_major_version\", i32 1}\n!1 = !{i32 7, !\"qir_minor_version\", i32 0}\n!2 = !{i32 1, !\"dynamic_qubit_management\", i1 false}\n!3 = !{i32 1, !\"dynamic_result_management\", i1 false}");
}

#[test]
#[serial]
fn test_qir_given_rotation_circuit() {
    let backend = Backend::new(None, None).unwrap();
    let mut circuit = Circuit::new();
    circuit.add_operation(GivensRotation::new(
        0,
        1,
        CalculatorFloat::PI,
        CalculatorFloat::ZERO,
    ));
    circuit.add_operation(GivensRotation::new(
        2,
        1,
        CalculatorFloat::PI,
        CalculatorFloat::ZERO,
    ));
    let qir_str = backend.circuit_to_qir_str(&circuit, false).unwrap();
    assert_eq!(qir_str, "%Qubit = type opaque\n\ndefine void @main() #0 {\nentry:\n  call void @gvnsrot(double -3.141592653589793, double 1.5707963267948966, %Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  call void @gvnsrot(double -3.141592653589793, double 1.5707963267948966, %Qubit* inttoptr (i64 2 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  ret void\n}\n\ndeclare void @__quantum__qis__rz__body(double, %Qubit*)\ndeclare void @__quantum__qis__rx__body(double, %Qubit*)\ndeclare void @__quantum__qis__cnot__body(%Qubit*, %Qubit*)\ndeclare void @__quantum__qis__ry__body(double, %Qubit*)\n\ndefine void @gvnsrot(double %minus_theta, double %phi_pi_over_2, %Qubit* %qubit0, %Qubit* %qubit1) {\nentry:\n  call void @__quantum__qis__rz__body(double %phi_pi_over_2, %Qubit* %qubit1)\n  call void @__quantum__qis__rx__body(double 1.5707963267948966, %Qubit* %qubit0)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__rx__body(double %minus_theta, %Qubit* %qubit0)\n  call void @__quantum__qis__ry__body(double %minus_theta, %Qubit* %qubit1)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__rx__body(double -1.5707963267948966, %Qubit* %qubit0)\n  call void @__quantum__qis__rz__body(double -1.5707963267948966, %Qubit* %qubit1)\n  ret void\n}\n\nattributes #0 = { \"entry_point\" \"required_num_qubits\"=\"3\" \"required_num_results\"=\"0\" \"output_labeling_schema\" \"qir_profiles\"=\"base_profile\" }\n\n!llvm.module.flags = !{!0, !1, !2, !3}\n\n!0 = !{i32 1, !\"qir_major_version\", i32 1}\n!1 = !{i32 7, !\"qir_minor_version\", i32 0}\n!2 = !{i32 1, !\"dynamic_qubit_management\", i1 false}\n!3 = !{i32 1, !\"dynamic_result_management\", i1 false}");
}

#[test]
#[serial]
fn test_qir_given_rotation_little_endian_circuit() {
    let backend = Backend::new(None, None).unwrap();
    let mut circuit = Circuit::new();
    circuit.add_operation(GivensRotationLittleEndian::new(
        0,
        1,
        CalculatorFloat::PI,
        CalculatorFloat::ZERO,
    ));
    circuit.add_operation(GivensRotationLittleEndian::new(
        2,
        1,
        CalculatorFloat::PI,
        CalculatorFloat::ZERO,
    ));
    let qir_str = backend.circuit_to_qir_str(&circuit, false).unwrap();
    assert_eq!(qir_str, "%Qubit = type opaque\n\ndefine void @main() #0 {\nentry:\n  call void @gvnsrotle(double -3.141592653589793, double 1.5707963267948966, %Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  call void @gvnsrotle(double -3.141592653589793, double 1.5707963267948966, %Qubit* inttoptr (i64 2 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  ret void\n}\n\ndeclare void @__quantum__qis__rz__body(double, %Qubit*)\ndeclare void @__quantum__qis__rx__body(double, %Qubit*)\ndeclare void @__quantum__qis__cnot__body(%Qubit*, %Qubit*)\ndeclare void @__quantum__qis__ry__body(double, %Qubit*)\n\ndefine void @gvnsrotle(double %minus_theta, double %phi_pi_over_2, %Qubit* %qubit0, %Qubit* %qubit1) {\nentry:\n  call void @__quantum__qis__rz__body(double -1.5707963267948966, %Qubit* %qubit0)\n  call void @__quantum__qis__rx__body(double 1.5707963267948966, %Qubit* %qubit0)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__rx__body(double %minus_theta, %Qubit* %qubit0)\n  call void @__quantum__qis__ry__body(double %minus_theta, %Qubit* %qubit1)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__rx__body(double -1.5707963267948966, %Qubit* %qubit0)\n  call void @__quantum__qis__rz__body(double %phi_pi_over_2, %Qubit* %qubit0)\n  ret void\n}\n\nattributes #0 = { \"entry_point\" \"required_num_qubits\"=\"3\" \"required_num_results\"=\"0\" \"output_labeling_schema\" \"qir_profiles\"=\"base_profile\" }\n\n!llvm.module.flags = !{!0, !1, !2, !3}\n\n!0 = !{i32 1, !\"qir_major_version\", i32 1}\n!1 = !{i32 7, !\"qir_minor_version\", i32 0}\n!2 = !{i32 1, !\"dynamic_qubit_management\", i1 false}\n!3 = !{i32 1, !\"dynamic_result_management\", i1 false}");
}

#[test]
#[serial]
fn test_qir_phase_shift_cz_circuit() {
    let backend = Backend::new(None, None).unwrap();
    let mut circuit = Circuit::new();
    circuit.add_operation(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::PI));
    circuit.add_operation(PhaseShiftedControlledZ::new(2, 1, CalculatorFloat::PI));
    let qir_str = backend.circuit_to_qir_str(&circuit, false).unwrap();
    assert_eq!(qir_str, "%Qubit = type opaque\n\ndefine void @main() #0 {\nentry:\n  call void @pscz(double 3.141592653589793, %Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  call void @pscz(double 3.141592653589793, %Qubit* inttoptr (i64 2 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  ret void\n}\n\ndeclare void @__quantum__qis__rz__body(double, %Qubit*)\ndeclare void @__quantum__qis__ry__body(double, %Qubit*)\ndeclare void @__quantum__qis__cnot__body(%Qubit*, %Qubit*)\ndeclare void @__quantum__qis__rx__body(double, %Qubit*)\n\ndefine void @pscz(double %phi, %Qubit* %qubit0, %Qubit* %qubit1) {\nentry:\n  call void @__quantum__qis__rz__body(double 1.5707963267948966, %Qubit* %qubit0)\n  call void @__quantum__qis__rz__body(double 1.5707963267948966, %Qubit* %qubit1)\n  call void @__quantum__qis__ry__body(double 1.5707963267948966, %Qubit* %qubit1)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__rx__body(double -1.5707963267948966, %Qubit* %qubit1)\n  call void @__quantum__qis__rz__body(double -1.5707963267948966, %Qubit* %qubit0)\n  call void @__quantum__qis__ry__body(double -1.5707963267948966, %Qubit* %qubit1)\n  call void @__quantum__qis__rz__body(double %phi, %Qubit* %qubit0)\n  call void @__quantum__qis__rz__body(double %phi, %Qubit* %qubit1)\n  ret void\n}\n\nattributes #0 = { \"entry_point\" \"required_num_qubits\"=\"3\" \"required_num_results\"=\"0\" \"output_labeling_schema\" \"qir_profiles\"=\"base_profile\" }\n\n!llvm.module.flags = !{!0, !1, !2, !3}\n\n!0 = !{i32 1, !\"qir_major_version\", i32 1}\n!1 = !{i32 7, !\"qir_minor_version\", i32 0}\n!2 = !{i32 1, !\"dynamic_qubit_management\", i1 false}\n!3 = !{i32 1, !\"dynamic_result_management\", i1 false}");
}

#[test]
#[serial]
fn test_qir_phase_shift_ctrl_phase_circuit() {
    let backend = Backend::new(None, None).unwrap();
    let mut circuit = Circuit::new();
    circuit.add_operation(PhaseShiftedControlledPhase::new(
        0,
        1,
        CalculatorFloat::PI,
        CalculatorFloat::SQRT_2,
    ));
    circuit.add_operation(PhaseShiftedControlledPhase::new(
        2,
        1,
        CalculatorFloat::PI,
        CalculatorFloat::SQRT_2,
    ));
    let qir_str = backend.circuit_to_qir_str(&circuit, false).unwrap();
    assert_eq!(qir_str, "%Qubit = type opaque\n\ndefine void @main() #0 {\nentry:\n  call void @pscp(double 1.5707963267948966, double -1.5707963267948966, double 1.4142135623730951, %Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  call void @pscp(double 1.5707963267948966, double -1.5707963267948966, double 1.4142135623730951, %Qubit* inttoptr (i64 2 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  ret void\n}\n\ndeclare void @__quantum__qis__rz__body(double, %Qubit*)\ndeclare void @__quantum__qis__cnot__body(%Qubit*, %Qubit*)\n\ndefine void @pscp(double %half_theta, double %minus_half_theta, double %phi, %Qubit* %qubit0, %Qubit* %qubit1) {\nentry:\n  call void @__quantum__qis__rz__body(double %half_theta, %Qubit* %qubit0)\n  call void @__quantum__qis__rz__body(double %half_theta, %Qubit* %qubit1)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__rz__body(double %minus_half_theta, %Qubit* %qubit1)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__rz__body(double %phi, %Qubit* %qubit0)\n  call void @__quantum__qis__rz__body(double %phi, %Qubit* %qubit1)\n  ret void\n}\n\nattributes #0 = { \"entry_point\" \"required_num_qubits\"=\"3\" \"required_num_results\"=\"0\" \"output_labeling_schema\" \"qir_profiles\"=\"base_profile\" }\n\n!llvm.module.flags = !{!0, !1, !2, !3}\n\n!0 = !{i32 1, !\"qir_major_version\", i32 1}\n!1 = !{i32 7, !\"qir_minor_version\", i32 0}\n!2 = !{i32 1, !\"dynamic_qubit_management\", i1 false}\n!3 = !{i32 1, !\"dynamic_result_management\", i1 false}");
}

#[test]
#[serial]
fn test_qir_rxx_circuit() {
    let backend = Backend::new(None, None).unwrap();
    let mut circuit = Circuit::new();
    circuit.add_operation(MolmerSorensenXX::new(0, 1));
    circuit.add_operation(VariableMSXX::new(2, 1, CalculatorFloat::PI));
    let qir_str = backend.circuit_to_qir_str(&circuit, false).unwrap();
    assert_eq!(qir_str, "%Qubit = type opaque\n\ndefine void @main() #0 {\nentry:\n  call void @rxx(double 0.0, double 0.0, %Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  call void @rxx(double 1.5707963267948966, double -1.5707963267948966, %Qubit* inttoptr (i64 2 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  ret void\n}\n\ndeclare void @__quantum__qis__rx__body(double, %Qubit*)\ndeclare void @__quantum__qis__cnot__body(%Qubit*, %Qubit*)\n\ndefine void @rxx(double %half_theta, double %minus_half_theta, %Qubit* %qubit0, %Qubit* %qubit1) {\nentry:\n  call void @__quantum__qis__rx__body(double %half_theta, %Qubit* %qubit0)\n  call void @__quantum__qis__rx__body(double %half_theta, %Qubit* %qubit1)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__rx__body(double %minus_half_theta, %Qubit* %qubit0)\n  call void @__quantum__qis__rx__body(double %minus_half_theta, %Qubit* %qubit1)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  ret void\n}\n\nattributes #0 = { \"entry_point\" \"required_num_qubits\"=\"3\" \"required_num_results\"=\"0\" \"output_labeling_schema\" \"qir_profiles\"=\"base_profile\" }\n\n!llvm.module.flags = !{!0, !1, !2, !3}\n\n!0 = !{i32 1, !\"qir_major_version\", i32 1}\n!1 = !{i32 7, !\"qir_minor_version\", i32 0}\n!2 = !{i32 1, !\"dynamic_qubit_management\", i1 false}\n!3 = !{i32 1, !\"dynamic_result_management\", i1 false}");
}

#[test]
#[serial]
fn test_qir_cy_circuit() {
    let backend = Backend::new(None, None).unwrap();
    let mut circuit = Circuit::new();
    circuit.add_operation(ControlledPauliY::new(0, 1));
    circuit.add_operation(ControlledPauliY::new(2, 1));
    let qir_str = backend.circuit_to_qir_str(&circuit, false).unwrap();
    assert_eq!(qir_str, "%Qubit = type opaque\n\ndefine void @main() #0 {\nentry:\n  call void @cy(%Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  call void @cy(%Qubit* inttoptr (i64 2 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  ret void\n}\n\ndeclare void @__quantum__qis__s__adj(%Qubit*)\ndeclare void @__quantum__qis__cnot__body(%Qubit*, %Qubit*)\ndeclare void @__quantum__qis__s__body(%Qubit*)\n\ndefine void @cy(%Qubit* qubit0, %Qubit* qubit1) {\nentry:\n  call void @__quantum__qis__s__adj(%Qubit* qubit1)\n  call void @__quantum__qis__cnot__body(%Qubit* qubit0, %Qubit* qubit1)\n  call void @__quantum__qis__s__body(%Qubit* qubit1)\n  ret void\n}\n\nattributes #0 = { \"entry_point\" \"required_num_qubits\"=\"3\" \"required_num_results\"=\"0\" \"output_labeling_schema\" \"qir_profiles\"=\"base_profile\" }\n\n!llvm.module.flags = !{!0, !1, !2, !3}\n\n!0 = !{i32 1, !\"qir_major_version\", i32 1}\n!1 = !{i32 7, !\"qir_minor_version\", i32 0}\n!2 = !{i32 1, !\"dynamic_qubit_management\", i1 false}\n!3 = !{i32 1, !\"dynamic_result_management\", i1 false}");
}

#[test]
#[serial]
fn test_qir_controlled_phase_shift_circuit() {
    let backend = Backend::new(None, None).unwrap();
    let mut circuit = Circuit::new();
    circuit.add_operation(ControlledPhaseShift::new(
        0,
        1,
        CalculatorFloat::from("0.6"),
    ));
    circuit.add_operation(ControlledPhaseShift::new(
        2,
        1,
        CalculatorFloat::from("0.6"),
    ));
    let qir_str = backend.circuit_to_qir_str(&circuit, false).unwrap();
    assert_eq!(qir_str, "%Qubit = type opaque\n\ndefine void @main() #0 {\nentry:\n  call void @cp(double 0.3, double -0.3, %Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  call void @cp(double 0.3, double -0.3, %Qubit* inttoptr (i64 2 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  ret void\n}\n\ndeclare void @__quantum__qis__rz__body(double, %Qubit*)\ndeclare void @__quantum__qis__cnot__body(%Qubit*, %Qubit*)\n\ndefine void @cp(double %half_theta, double %minus_half_theta, %Qubit* %qubit0, %Qubit* %qubit1) {\nentry:\n  call void @__quantum__qis__rz__body(double %half_theta, %Qubit* %qubit0)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__rz__body(double %minus_half_theta, %Qubit* %qubit1)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__rz__body(double %half_theta, %Qubit* %qubit1)\n  ret void\n}\n\nattributes #0 = { \"entry_point\" \"required_num_qubits\"=\"3\" \"required_num_results\"=\"0\" \"output_labeling_schema\" \"qir_profiles\"=\"base_profile\" }\n\n!llvm.module.flags = !{!0, !1, !2, !3}\n\n!0 = !{i32 1, !\"qir_major_version\", i32 1}\n!1 = !{i32 7, !\"qir_minor_version\", i32 0}\n!2 = !{i32 1, !\"dynamic_qubit_management\", i1 false}\n!3 = !{i32 1, !\"dynamic_result_management\", i1 false}");
}

#[test]
#[serial]
fn test_qir_rotate_xy_circuit() {
    let backend = Backend::new(None, None).unwrap();
    let mut circuit = Circuit::new();
    circuit.add_operation(RotateXY::new(
        0,
        CalculatorFloat::FRAC_PI_4,
        CalculatorFloat::from("0.6"),
    ));
    circuit.add_operation(RotateXY::new(
        2,
        CalculatorFloat::FRAC_PI_4,
        CalculatorFloat::from("0.6"),
    ));
    let qir_str = backend.circuit_to_qir_str(&circuit, false).unwrap();
    assert_eq!(qir_str, "%Qubit = type opaque\n\ndefine void @main() #0 {\nentry:\n  call void @rxy(double 0.7853981633974483, double 0.6, double -0.6, %Qubit* inttoptr (i64 0 to %Qubit*))\n  call void @rxy(double 0.7853981633974483, double 0.6, double -0.6, %Qubit* inttoptr (i64 2 to %Qubit*))\n  ret void\n}\n\ndeclare void @__quantum__qis__rz__body(double, %Qubit*)\ndeclare void @__quantum__qis__rx__body(double, %Qubit*)\n\ndefine void @rxy(double %theta, double %phi, double %minus_phi, %Qubit* %qubit0) {\nentry:\n  call void @__quantum__qis__rz__body(double %minus_phi, %Qubit* %qubit0)\n  call void @__quantum__qis__rx__body(double %theta, %Qubit* %qubit0)\n  call void @__quantum__qis__rz__body(double %phi, %Qubit* %qubit0)\n  ret void\n}\n\nattributes #0 = { \"entry_point\" \"required_num_qubits\"=\"3\" \"required_num_results\"=\"0\" \"output_labeling_schema\" \"qir_profiles\"=\"base_profile\" }\n\n!llvm.module.flags = !{!0, !1, !2, !3}\n\n!0 = !{i32 1, !\"qir_major_version\", i32 1}\n!1 = !{i32 7, !\"qir_minor_version\", i32 0}\n!2 = !{i32 1, !\"dynamic_qubit_management\", i1 false}\n!3 = !{i32 1, !\"dynamic_result_management\", i1 false}");
}

#[test]
#[serial]
fn test_qir_rotate_ccz_circuit() {
    let backend = Backend::new(None, None).unwrap();
    let mut circuit = Circuit::new();
    circuit.add_operation(ControlledControlledPauliZ::new(0, 1, 2));
    circuit.add_operation(ControlledControlledPauliZ::new(2, 1, 0));
    let qir_str = backend.circuit_to_qir_str(&circuit, false).unwrap();
    assert_eq!(qir_str, "%Qubit = type opaque\n\ndefine void @main() #0 {\nentry:\n  call void @ccz(%Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*), %Qubit* inttoptr (i64 2 to %Qubit*))\n  call void @ccz(%Qubit* inttoptr (i64 2 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*), %Qubit* inttoptr (i64 0 to %Qubit*))\n  ret void\n}\n\ndeclare void @__quantum__qis__rz__body(double, %Qubit*)\ndeclare void @__quantum__qis__cnot__body(%Qubit*, %Qubit*)\n\ndefine void @ccz(%Qubit* %qubit0, %Qubit* %qubit1, %Qubit* %qubit2) {\nentry:\n  call void @__quantum__qis__rz__body(double 0.7853981633974483, %Qubit* %qubit1)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit1, %Qubit* %qubit2)\n  call void @__quantum__qis__rz__body(double -0.7853981633974483, %Qubit* %qubit2)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit1, %Qubit* %qubit2)\n  call void @__quantum__qis__rz__body(double 0.7853981633974483, %Qubit* %qubit2)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__rz__body(double -0.7853981633974483, %Qubit* %qubit1)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit1, %Qubit* %qubit2)\n  call void @__quantum__qis__rz__body(double 0.7853981633974483, %Qubit* %qubit2)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit1, %Qubit* %qubit2)\n  call void @__quantum__qis__rz__body(double -0.7853981633974483, %Qubit* %qubit2)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__rz__body(double 0.7853981633974483, %Qubit* %qubit0)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit2)\n  call void @__quantum__qis__rz__body(double -0.7853981633974483, %Qubit* %qubit2)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit2)\n  call void @__quantum__qis__rz__body(double 0.7853981633974483, %Qubit* %qubit2)\n  ret void\n}\n\nattributes #0 = { \"entry_point\" \"required_num_qubits\"=\"3\" \"required_num_results\"=\"0\" \"output_labeling_schema\" \"qir_profiles\"=\"base_profile\" }\n\n!llvm.module.flags = !{!0, !1, !2, !3}\n\n!0 = !{i32 1, !\"qir_major_version\", i32 1}\n!1 = !{i32 7, !\"qir_minor_version\", i32 0}\n!2 = !{i32 1, !\"dynamic_qubit_management\", i1 false}\n!3 = !{i32 1, !\"dynamic_result_management\", i1 false}");
}

#[test]
#[serial]
fn test_qir_rotate_ccp_circuit() {
    let backend = Backend::new(None, None).unwrap();
    let mut circuit = Circuit::new();
    circuit.add_operation(ControlledControlledPhaseShift::new(
        0,
        1,
        2,
        CalculatorFloat::from("1.8"),
    ));
    circuit.add_operation(ControlledControlledPhaseShift::new(
        2,
        1,
        0,
        CalculatorFloat::PI,
    ));
    let qir_str = backend.circuit_to_qir_str(&circuit, false).unwrap();
    assert_eq!(qir_str, "%Qubit = type opaque\n\ndefine void @main() #0 {\nentry:\n  call void @ccp(double 0.45, double -0.45, %Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*), %Qubit* inttoptr (i64 2 to %Qubit*))\n  call void @ccp(double 0.7853981633974483, double -0.7853981633974483, %Qubit* inttoptr (i64 2 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*), %Qubit* inttoptr (i64 0 to %Qubit*))\n  ret void\n}\n\ndeclare void @__quantum__qis__rz__body(double, %Qubit*)\ndeclare void @__quantum__qis__cnot__body(%Qubit*, %Qubit*)\n\ndefine void @ccp(double %frac_theta_4, double %minus_frac_theta_4, %Qubit* %qubit0, %Qubit* %qubit1, %Qubit* %qubit2) {\nentry:\n  call void @__quantum__qis__rz__body(double %frac_theta_4, %Qubit* %qubit1)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit1, %Qubit* %qubit2)\n  call void @__quantum__qis__rz__body(double %minus_frac_theta_4, %Qubit* %qubit2)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit1, %Qubit* %qubit2)\n  call void @__quantum__qis__rz__body(double %frac_theta_4, %Qubit* %qubit2)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__rz__body(double %minus_frac_theta_4, %Qubit* %qubit1)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit1, %Qubit* %qubit2)\n  call void @__quantum__qis__rz__body(double %frac_theta_4, %Qubit* %qubit2)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit1, %Qubit* %qubit2)\n  call void @__quantum__qis__rz__body(double %minus_frac_theta_4, %Qubit* %qubit2)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__rz__body(double %frac_theta_4, %Qubit* %qubit0)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit2)\n  call void @__quantum__qis__rz__body(double %minus_frac_theta_4, %Qubit* %qubit2)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit2)\n  call void @__quantum__qis__rz__body(double %frac_theta_4, %Qubit* %qubit2)\n  ret void\n}\n\nattributes #0 = { \"entry_point\" \"required_num_qubits\"=\"3\" \"required_num_results\"=\"0\" \"output_labeling_schema\" \"qir_profiles\"=\"base_profile\" }\n\n!llvm.module.flags = !{!0, !1, !2, !3}\n\n!0 = !{i32 1, !\"qir_major_version\", i32 1}\n!1 = !{i32 7, !\"qir_minor_version\", i32 0}\n!2 = !{i32 1, !\"dynamic_qubit_management\", i1 false}\n!3 = !{i32 1, !\"dynamic_result_management\", i1 false}");
}

#[test]
#[serial]
fn test_conditional_circuit() {
    *NUMBER_LABEL.lock().unwrap() = 0;
    *NUMBER_VARS.lock().unwrap() = 0;
    let backend = Backend::new(None, Some("0.1".to_string())).unwrap();

    let mut circuit_cond = Circuit::new();
    circuit_cond += PauliX::new(0);
    circuit_cond += Hadamard::new(0);
    circuit_cond += CNOT::new(0, 1);
    circuit_cond += RotateX::new(0, CalculatorFloat::FRAC_PI_2);
    circuit_cond += RotateX::new(1, CalculatorFloat::from("0.5"));
    let mut circuit = Circuit::new();

    let mut circuit_cond2 = Circuit::new();
    circuit_cond2 += CNOT::new(1, 2);

    circuit += DefinitionBit::new("ro".to_owned(), 2, true);
    circuit += Hadamard::new(0);
    circuit += MeasureQubit::new(0, "ro".to_string(), 0);
    circuit += PragmaConditional::new("ro".to_owned(), 0, circuit_cond);
    circuit += PauliY::new(1);
    circuit += MeasureQubit::new(1, "ro".to_string(), 1);
    circuit += PragmaConditional::new("ro".to_owned(), 1, circuit_cond2);
    let qir_str = { backend.circuit_to_qir_str(&circuit, false).unwrap() };
    assert_eq!(qir_str, "%Qubit = type opaque\n%Result = type opaque\n\ndefine void @main() #0 {\nentry:\n  call void @__quantum__qis__h__body(%Qubit* inttoptr (i64 0 to %Qubit*))\n  call void @__quantum__qis__mz__body(%Qubit* inttoptr (i64 0 to %Qubit*), %Result* inttoptr (i64 0 to %Result*)) #1\n  %0 = call i1 @__quantum__qis__read_result__body(%Result* inttoptr (i64 0 to %Result*))\n  br i1 %0, label %then0, label %continue0\n\nthen0:\n  call void @__quantum__qis__x__body(%Qubit* inttoptr (i64 0 to %Qubit*))\n  call void @__quantum__qis__h__body(%Qubit* inttoptr (i64 0 to %Qubit*))\n  call void @__quantum__qis__cnot__body(%Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  call void @__quantum__qis__rx__body(double 1.5707963267948966, %Qubit* inttoptr (i64 0 to %Qubit*))\n  call void @__quantum__qis__rx__body(double 0.5, %Qubit* inttoptr (i64 1 to %Qubit*))\n  br label %continue0\n\ncontinue0:\n  call void @__quantum__qis__y__body(%Qubit* inttoptr (i64 1 to %Qubit*))\n  call void @__quantum__qis__mz__body(%Qubit* inttoptr (i64 1 to %Qubit*), %Result* inttoptr (i64 1 to %Result*)) #1\n  %1 = call i1 @__quantum__qis__read_result__body(%Result* inttoptr (i64 1 to %Result*))\n  br i1 %1, label %then1, label %continue1\n\nthen1:\n  call void @__quantum__qis__cnot__body(%Qubit* inttoptr (i64 1 to %Qubit*), %Qubit* inttoptr (i64 2 to %Qubit*))\n  br label %continue1\n\ncontinue1:\n  ret void\n}\n\ndeclare void @__quantum__qis__h__body(%Qubit*)\ndeclare void @__quantum__qis__mz__body(%Qubit*, %Result* writeonly) #1\ndeclare i1 @__quantum__qis__read_result__body(%Result*)\ndeclare void @__quantum__qis__x__body(%Qubit*)\ndeclare void @__quantum__qis__cnot__body(%Qubit*, %Qubit*)\ndeclare void @__quantum__qis__rx__body(double, %Qubit*)\ndeclare void @__quantum__qis__y__body(%Qubit*)\n\nattributes #0 = { \"entry_point\" \"required_num_qubits\"=\"3\" \"required_num_results\"=\"2\" \"output_labeling_schema\" \"qir_profiles\"=\"base_profile\" \"irreversible\" }\nattributes #1 = { \"irreversible\" }\n\n!llvm.module.flags = !{!0, !1, !2, !3}\n\n!0 = !{i32 1, !\"qir_major_version\", i32 1}\n!1 = !{i32 7, !\"qir_minor_version\", i32 0}\n!2 = !{i32 1, !\"dynamic_qubit_management\", i1 false}\n!3 = !{i32 1, !\"dynamic_result_management\", i1 false}");
}

#[test]
#[serial]
fn test_loop_circuit() {
    *NUMBER_LABEL.lock().unwrap() = 0;
    *NUMBER_VARS.lock().unwrap() = 0;
    let backend = Backend::new(None, Some("0.1".to_string())).unwrap();

    let mut circuitloop = Circuit::new();
    circuitloop += PauliX::new(0);
    circuitloop += Hadamard::new(0);
    circuitloop += CNOT::new(0, 1);
    circuitloop += RotateX::new(0, CalculatorFloat::FRAC_PI_2);
    circuitloop += RotateX::new(1, CalculatorFloat::from("5"));
    let mut circuit = Circuit::new();

    let mut circuitloop2 = Circuit::new();
    circuitloop2 += CNOT::new(1, 2);

    circuit += Hadamard::new(0);
    circuit += PragmaLoop::new(CalculatorFloat::from("7"), circuitloop);
    circuit += PauliY::new(1);
    circuit += PragmaLoop::new(CalculatorFloat::from("3.2"), circuitloop2);
    let qir_str = backend.circuit_to_qir_str(&circuit, false).unwrap();
    assert_eq!(qir_str, "%Qubit = type opaque\n\ndefine void @main() #0 {\nentry:\n  call void @__quantum__qis__h__body(%Qubit* inttoptr (i64 0 to %Qubit*))\n  br label %header0\n\nheader0:\n  %0 = phi i64 [ 1, %entry ], [ %2, %loop0 ]\n  %1 = icmp slt i64 %0, 8\n  br i1 %1, label %loop0, label %continue0\n\nloop0:\n  call void @__quantum__qis__x__body(%Qubit* inttoptr (i64 0 to %Qubit*))\n  call void @__quantum__qis__h__body(%Qubit* inttoptr (i64 0 to %Qubit*))\n  call void @__quantum__qis__cnot__body(%Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))\n  call void @__quantum__qis__rx__body(double 1.5707963267948966, %Qubit* inttoptr (i64 0 to %Qubit*))\n  call void @__quantum__qis__rx__body(double 5.0, %Qubit* inttoptr (i64 1 to %Qubit*))\n  %2 = add i64 %0, 1\n  br label %header0\n\ncontinue0:\n  call void @__quantum__qis__y__body(%Qubit* inttoptr (i64 1 to %Qubit*))\n  br label %header1\n\nheader1:\n  %3 = phi i64 [ 1, %continue0 ], [ %5, %loop1 ]\n  %4 = icmp slt i64 %3, 4\n  br i1 %4, label %loop1, label %continue1\n\nloop1:\n  call void @__quantum__qis__cnot__body(%Qubit* inttoptr (i64 1 to %Qubit*), %Qubit* inttoptr (i64 2 to %Qubit*))\n  %5 = add i64 %3, 1\n  br label %header1\n\ncontinue1:\n  ret void\n}\n\ndeclare void @__quantum__qis__h__body(%Qubit*)\ndeclare void @__quantum__qis__x__body(%Qubit*)\ndeclare void @__quantum__qis__cnot__body(%Qubit*, %Qubit*)\ndeclare void @__quantum__qis__rx__body(double, %Qubit*)\ndeclare void @__quantum__qis__y__body(%Qubit*)\n\nattributes #0 = { \"entry_point\" \"required_num_qubits\"=\"3\" \"required_num_results\"=\"0\" \"output_labeling_schema\" \"qir_profiles\"=\"base_profile\" }\n\n!llvm.module.flags = !{!0, !1, !2, !3}\n\n!0 = !{i32 1, !\"qir_major_version\", i32 1}\n!1 = !{i32 7, !\"qir_minor_version\", i32 0}\n!2 = !{i32 1, !\"dynamic_qubit_management\", i1 false}\n!3 = !{i32 1, !\"dynamic_result_management\", i1 false}");
}
#[test]
#[serial]
fn test_gate_definition_circuit() {
    let backend = Backend::new(None, Some("0.1".to_string())).unwrap();

    let mut circuit_gate = Circuit::new();
    circuit_gate += PauliX::new(0);
    circuit_gate += Hadamard::new(0);
    circuit_gate += CNOT::new(0, 1);
    circuit_gate += RotateX::new(0, CalculatorFloat::from("theta"));
    circuit_gate += RotateX::new(1, CalculatorFloat::from("2.54"));
    let mut circuit = Circuit::new();

    circuit += PauliY::new(0);
    circuit += GateDefinition::new(
        [
            Operation::from(RotateX::new(0, CalculatorFloat::FRAC_PI_4)),
            Operation::from(RotateZ::new(1, CalculatorFloat::from("phi"))),
            Operation::from(MeasureQubit::new(1, "ro".to_owned(), 1)),
        ]
        .into_iter()
        .collect(),
        "rotate_measure".to_owned(),
        vec![1, 2],
        vec!["phi".to_owned()],
    );
    circuit += GateDefinition::new(
        circuit_gate,
        "rotate_bell".to_owned(),
        vec![0, 1],
        vec!["theta".to_owned()],
    );
    circuit += GateDefinition::new(
        Circuit::new(),
        "rotate_bell".to_owned(),
        vec![0, 1],
        vec!["theta".to_owned()],
    );
    circuit += PauliZ::new(1);
    circuit += CallDefinedGate::new(
        "rotate_bell".to_owned(),
        vec![1, 2],
        vec![CalculatorFloat::PI],
    );
    circuit += CallDefinedGate::new(
        "rotate_measure".to_owned(),
        vec![2, 0],
        vec![CalculatorFloat::from(0.1)],
    );
    circuit += MeasureQubit::new(0, "ro".to_string(), 0);

    let qir_str = backend.circuit_to_qir_str(&circuit, false).unwrap();
    assert_eq!(qir_str, "%Qubit = type opaque\n%Result = type opaque\n\ndefine void @main() #0 {\nentry:\n  call void @__quantum__qis__y__body(%Qubit* inttoptr (i64 0 to %Qubit*))\n  call void @__quantum__qis__z__body(%Qubit* inttoptr (i64 1 to %Qubit*))\n  call void @rotate_bell(double 3.141592653589793, %Qubit* inttoptr (i64 1 to %Qubit*), %Qubit* inttoptr (i64 2 to %Qubit*))\n  call void @rotate_measure(double 0.1, %Qubit* inttoptr (i64 2 to %Qubit*), %Qubit* inttoptr (i64 0 to %Qubit*))\n  call void @__quantum__qis__mz__body(%Qubit* inttoptr (i64 0 to %Qubit*), %Result* inttoptr (i64 0 to %Result*)) #1\n  ret void\n}\n\ndeclare void @__quantum__qis__rx__body(double, %Qubit*)\ndeclare void @__quantum__qis__rz__body(double, %Qubit*)\ndeclare void @__quantum__qis__mz__body(%Qubit*, %Result* writeonly) #1\n\ndefine void @rotate_measure(double %phi, %Qubit* %qubit1, %Qubit* %qubit2) #1 {\nentry:\n  call void @__quantum__qis__rx__body(double 0.7853981633974483, %Qubit* %qubit1)\n  call void @__quantum__qis__rz__body(double %phi, %Qubit* %qubit2)\n  call void @__quantum__qis__mz__body(%Qubit* %qubit2, %Result* inttoptr (i64 1 to %Result*)) #1\n  ret void\n}\n\ndeclare void @__quantum__qis__x__body(%Qubit*)\ndeclare void @__quantum__qis__h__body(%Qubit*)\ndeclare void @__quantum__qis__cnot__body(%Qubit*, %Qubit*)\n\ndefine void @rotate_bell(double %theta, %Qubit* %qubit0, %Qubit* %qubit1) {\nentry:\n  call void @__quantum__qis__x__body(%Qubit* %qubit0)\n  call void @__quantum__qis__h__body(%Qubit* %qubit0)\n  call void @__quantum__qis__cnot__body(%Qubit* %qubit0, %Qubit* %qubit1)\n  call void @__quantum__qis__rx__body(double %theta, %Qubit* %qubit0)\n  call void @__quantum__qis__rx__body(double 2.54, %Qubit* %qubit1)\n  ret void\n}\n\ndeclare void @__quantum__qis__y__body(%Qubit*)\ndeclare void @__quantum__qis__z__body(%Qubit*)\n\nattributes #0 = { \"entry_point\" \"required_num_qubits\"=\"3\" \"required_num_results\"=\"1\" \"output_labeling_schema\" \"qir_profiles\"=\"base_profile\" \"irreversible\" }\nattributes #1 = { \"irreversible\" }\n\n!llvm.module.flags = !{!0, !1, !2, !3}\n\n!0 = !{i32 1, !\"qir_major_version\", i32 1}\n!1 = !{i32 7, !\"qir_minor_version\", i32 0}\n!2 = !{i32 1, !\"dynamic_qubit_management\", i1 false}\n!3 = !{i32 1, !\"dynamic_result_management\", i1 false}");
}

/// Test Debug, Clone and PartialEq for Backend
#[test]
#[serial]
fn test_debug_clone_partialeq() {
    let backend = Backend::new(None, None).unwrap();

    // Test Debug trait
    assert_eq!(
        format!("{backend:?}"),
        "Backend { qir_profile: BaseProfile, qir_version: V0point1 }"
    );

    // Test Clone trait
    assert_eq!(backend.clone(), backend);

    // PartialEq
    let backend_0 = Backend::new(None, None).unwrap();

    assert!(backend_0 == backend);
    assert!(backend == backend_0);
}
