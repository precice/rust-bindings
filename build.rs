extern crate pkg_config;
extern crate semver;
use semver::Version;

fn main() {
    // Rebuild when the bridge files change
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/precice-bridge.cpp");
    println!("cargo:rerun-if-changed=src/precice-bridge.hpp");

    // Get version from the precice crate
    let version = Version::parse(env!("CARGO_PKG_VERSION")).expect("Unable to parse crate version");
    let (major, minor) = (version.major, version.minor);
    let lower: &str = &format!("{major}.{minor}");
    let upper: &str = &format!("{major}.{}", minor + 1);

    // Get version from the precice crate
    let precice = pkg_config::Config::new()
        .range_version(lower..upper)
        .probe("libprecice")
        .expect("Couldn't find a suitable preCICE installation with pkg-config");

    cxx_build::bridge("src/lib.rs")
        .file("src/precice-bridge.cpp")
        .flag("--std=c++17")
        .flag_if_supported("-Wno-deprecated-declarations")
        .includes(precice.include_paths)
        .compile("precice-bridge")
}
