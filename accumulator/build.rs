use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=inc/");
    println!("cargo:rerun-if-changed=build.rs");

    cc::Build::new()
        .files(["inc/mylib.c"].iter())
        .include(Path::new("inc"))
        .opt_level(3)
        .extra_warnings(true)
        .compile("mylib");

    println!("cargo:rustc-link-lib=mylib");
}
