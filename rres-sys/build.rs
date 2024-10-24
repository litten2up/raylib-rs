use std::{env, path::PathBuf};

fn gen_rres() {
    // Compile the code and link with cc crate
    #[cfg(target_os = "windows")]
    {
        cc::Build::new()
            .files(vec!["binding/rres_wrapper.cpp"])
            .include("binding")
            .warnings(false)
            // .flag("-std=c99")
            .extra_warnings(false)
            .compile("rres");
    }
    #[cfg(not(target_os = "windows"))]
    {
        cc::Build::new()
            .files(vec!["binding/rres_wrapper.c"])
            .include("binding")
            .warnings(false)
            // .flag("-std=c99")
            .extra_warnings(false)
            .compile("rres");
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let builder = bindgen::builder()
        .header("binding/binding.h")
        .rustified_enum(".+")
        // generate nothing from Raylib, since we're linking it to raylib_sys anyways.
        .blocklist_file("binding/raylib.h")
        .clang_arg("-std=c99");

    // Build
    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    gen_rres();
    Ok(())
}
