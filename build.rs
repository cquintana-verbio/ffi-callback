use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=inc/");
    println!("cargo:rerun-if-changed=build.rs");

    compile();
    bindings();
}

fn compile() {
    cc::Build::new()
        .files(["inc/mylib.c"].iter())
        .include(Path::new("inc"))
        .opt_level(3)
        .extra_warnings(true)
        .compile("mylib");

    println!("cargo:rustc-link-lib=mylib");
}

fn bindings() {
    bindgen::Builder::default()
        .raw_line("#![allow(clippy::just_underscores_and_digits, clippy::unreadable_literal, clippy::missing_safety_doc, clippy::useless_transmute, clippy::redundant_static_lifetimes, improper_ctypes, dead_code)]")
        .header("inc/mylib.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .use_core()
        .opaque_type("std::.*")
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(Path::new("src").join("bindings.rs"))
        .expect("Couldn't write bindings");
}
