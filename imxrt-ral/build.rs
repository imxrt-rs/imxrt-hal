use std::env;
use std::fs;
use std::path::PathBuf;
fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());
        let device_file = if env::var_os("CARGO_FEATURE_IMXRT1011").is_some() {
            "src/imxrt/imxrt1011/device.x"
        } else if env::var_os("CARGO_FEATURE_IMXRT1015").is_some() {
            "src/imxrt/imxrt1015/device.x"
        } else if env::var_os("CARGO_FEATURE_IMXRT1021").is_some() {
            "src/imxrt/imxrt1021/device.x"
        } else if env::var_os("CARGO_FEATURE_IMXRT1051").is_some() {
            "src/imxrt/imxrt1051/device.x"
        } else if env::var_os("CARGO_FEATURE_IMXRT1052").is_some() {
            "src/imxrt/imxrt1052/device.x"
        } else if env::var_os("CARGO_FEATURE_IMXRT1061").is_some() {
            "src/imxrt/imxrt1061/device.x"
        } else if env::var_os("CARGO_FEATURE_IMXRT1062").is_some() {
            "src/imxrt/imxrt1062/device.x"
        } else if env::var_os("CARGO_FEATURE_IMXRT1064").is_some() {
            "src/imxrt/imxrt1064/device.x"
        } else {
            panic!("No device features selected");
        };
        fs::copy(device_file, out.join("device.x")).unwrap();
        println!("cargo:rerun-if-changed={}", device_file);
    }
    println!("cargo:rerun-if-changed=build.rs");
}
