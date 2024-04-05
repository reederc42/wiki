use crate::*;

pub struct RustChecks {}

impl Stage for RustChecks {
    fn name(&self) -> String {
        String::from("rust_checks")
    }

    // run runs unit tests and linters for Rust
    fn run(&self, _context: &Context, config: &Config) -> Result<(), Error> {
        config.runner.run("build", r"
            set -xe
            RUSTFLAGS='-Dwarnings' cargo clippy --all-targets --all-features
            cargo test --all-targets --all-features
        ")
    }
}
