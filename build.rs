#[cfg(feature = "copy-sample-patches")]
use std::env;

#[cfg(feature = "copy-sample-patches")]
fn main() {
    println!("cargo:rerun-if-changed=patches/*");

    let profile = env::var("PROFILE").expect("PROFILE environment variable must be defined");

    copy_to_output::copy_to_output("patches", &profile)
        .expect("could not copy patches folder to target directory");
}

#[cfg(not(feature = "copy-sample-patches"))]
fn main() {}
