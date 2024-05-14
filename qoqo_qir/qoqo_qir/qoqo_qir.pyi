# Copyright © 2019-2024 HQS Quantum Simulations GmbH. All Rights Reserved.
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

from qoqo import Circuit
from typing import Optional

class QirBackend:
    """
    Backend to qoqo that produces QIR output.
    """
    def __init__(self, QirProfile: Optional[str], QirVersion: Optional[str]) -> None:
        """Create a new QirBackend

        Args:
            QirProfile (QirProfile): Qir profile to use.
            QirVersion (QirVersion): Qir version to use.
        """
    
    def circuit_to_qir_str(self, circuit: Circuit, measure_all: bool = False) -> str:
        """Translates a Circuit to a valid QIR string.

        Args:
            circuit (Circuit): The Circuit items that is translated
            measure_all (bool): Wether or not to measure all qubits at the end.

        Returns:
            str: The QIR string
        """

    def circuit_to_qir_file(
        self,
        circuit: Circuit,
        folder_name: str = ".",
        filename: str = "qir_output.ll",
        overwrite: bool = True,
        measure_all: bool = False,
    ):
        """Translates a Circuit to a valid QIR file.

        Args:
            circuit (Circuit): The Circuit items that is translated
            folder_name (str): The folder to save the QIR file in.
            filename (str): The name of the QIR file.
            overwrite (bool): Wether or not to overwrite an existing file.
            measure_all (bool): Wether or not to measure all qubits at the end.
        """
