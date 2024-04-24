use crate::*;

pub struct RustChecks {}

impl Stage for RustChecks {
    fn name(&self) -> &'static str {
        "Rust_Checks"
    }

    // run runs unit tests and linters for Rust
    fn run(&self, _context: &Context, config: &Config) -> Result<(), Error> {
        let db = config.runner.run_background(
            ExecutionContext::Postgres,
            vec!["POSTGRES_HOST_AUTH_METHOD=trust"],
            false,
            Vec::new(),
        )?;

        config.runner.run(
            ExecutionContext::Build,
            Vec::new(),
            true,
            vec![
                "sh",
                "-c",
                &format!(r"
                    set -xe
                    RUSTFLAGS='-Dwarnings' cargo clippy --all-targets --all-features
                    export WIKI_CI_TEST_POSTGRES_HOST={}
                    cargo nextest run --run-ignored all --config-file .nextest-config.toml
                    mv target/nextest/rust-unit.xml test_results/
                ", &db.addr()),
            ],
        )
    }
}
