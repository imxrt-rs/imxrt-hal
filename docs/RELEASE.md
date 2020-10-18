## Publish a release

From a clean repository, at the root:

1. Update the `imxrt-iomuxc` crate versions.
2. Update the `imxrt-ral` crate version in `imxrt-ral/imxrtral.py`.
3. In each HAL such as imxrt1062-hal/Cargo.toml`, update both
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
8. Publish the HAL(s):
    ```
    cargo publish --manifest-path imxrt1062-hal/Cargo.toml
    ```

## Maintaining older releases

This section describes how imxrt-rs project maintainers support older releases.
If there is a bug fix that you would like to apply to an older version of the
RAL, HAL, or IOMUXC crates, follow the process below to create a new patch
release.

- Integrate bug fixes on the main branch.
- If it doesn't already exist, create a maintenance branch. The maintenance branch should
  branch from the commit tagged with the release. It should be named `maint-v[VERSION]`,
  where `VERSION` is the major and minor release numbers.
- Cherry-pick or backport the patches onto the maintenance branch.
- Tag the release on the maintenance branch.
