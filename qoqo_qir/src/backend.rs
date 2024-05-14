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

use std::path::Path;

use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    prelude::*,
};
use qoqo::convert_into_circuit;
use roqoqo_qir::Backend;

/// Backend to qoqo that produces QIR output which can be imported.
///
/// This backend takes a qoqo circuit to be run on a certain device and returns a QIR file
/// containing the translated circuit. The circuit itself is translated using the qoqo_qir
/// interface. In this backend, the initialization sets up the relevant parameters and the run
/// function calls the QIR interface and writes the QIR file, which is saved to be used by the
/// user on whatever platform they see fit. QIR input is widely supported on various quantum
/// computing platforms.
#[pyclass(name = "QirBackend", module = "qoqo_qir")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QirBackendWrapper {
    /// Internal storage of [roqoqo_qir::Backend]
    pub internal: Backend,
}

#[pymethods]
impl QirBackendWrapper {
    /// Creates new QIR backend.
    ///
    /// Args:
    ///     qubit_register_name (Optional[str]): The name of the qubit register.
    ///
    /// Returns:
    ///     Self: The new QirBackend intance.
    #[new]
    pub fn new(qir_profile: Option<String>, qir_version: Option<String>) -> PyResult<Self> {
        Ok(Self {
            internal: Backend::new(qir_profile, qir_version)
                .map_err(|x| PyValueError::new_err(format!("{x}")))?,
        })
    }

    /// Translates a Circuit to a QIR string.
    ///
    /// Args:
    ///     circuit: The Circuit items that is translated
    ///
    /// Returns:
    ///     str: The QIR string
    ///
    /// Raises:
    ///     TypeError: Circuit conversion error
    ///     ValueError: Operation not in QIR backend
    #[pyo3(signature = (circuit, measure_all=false))]
    pub fn circuit_to_qir_str(
        &self,
        circuit: &Bound<PyAny>,
        measure_all: bool,
    ) -> PyResult<String> {
        let circuit = convert_into_circuit(circuit).map_err(|x| {
            PyTypeError::new_err(format!("Cannot convert python object to Circuit: {x:?}"))
        })?;
        Backend::circuit_to_qir_str(&self.internal, &circuit, measure_all)
            .map_err(|x| PyValueError::new_err(format!("Error during QIR translation: {x:?}")))
    }

    /// Translates a Circuit to a QIR file.
    ///
    /// Args:
    ///     circuit: The Circuit that is translated
    ///     folder_name: The name of the folder that is prepended to all filenames.
    ///     filename: The name of the file the QIR text is saved to.
    ///     overwrite: Whether to overwrite file if it already exists.
    ///
    /// Returns:
    ///     Ok(()): The QIR file was correctly written
    ///
    /// Raises:
    ///     TypeError: Circuit conversion error
    ///     ValueError: Operation not in QIR backend
    #[pyo3(
        signature = (circuit, folder_name=".".to_owned(), filename="qir_output.ll".to_owned(), overwrite=true, measure_all=false)
    )]
    pub fn circuit_to_qir_file(
        &self,
        circuit: &Bound<PyAny>,
        folder_name: String,
        filename: String,
        overwrite: bool,
        measure_all: bool,
    ) -> PyResult<()> {
        let circuit = convert_into_circuit(circuit).map_err(|x| {
            PyTypeError::new_err(format!("Cannot convert python object to Circuit: {x:?}"))
        })?;
        let folder_name = Path::new(&folder_name);
        let filename = Path::new(&filename);
        Backend::circuit_to_qir_file(
            &self.internal,
            &circuit,
            folder_name,
            filename,
            overwrite,
            measure_all,
        )
        .map_err(|x| PyValueError::new_err(format!("Error during QIR translation: {x:?}")))
    }
}
