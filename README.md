# Python language bindings for the C++ library preCICE

This package provides rust language bindings for the C++ library [preCICE](https://github.com/precice/precice). Note that the first two digits of the version number of the bindings indicate the preCICE version that the bindings support. The last digit represents the version of the bindings. Example: `v2.5.0` and `v2.5.2` of the bindings represent versions `0` and `2` of the bindings that are compatible with preCICE `v2.5.x`.

## User documentation

Please refer to [the preCICE documentation](https://www.precice.org/installation-bindings-rust.html) for information on how to install and use the rust bindings.

## Required dependencies

**preCICE**: Refer to [the preCICE documentation](https://precice.org/installation-overview.html) for information on building and installation.

**pkg-config**: A working installation of pkg-config, which is able to find preCICE

**cargo**: A working installation of cargo

## Installing the package from the registry

```
$ cargo add precice@2.5
```

## Installing the package from the git repository

```
$ cargo add --git https://github.com/precice/rust-bindings.git --rev v2.5.0 precice
```

## Usage

```rust
// create a solver interface
let mut interface = precice::new("SolverOne", "config.xml", 0, 1);

// call const members
let dims = interface.get_dimenstions();

// non-const members require the object to be pinned in memory
let dims = interface.pin_mut().initialize();

// action constants are directly part of precice
precice::action_write_iteration_checkpoint()
```

## Contributors

* [Frédéric Simonis](https://github.com/fsimonis)
