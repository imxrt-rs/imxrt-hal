use std::env;

fn main() {
    env::vars()
        .map(|(k, _)| k)
        .flat_map(|feat| feat.strip_prefix("CARGO_FEATURE_").map(str::to_lowercase))
        .for_each(|feature| {
            println!("cargo:rustc-cfg=family=\"{}\"", feature);
        })
}
