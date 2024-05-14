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
//
//! Testing the roqoqo-qir Interface
//! run with `RUST_TEST_THREADS=1 cargo test`

use qoqo_calculator::CalculatorFloat;
use roqoqo::{operations::*, Circuit};
use roqoqo_qir::{call_operation, gate_declaration, NUMBER_LABEL, NUMBER_VARS};
use std::f64::consts::PI;
use test_case::test_case;

/// Test that all operations return the correct gate declaration
#[test_case(Operation::from(PauliX::new(0)), "declare void @__quantum__qis__x__body(%Qubit*)"; "PauliX")]
#[test_case(Operation::from(PauliY::new(0)), "declare void @__quantum__qis__y__body(%Qubit*)"; "PauliY")]
#[test_case(Operation::from(PauliZ::new(0)), "declare void @__quantum__qis__z__body(%Qubit*)"; "PauliZ")]
#[test_case(Operation::from(Hadamard::new(0)), "declare void @__quantum__qis__h__body(%Qubit*)"; "Hadamard")]
#[test_case(Operation::from(SGate::new(0)), "declare void @__quantum__qis__s__body(%Qubit*)"; "SGate")]
#[test_case(Operation::from(TGate::new(0)), "declare void @__quantum__qis__t__body(%Qubit*)"; "TGate")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(-PI))), "declare void @__quantum__qis__rx__body(double, %Qubit*)"; "RotateX")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(-PI))), "declare void @__quantum__qis__ry__body(double, %Qubit*)"; "RotateY")]
#[test_case(Operation::from(RotateZ::new(0, CalculatorFloat::from(-PI))), "declare void @__quantum__qis__rz__body(double, %Qubit*)"; "RotateZ")]
#[test_case(Operation::from(CNOT::new(0, 1)), "declare void @__quantum__qis__cnot__body(%Qubit*, %Qubit*)"; "CNOT")]
#[test_case(Operation::from(ControlledPauliZ::new(0, 1)), "declare void @__quantum__qis__cz__body(%Qubit*, %Qubit*)"; "ControlledPauliZ")]
#[test_case(Operation::from(SWAP::new(0, 1)), ""; "SWAP")]
#[test_case(Operation::from(ISwap::new(0, 1)), ""; "ISwap")]
#[test_case(Operation::from(SqrtISwap::new(0, 1)), ""; "SqrtISwap")]
#[test_case(Operation::from(InvSqrtISwap::new(0, 1)), ""; "InvSqrtISwap")]
#[test_case(Operation::from(FSwap::new(0, 1)), ""; "FSwap")]
#[test_case(Operation::from(MeasureQubit::new(0,"ro".to_owned(), 0)), "declare void @__quantum__qis__mz__body(%Qubit*, %Result* writeonly) #1"; "MeasureQubit")]
#[test_case(Operation::from(Toffoli::new(0, 1, 2)), "declare void @__quantum__qis__ccx__body(%Qubit*, %Qubit*, %Qubit*)"; "Toffoli")]
#[test_case(Operation::from(GateDefinition::new(vec![Operation::from(RotateX::new(0, CalculatorFloat::from("theta"))), Operation::from(RotateX::new(1, CalculatorFloat::PI))].into_iter().collect(), "test_gate".to_owned(), vec![0, 1], vec!["theta".to_owned()])), "\ndefine void @test_gate(double %theta, %Qubit* %qubit0, %Qubit* %qubit1) {\nentry:\n  call void @__quantum__qis__rx__body(double %theta, %Qubit* %qubit0)\n  call void @__quantum__qis__rx__body(double 3.141592653589793, %Qubit* %qubit1)\n  ret void\n}\n"; "GateDefinition")]
#[test_case(Operation::from(CallDefinedGate::new("test".to_owned(), vec![0, 1], vec![CalculatorFloat::from("3.14")])), ""; "CallDefinedGate")]
#[test_case(Operation::from(PragmaConditional::new("q".to_owned(), 0, Circuit::new())), ""; "PragmaConditional")]
#[test_case(Operation::from(PragmaLoop::new(CalculatorFloat::Float(5.2), Circuit::new())), ""; "PragmaLoop")]
#[test_case(Operation::from(MultiQubitZZ::new(vec![0, 1], CalculatorFloat::from(-PI))), "declare void @__quantum__qis__rzz__body(double, %Qubit*, %Qubit*)"; "MultiqubitZZ")]
#[test_case(Operation::from(XY::new(0, 1, CalculatorFloat::PI)), ""; "XY")]
#[test_case(Operation::from(InvSqrtPauliX::new(0)), ""; "InvSqrtPauliX")]
#[test_case(Operation::from(Identity::new(0)), ""; "Identity")]
#[test_case(Operation::from(PMInteraction::new(0, 1, CalculatorFloat::from(0.069))), ""; "PMInteraction")]
#[test_case(Operation::from(GivensRotation::new(0, 1, CalculatorFloat::from("5"), CalculatorFloat::FRAC_1_SQRT_2)), ""; "GivensRotation")]
#[test_case(Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::from("5"), CalculatorFloat::FRAC_1_SQRT_2)), ""; "GivensRotationLittleEndian")]
#[test_case(Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::FRAC_PI_4)), ""; "PhaseShiftedControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)), ""; "PhaseShiftedControlledPhase")]
#[test_case(Operation::from(PhaseShiftState1::new(4, CalculatorFloat::FRAC_PI_4)), ""; "PhaseShiftState1")]
#[test_case(Operation::from(MolmerSorensenXX::new(0, 1)), ""; "MolmerSorensenXX")]
#[test_case(Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::FRAC_PI_4)), ""; "VariableMSXX")]
#[test_case(Operation::from(ControlledPauliY::new(0, 1)), "\ndefine void @cy(%Qubit* qubit0, %Qubit* qubit1) {\nentry:\n  call void @__quantum__qis__s__adj(%Qubit* qubit1)\n  call void @__quantum__qis__cnot__body(%Qubit* qubit0, %Qubit* qubit1)\n  call void @__quantum__qis__s__body(%Qubit* qubit1)\n  ret void\n}\n"; "ControlledPauliY")]
#[test_case(Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::FRAC_PI_2)), ""; "ControlledPhaseShift")]
#[test_case(Operation::from(RotateXY::new(0, CalculatorFloat::from("1"), CalculatorFloat::FRAC_PI_2)), ""; "RotateXY")]
#[test_case(Operation::from(ControlledControlledPauliZ::new(0, 1, 2)), ""; "ControlledControlledPauliZ")]
#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::PI)), ""; "ControlledControlledPhaseShift")]
#[serial_test::serial]
fn test_gate_definition(operation: Operation, converted: &str) {
    *NUMBER_LABEL.lock().unwrap() = 0;
    assert_eq!(gate_declaration(&operation).unwrap(), converted.to_string())
}

/// Test that all operations return the correct gate call
#[test_case(Operation::from(PauliX::new(3)), "  call void @__quantum__qis__x__body(%Qubit* inttoptr (i64 3 to %Qubit*))"; "PauliX")]
#[test_case(Operation::from(PauliY::new(0)), "  call void @__quantum__qis__y__body(%Qubit* inttoptr (i64 0 to %Qubit*))"; "PauliY")]
#[test_case(Operation::from(PauliZ::new(1)), "  call void @__quantum__qis__z__body(%Qubit* inttoptr (i64 1 to %Qubit*))"; "PauliZ")]
#[test_case(Operation::from(Hadamard::new(0)), "  call void @__quantum__qis__h__body(%Qubit* inttoptr (i64 0 to %Qubit*))"; "Hadamard")]
#[test_case(Operation::from(SGate::new(0)), "  call void @__quantum__qis__s__body(%Qubit* inttoptr (i64 0 to %Qubit*))"; "SGate")]
#[test_case(Operation::from(TGate::new(3)), "  call void @__quantum__qis__t__body(%Qubit* inttoptr (i64 3 to %Qubit*))"; "TGate")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(PI))), "  call void @__quantum__qis__rx__body(double 3.141592653589793, %Qubit* inttoptr (i64 0 to %Qubit*))"; "RotateX")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(-PI))), "  call void @__quantum__qis__ry__body(double -3.141592653589793, %Qubit* inttoptr (i64 0 to %Qubit*))"; "RotateY")]
#[test_case(Operation::from(RotateZ::new(1, CalculatorFloat::from(-PI))), "  call void @__quantum__qis__rz__body(double -3.141592653589793, %Qubit* inttoptr (i64 1 to %Qubit*))"; "RotateZ")]
#[test_case(Operation::from(CNOT::new(0, 1)), "  call void @__quantum__qis__cnot__body(%Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))"; "CNOT")]
#[test_case(Operation::from(SWAP::new(2, 1)), "  call void @swap(%Qubit* inttoptr (i64 2 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))"; "SWAP")]
#[test_case(Operation::from(ISwap::new(2, 1)), "  call void @iswap(%Qubit* inttoptr (i64 2 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))"; "ISwap")]
#[test_case(Operation::from(SqrtISwap::new(2, 1)), "  call void @siswap(%Qubit* inttoptr (i64 2 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))"; "SqrtISwap")]
#[test_case(Operation::from(InvSqrtISwap::new(2, 1)), "  call void @siswap_adj(%Qubit* inttoptr (i64 2 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))"; "InvSqrtISwap")]
#[test_case(Operation::from(FSwap::new(2, 1)), "  call void @fswap(%Qubit* inttoptr (i64 2 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))"; "FSwap")]
#[test_case(Operation::from(MeasureQubit::new(1,"ro".to_owned(), 1)), "  call void @__quantum__qis__mz__body(%Qubit* inttoptr (i64 1 to %Qubit*), %Result* inttoptr (i64 1 to %Result*)) #1"; "MeasureQubit")]
#[test_case(Operation::from(Toffoli::new(0, 1, 2)), "  call void @__quantum__qis__ccx__body(%Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*), %Qubit* inttoptr (i64 2 to %Qubit*))"; "Toffoli")]
#[test_case(Operation::from(GateDefinition::new(vec![Operation::from(RotateX::new(0, CalculatorFloat::from("theta"))), Operation::from(RotateX::new(1, CalculatorFloat::PI))].into_iter().collect(), "test_gate".to_owned(), vec![0, 1], vec!["theta".to_owned()])), ""; "GateDefinition")]
#[test_case(Operation::from(CallDefinedGate::new("test".to_owned(), vec![0, 1], vec![CalculatorFloat::from("3.14")])), "  call void @test(double 3.14, %Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))"; "CallDefinedGate")]
#[test_case(Operation::from(PragmaConditional::new("q".to_owned(), 1, vec![Operation::from(RotateX::new(0, CalculatorFloat::from("0.5"))), Operation::from(RotateX::new(1, CalculatorFloat::PI))].into_iter().collect())), "  %0 = call i1 @__quantum__qis__read_result__body(%Result* inttoptr (i64 1 to %Result*))\n  br i1 %0, label %then0, label %continue0\n\nthen0:\n  call void @__quantum__qis__rx__body(double 0.5, %Qubit* inttoptr (i64 0 to %Qubit*))\n  call void @__quantum__qis__rx__body(double 3.141592653589793, %Qubit* inttoptr (i64 1 to %Qubit*))\n  br label %continue0\n\ncontinue0:"; "PragmaConditional")]
#[test_case(Operation::from(PragmaLoop::new(CalculatorFloat::Float(5.2), vec![Operation::from(Hadamard::new(0))].into_iter().collect())), "  br label %header0\n\nheader0:\n  %0 = phi i64 [ 1, %entry ], [ %2, %loop0 ]\n  %1 = icmp slt i64 %0, 6\n  br i1 %1, label %loop0, label %continue0\n\nloop0:\n  call void @__quantum__qis__h__body(%Qubit* inttoptr (i64 0 to %Qubit*))\n  %2 = add i64 %0, 1\n  br label %header0\n\ncontinue0:"; "PragmaLoop")]
#[test_case(Operation::from(MultiQubitZZ::new(vec![0, 1], CalculatorFloat::FRAC_PI_2)), "  call void @__quantum__qis__rzz__body(double 1.5707963267948966, %Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))"; "MultiqubitZZ")]
#[test_case(Operation::from(XY::new(0, 1, CalculatorFloat::PI)), "  call void @xy(double -1.5707963267948966, %Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))"; "XY")]
#[test_case(Operation::from(SqrtPauliX::new(0)), "  call void @__quantum__qis__rx__body(double 1.5707963267948966, %Qubit* inttoptr (i64 0 to %Qubit*))"; "SqrtPauliX")]
#[test_case(Operation::from(InvSqrtPauliX::new(0)), "  call void @__quantum__qis__rx__body(double -1.5707963267948966, %Qubit* inttoptr (i64 0 to %Qubit*))"; "InvSqrtPauliX")]
#[test_case(Operation::from(Identity::new(0)), ""; "Identity")]
#[test_case(Operation::from(PMInteraction::new(0, 1, CalculatorFloat::from(0.069))), "  call void @pmint(double 0.069, %Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))"; "PMInteraction")]
#[test_case(Operation::from(GivensRotation::new(0, 1, CalculatorFloat::from("5"), CalculatorFloat::FRAC_1_SQRT_2)), "  call void @gvnsrot(double -5.0, double 2.277903107981444, %Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))"; "GivensRotation")]
#[test_case(Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::from("5"), CalculatorFloat::FRAC_1_SQRT_2)), "  call void @gvnsrotle(double -5.0, double 2.277903107981444, %Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))"; "GivensRotationLittleEndian")]
#[test_case(Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::FRAC_PI_4)), "  call void @pscz(double 0.7853981633974483, %Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))"; "PhaseShiftedControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::PI, CalculatorFloat::FRAC_PI_4)), "  call void @pscp(double 1.5707963267948966, double -1.5707963267948966, double 0.7853981633974483, %Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))"; "PhaseShiftedControlledPhase")]
#[test_case(Operation::from(PhaseShiftState1::new(4, CalculatorFloat::FRAC_PI_4)), "  call void @__quantum__qis__rz__body(double 0.7853981633974483, %Qubit* inttoptr (i64 4 to %Qubit*))"; "PhaseShiftState1")]
#[test_case(Operation::from(MolmerSorensenXX::new(0, 1)), "  call void @rxx(double 0.0, double 0.0, %Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))"; "MolmerSorensenXX")]
#[test_case(Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::FRAC_PI_2)), "  call void @rxx(double 0.7853981633974483, double -0.7853981633974483, %Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))"; "VariableMSXX")]
#[test_case(Operation::from(ControlledPauliY::new(0, 1)), "  call void @cy(%Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))"; "ControlledPauliY")]
#[test_case(Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::FRAC_PI_2)), "  call void @cp(double 0.7853981633974483, double -0.7853981633974483, %Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*))"; "ControlledPhaseShift")]
#[test_case(Operation::from(RotateXY::new(0, CalculatorFloat::from("1"), CalculatorFloat::FRAC_PI_2)), "  call void @rxy(double 1.0, double 1.5707963267948966, double -1.5707963267948966, %Qubit* inttoptr (i64 0 to %Qubit*))"; "RotateXY")]
#[test_case(Operation::from(ControlledControlledPauliZ::new(0, 1, 2)), "  call void @ccz(%Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*), %Qubit* inttoptr (i64 2 to %Qubit*))"; "ControlledControlledPauliZ")]
#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, CalculatorFloat::PI)), "  call void @ccp(double 0.7853981633974483, double -0.7853981633974483, %Qubit* inttoptr (i64 0 to %Qubit*), %Qubit* inttoptr (i64 1 to %Qubit*), %Qubit* inttoptr (i64 2 to %Qubit*))"; "ControlledControlledPhaseShift")]
#[serial_test::serial]
fn test_gate_call(operation: Operation, converted: &str) {
    *NUMBER_LABEL.lock().unwrap() = 0;
    *NUMBER_VARS.lock().unwrap() = 0;
    assert_eq!(call_operation(&operation).unwrap(), converted.to_string())
}
