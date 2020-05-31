

use std::env;

fn main() {
//    let out_dir = env::var("OUT_DIR").unwrap();
    let work_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search={}/static_libs/",  work_dir);


    #[cfg(feature = "stm32f3x")]
    println!("cargo:rustc-link-lib=cmsis_rtos2_stm32f3");

    #[cfg(feature = "stm32f4x")]
    println!("cargo:rustc-link-lib=cmsis_rtos2_stm32f4");

    #[cfg(feature = "stm32f7x")]
    println!("cargo:rustc-link-lib=cmsis_rtos2_stm32f7");

    #[cfg(feature = "stm32h7x")]
    println!("cargo:rustc-link-lib=cmsis_rtos2_stm32h7");
}

