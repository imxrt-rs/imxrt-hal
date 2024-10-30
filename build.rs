use std::{collections::HashSet, env};

fn features_enabled() -> HashSet<String> {
    env::vars()
        .map(|(key, _)| key)
        .flat_map(|feature| {
            feature
                .strip_prefix("CARGO_FEATURE_")
                .map(str::to_lowercase)
        })
        .collect()
}

fn features_10xx() -> HashSet<String> {
    [
        "imxrt1010",
        "imxrt1015",
        "imxrt1020",
        "imxrt1050",
        "imxrt1060",
        "imxrt1064",
    ]
    .iter()
    .map(ToString::to_string)
    .collect()
}

fn features_11xx() -> HashSet<String> {
    ["imxrt1160", "imxrt1170", "imxrt1180"]
        .iter()
        .map(ToString::to_string)
        .collect()
}

fn features_chip() -> HashSet<String> {
    features_10xx().into_iter().chain(features_11xx()).collect()
}

fn emit_cfg_checks<F>(cfg: &str, values: impl IntoIterator<Item = F>)
where
    F: std::fmt::Display,
{
    let quoted: Vec<String> = values
        .into_iter()
        .map(|value| format!("\"{}\"", value))
        .collect();
    let joined = quoted.join(", ");
    // Single ":" permitted for backwards compatibility.
    println!("cargo:rustc-check-cfg=cfg({cfg}, values({joined}))");
}

fn main() {
    let all_features = features_enabled();
    let feat_chip: HashSet<_> = features_chip();

    emit_cfg_checks(
        "chip",
        feat_chip.iter().chain(std::iter::once(&"none".into())),
    );

    let enabled_chip: Vec<_> = all_features.intersection(&feat_chip).collect();
    assert!(
        enabled_chip.len() < 2,
        "Too many chip features! Select one of {:?}",
        enabled_chip
    );

    if let Some(chip) = enabled_chip.first() {
        println!("cargo:rustc-cfg=chip=\"{chip}\"");
    } else {
        println!("cargo:rustc-cfg=chip=\"none\"");
    }
}
