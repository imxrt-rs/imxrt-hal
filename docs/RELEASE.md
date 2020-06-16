## Publish a release

From a clean repository, at the root:

1. Update the `imxrt-ral` crate version in `imxrt-ral/imxrtral.py`.
2. Build the RAL: `make -C imxrt-ral`
3. In `imxrt-hal/Cargo.toml`, update both
    - the version of the HAL
    - the HAL's dependency of the RAL
4. Sanity check the RAL: 

```
cd imxrt-ral && cargo build --features imxrt1062 && cd ..
```

5. Publish the RAL: `cargo publish --manifest-path imxrt-ral/Cargo.toml`
6. Sanity check the HAL.

```
cd imxrt-hal && cargo build --features imxrt1062 && cd ..
```

7. Make a commit of the release.
8. Publish the HAL: `cargo publish --manifest-path imxrt-hal/Cargo.toml --features imxrt1062`