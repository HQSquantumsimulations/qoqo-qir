# Copyright © 2023 HQS Quantum Simulations GmbH. All Rights Reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
# in compliance with the License. You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software distributed under the License
# is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express
# or implied. See the License for the specific language governing permissions and limitations under
# the License.

import pytest
import sys
from qoqo_qir import QirBackend
from qoqo import Circuit
from qoqo import operations as ops # type: ignore


def test_qir() -> None:
    circuit = Circuit()
    circuit += ops.Hadamard(0)

    backend = QirBackend(None, "0.1")
    qir = backend.circuit_to_qir_str(circuit)
    assert qir


if __name__ == "__main__":
    pytest.main(sys.argv)
