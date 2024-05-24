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
    fn run(&self, _context: &Context, config: &Config) -> Result<(), Error> {
        let expiration = 3000;

        node_dev_e2e(expiration, config)?;

        rust_dev_e2e(expiration, config)
    }
}

fn node_dev_e2e(expiration: u32, config: &Config) -> Result<(), Error> {
    let server = config.runner.run_background(
        ExecutionContext::Build,
        vec![],
        true,
        vec![
            "sh",
            "-c",
            &format!(r"
                set -xe
                ln -s /ci/node_modules ./ui/node_modules || true
                cd ui
                npm run dev -- --user-expiration {0} --api-expiration {0}
            ", expiration),
        ]
    )?;

    config.runner.run(
        ExecutionContext::E2E,
        vec![],
        true,
        vec![
            "sh",
            "-c",
            &cypress_script(&server.addr(), "node-dev", &BROWSERS),
        ]
    )
}

fn rust_dev_e2e(expiration: u32, config: &Config) -> Result<(), Error> {
    config.runner.run(
        ExecutionContext::Build,
        vec![],
        true,
        vec![
            "sh",
            "-c",
            &format!(r#"
                set -xe
                ln -s /ci/node_modules ./ui/node_modules || true
                export WIKI_CI_UI_BUILD_OPTIONS='--build dev --api server --user-expiration {0} --api-expiration {0}'
                cargo build -vv --bin wiki
            "#, expiration),
        ],
    )?;

    for browser in BROWSERS {
        let db = config.runner.run_background(
            ExecutionContext::Postgres,
            vec!["POSTGRES_HOST_AUTH_METHOD=trust"],
            false,
            vec![],
        )?;

        let server = config.runner.run_background(
            ExecutionContext::Build,
            vec![],
            true,
            vec![
                "sh",
                "-c",
                &format!(r"
                    set -xe
                    sleep 10s
                    ./target/debug/wiki --postgres-host={}
                ", &db.addr()),
            ],
        )?;

        std::thread::sleep(std::time::Duration::from_secs(10));

        config.runner.run(
            ExecutionContext::E2E,
            vec![
                &format!("CYPRESS_USER_EXPIRATION={}", expiration),
                &format!("CYPRESS_API_URL=http://{}:8080/api/v1", server.addr()),
                "CYPRESS_REQUIRE_CLEAN_PERSISTENCE=true",
            ],
            true,
            vec![
                "sh",
                "-c",
                &cypress_script(&server.addr(), "rust-dev", &[browser]),
            ],
        )?;
    }

    Ok(())
}

fn cypress_script(server_addr: &str, stage_name: &str, browsers: &[&str]) -> String {
    format!(r"
        set -xe
        ln -s /ci/node_modules ./ui/node_modules || true
        cd ui
        trap 'mv *-e2e.xml ../test_results/' EXIT
        for b in {0}; do
            CYPRESS_MOCHA_FILE={2}-$b-e2e.xml npx cypress run \
                --browser $b \
                --config baseUrl=http://{1}:8080
        done
    ", browsers.join(" "), server_addr, stage_name)
}
