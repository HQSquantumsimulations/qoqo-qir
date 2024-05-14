# qoqo-qir

[![Documentation Status](https://img.shields.io/badge/docs-read-blue)](https://hqsquantumsimulations.github.io/qoqo_qir/)
[![GitHub Workflow Status](https://github.com/HQSquantumsimulations/qoqo_qir/workflows/ci_tests/badge.svg)](https://github.com/HQSquantumsimulations/qoqo_qir/actions)
[![PyPI](https://img.shields.io/pypi/v/qoqo_qir)](https://pypi.org/project/qoqo_qir/)
[![PyPI - Format](https://img.shields.io/pypi/format/qoqo_qir)](https://pypi.org/project/qoqo_qir/)
![Crates.io](https://img.shields.io/crates/l/qoqo-qir)

QIR interface for the qoqo quantum toolkit by [HQS Quantum Simulations](https://quantumsimulations.de).

qoqo-qir provides the QirBackend class that allows users translate a qoqo circuit into a QIR file.
Not all qoqo operations have a corresponding QIR expression.  
Circuits containing operations without a corresponding expression can not be translated.

A source distribution now exists but requires a Rust install with a rust version > 1.47 and a maturin version { >= 0.14, <0.15 } in order to be built.

## General Notes

This software is still in the beta stage. Functions and documentation are not yet complete and breaking changes can occur.

## Contributing

We welcome contributions to the project. If you want to contribute code, please have a look at CONTRIBUTE.md for our code contribution guidelines.
