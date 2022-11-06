fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("resources/icon.ico");
    res.compile().expect("Cannot compile resource bundle.");

    // Manually tell linker to include the resources in a binary, this is a workaround for:
    // https://github.com/mxre/winres/issues/32.
    println!("cargo:rustc-link-arg-bins=resource.lib");
    println!("cargo:rerun-if-changed=resources");
}