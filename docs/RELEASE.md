## Publish a release

Ensure that all other imxrt-rs dependencies, like `imxrt-ral` and
`imxrt-iomuxc`, are already published. Then, publish the HAL:

```
cargo publish --manifest-path imxrt-hal/Cargo.toml --features imxrt1062
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
