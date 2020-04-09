## Publish a release

From a clean repository, at the root:

1. Update the `imxrt-ral` crate version in `imxrt-ral/imxrtral.py`.
2. Build the RAL: `make -C imxrt-ral`
3. Sanity check the RAL: `cargo build -p imxrt-ral --features imxrt1062`
4. Publish the RAL: `cargo publish --manifest-path imxrt-ral/Cargo.toml`
5. In `imxrt-hal/Cargo.toml`, update both
    - the version of the HAL
    - the HAL's dependency of the RAL
6. Sanity check the HAL. Note that this will reference the RAL published to crates.io in step 4.

```
cd imxrt-hal && cargo build --features imxrt1062 && cd ..
```

7. Make a commit of the release.
8. Publish the HAL: `cargo publish --manifest-path imxrt-hal/Cargo.toml --features imxrt1062`