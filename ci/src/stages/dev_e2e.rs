use crate::*;

pub struct DevE2E {}

const BROWSERS: [&str; 2] = [
    "firefox",
    "chrome",
];

impl Stage for DevE2E {
    fn name(&self) -> &'static str {
        "Dev_E2E"
    }

    // run e2e tests against dev servers
    fn run(&self, context: &Context, runner: &dyn Runner) -> Result<(), Error> {
        let expiration = 3000;

        node_dev_e2e(expiration, runner, context)?;

        rust_dev_e2e(expiration, runner, context)
    }
}

fn node_dev_e2e(expiration: u32, runner: &dyn Runner, context: &Context) -> Result<(), Error> {
    let server = runner.run_background(
        ExecutionContext::Build,
        vec![],
        true,
        &format!(r"
            set -xe
            cd ui
            npm run dev -- --user-expiration {0} --api-expiration {0}
        ", expiration),
    )?;

    runner.run(
        ExecutionContext::E2E,
        vec![],
        true,
        &cypress_script(
            &server.addr(),
            "node-dev",
            &BROWSERS,
            context.headed,
        ),
    )
}

fn rust_dev_e2e(expiration: u32, runner: &dyn Runner, context: &Context) -> Result<(), Error> {
    runner.run(
        ExecutionContext::Build,
        vec![&format!(
            "WIKI_CI_UI_BUILD_OPTIONS=--build dev --api server --user-expiration {0} --api-expiration {0}",
            expiration,
        )],
        true,
            r"
                set -xe
                cargo build --bin wiki
            ",
    )?;

    for browser in BROWSERS {
        let db = runner.run_background(
            ExecutionContext::Postgres,
            vec!["POSTGRES_HOST_AUTH_METHOD=trust"],
            false,
            "",
        )?;

        std::thread::sleep(std::time::Duration::from_secs(10));

        let server = runner.run_background(
            ExecutionContext::Build,
            vec![],
            true,
            &format!(r#"
                set -xe
                ./target/debug/wiki --postgres-host={}
            "#, &db.addr()),
        )?;

        std::thread::sleep(std::time::Duration::from_secs(10));

        runner.run(
            ExecutionContext::E2E,
            vec![
                &format!("CYPRESS_USER_EXPIRATION={}", expiration),
                &format!("CYPRESS_API_URL=http://{}:8080/api/v1", server.addr()),
                "CYPRESS_REQUIRE_CLEAN_PERSISTENCE=true",
            ],
            true,
            &cypress_script(
                &server.addr(),
                "rust-dev",
                &[browser],
                context.headed,
            ),
        )?;
    }

    Ok(())
}

fn cypress_script(server_addr: &str, stage_name: &str, browsers: &[&str], headed: bool) -> String {
    let headed = if headed { "--headed" } else { "--headless" };
    format!(r"
        set -xe
        cd ui
        trap 'mv *-e2e.xml ../test_results/' EXIT
        for b in {0}; do
            CYPRESS_MOCHA_FILE={1}-$b-e2e.xml npx cypress run \
                {2} \
                --browser $b \
                --config baseUrl=http://{3}:8080
        done
    ", browsers.join(" "), stage_name, headed, server_addr)
}
