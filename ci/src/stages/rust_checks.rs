use crate::*;

pub struct RustChecks {}

impl Stage for RustChecks {
    fn name(&self) -> &'static str {
        "Rust_Checks"
    }

    // run runs unit tests and linters for Rust
    fn run(&self, _context: &Context, config: &Config) -> Result<(), Error> {
        let db = config.runner.run_background(
            ExecutionContext::External("postgres:16-alpine"),
            vec!["POSTGRES_HOST_AUTH_METHOD=trust"],
            false,
            Vec::new(),
        )?;

        config.runner.run(
            ExecutionContext::Internal("build"),
            Vec::new(),
            true,
            vec![
                "sh",
                "-c",
                &format!(r"
                    set -xe
                    RUSTFLAGS='-Dwarnings' cargo clippy --all-targets --all-features
                    export WIKI_CI_TEST_POSTGRES_HOST={}
                    cargo test --all-targets --all-features -- --include-ignored
                ", &db.addr()),
            ],
        )
    }
}
