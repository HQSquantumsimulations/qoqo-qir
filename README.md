<img src="qoqo_Logo_vertical_color.png" alt="qoqo logo" width="300" />

# qoqo-qir

QIR interface for the qoqo/roqoqo quantum toolkit by [HQS Quantum Simulations](https://quantumsimulations.de).

This repository contains two components:

* The qoqo_qir backend for the qoqo python interface to roqoqo
* The roqoqo_qir backend for roqoqo directly

## qoqo-qir

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

## roqoqo-qir

[![Crates.io](https://img.shields.io/crates/v/roqoqo-qir)](https://crates.io/crates/roqoqo-qir)
[![GitHub Workflow Status](https://github.com/HQSquantumsimulations/qoqo_qir/workflows/ci_tests/badge.svg)](https://github.com/HQSquantumsimulations/qoqo_qir/actions)
[![docs.rs](https://img.shields.io/docsrs/roqoqo-qir)](https://docs.rs/roqoqo-qir/)
![Crates.io](https://img.shields.io/crates/l/roqoqo-qir)

QIR interface for the roqoqo quantum toolkit by [HQS Quantum Simulations](https://quantumsimulations.de).

roqoqo-qir provides the QirBackend class that allows users translate a roqoqo circuit into a QIR file.
Not all roqoqo operations have a corresponding QIR expression.  
Circuits containing operations without a corresponding expression can not be translated.

## General Notes

This software is still in the beta stage. Functions and documentation are not yet complete and breaking changes can occur.

## Contributing

We welcome contributions to the project. If you want to contribute code, please have a look at CONTRIBUTE.md for our code contribution guidelines.
