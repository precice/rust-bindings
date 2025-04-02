# Changelog of rust language bindings for preCICE

All notable changes to this project will be documented in this file.


## latest

## 3.2.0

- Added support for `precice::Error` to most functions, now returning `Result<>`
- Added bindings for all new methods of preCICE version 3.2.0

## 3.0.1

- Fix compatibility with the latest rust 1.73 which ships with Ubuntu 22.04 LTS

## 3.0.0

- Added rust wrapper to hide required pinning for mutable operations
- Added bindings for all methods of preCICE version 3.0.0

## 2.5.0

- First release of the rust bindings
- Added bindings for all non-deprecated methods of preCICE version 2.5.0
