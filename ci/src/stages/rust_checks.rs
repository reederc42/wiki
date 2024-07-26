use crate::*;

pub struct RustChecks {}

impl Stage for RustChecks {
    fn name(&self) -> &'static str {
        "Rust_Checks"
    }

    // run runs unit tests and linters for Rust
    fn run(&self, _context: &Context, runner: &dyn Runner) -> Result<(), Error> {
        let db = runner.run_background(
            ExecutionContext::Postgres,
            vec!["POSTGRES_HOST_AUTH_METHOD=trust"],
            false,
            "",
        )?;

        runner.run(
            ExecutionContext::Build,
            vec![&format!("WIKI_CI_TEST_POSTGRES_HOST={}", &db.addr())],
            true,
            r"
                set -xe
                RUSTFLAGS='-Dwarnings' cargo clippy --all-targets --all-features
                cargo nextest run --run-ignored all --config-file .nextest-config.toml
                mv target/nextest/default/rust-unit.xml test_results/
            ",
        )
    }
}
