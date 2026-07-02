fn main() {
    // Set the platform-specific linker args for Python extension modules
    // (e.g. `-undefined dynamic_lookup` on macOS), so plain `cargo build`
    // works outside maturin.
    pyo3_build_config::add_extension_module_link_args();
}
