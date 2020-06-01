

use std::env;

fn main() {
//    let out_dir = env::var("OUT_DIR").unwrap();
    let work_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    #[cfg(feature = "stm32f3x")]
    println!("cargo:rustc-link-search={}/static_libs/stm32f3x",  work_dir);

    #[cfg(feature = "stm32f4x")]
    println!("cargo:rustc-link-search={}/static_libs/stm32f4x",  work_dir);

    #[cfg(feature = "stm32f7x")]
    println!("cargo:rustc-link-search={}/static_libs/stm32f7x",  work_dir);

    #[cfg(feature = "stm32h7x")]
    println!("cargo:rustc-link-search={}/static_libs/stm32h7x",  work_dir);

    println!("cargo:rustc-link-lib=cmsis_rtos2");

}

