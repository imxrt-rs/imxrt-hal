## Adding new boards

1. Create a new board feature in [`Cargo.toml`](./Cargo.toml). This depends on support for
   your i.MX RT chip in both the RAL and IOMUXC crates.
2. Define a new alias in the repo's [`.cargo/config.toml`](../.cargo/config.toml). This makes
   it easier to build examples for your board.
3. Ad a new module in `src/`, and define your board configuration. Integrate your module into
   [`src/lib.rs`](./src/lib.rs).
