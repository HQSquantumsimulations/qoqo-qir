
# roqoqo-qir

[![Crates.io](https://img.shields.io/crates/v/roqoqo-qir)](https://crates.io/crates/roqoqo-qir)
[![GitHub Workflow Status](https://github.com/HQSquantumsimulations/qoqo_qir/workflows/ci_tests/badge.svg)](https://github.com/HQSquantumsimulations/qoqo_qir/actions)
[![docs.rs](https://img.shields.io/docsrs/roqoqo-qir)](https://docs.rs/roqoqo-qir/)
![Crates.io](https://img.shields.io/crates/l/roqoqo-qir)

QIR interface for the roqoqo quantum toolkit by [HQS Quantum Simulations](https://quantumsimulations.de).

roqoqo-qir provides the QirBackend class that allows users translate a roqoqo circuit into a QIR file.
Not all roqoqo operations have a corresponding QIR expression.  
Circuits containing operations without a corresponding expression cannot be translated.

If you intend to use the produced QIR expression with the QIR alliance's QIR-runner you should use the measure_all argument.

## General Notes

This software is still in the beta stage. Functions and documentation are not yet complete and breaking changes can occur.

## Contributing

We welcome contributions to the project. If you want to contribute code, please have a look at CONTRIBUTE.md for our code contribution guidelines.
