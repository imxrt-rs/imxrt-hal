## Publish a release

From a clean repository, at the root:

1. Update the `imxrt-iomuxc` crate versions.
2. Update the `imxrt-ral` crate version in `imxrt-ral/imxrtral.py`.
3. In `imxrt-hal/Cargo.toml`, update both
    - the version of the HAL
    - the HAL's dependency of the RAL
4. Generate the RAL: `make -C imxrt-ral`
5. Commit the changes, and create a tag.
6. Publish the IOMUXC crates:
    ```
    cargo publish --manifest-path imxrt-iomuxc/imxrt-iomuxc-build/Cargo.toml
    cargo publish --manifest-path imxrt-iomuxc/Cargo.toml --all-features
    ```
7. Publish the RAL:
    ```
    cargo publish --manifest-path imxrt-ral/Cargo.toml
    ```
8. Publish the HAL:
    ```
    cargo publish --manifest-path imxrt-hal/Cargo.toml --features imxrt1062
    ```