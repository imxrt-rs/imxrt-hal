## Publish a release

From a clean repository, at the root:

1. If needed, publish a new imxrt-ral following instructions there for release.
2. In each HAL such as imxrt1060-hal/Cargo.toml`, update both
    - the version of the HAL
    - the HAL's dependency of the RAL
3. Commit the changes, and create a tag.
4. Publish the HAL(s):
    ```
    cargo publish --manifest-path imxrt1060-hal/Cargo.toml
    ```

## Maintaining older releases

This section describes how imxrt-rs project maintainers support older releases.
If there is a bug fix that you would like to apply to an older version of the
RAL or HAL crates, follow the process below to create a new patch
release.

- Integrate bug fixes on the main branch.
- If it doesn't already exist, create a maintenance branch. The maintenance branch should
  branch from the commit tagged with the release. It should be named `maint-v[VERSION]`,
  where `VERSION` is the major and minor release numbers.
- Cherry-pick or backport the patches onto the maintenance branch.
- Tag the release on the maintenance branch.
