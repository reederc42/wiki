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
        let expiration = 1500;

        node_dev_e2e(expiration, config)?;

        rust_dev_e2e(expiration, config)
    }
}

fn node_dev_e2e(expiration: u32, config: &Config) -> Result<(), Error> {
    let server = config.runner.run_background(
        ExecutionContext::Build,
        Vec::new(),
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
        Vec::new(),
        true,
        vec![
            "sh",
            "-c",
            &cypress_script(expiration, &server.addr(), "node-dev", &BROWSERS),
        ]
    )
}

fn rust_dev_e2e(expiration: u32, config: &Config) -> Result<(), Error> {
    config.runner.run(
        ExecutionContext::Build,
        Vec::new(),
        true,
        vec![
            "sh",
            "-c",
            &format!(r"
                set -xe
                ln -s /ci/node_modules ./ui/node_modules || true
                cd ui
                npm run build -- --build dev --api server --user-expiration {0} --api-expiration {0}
                cd ..
                cargo build --bin wiki
            ", expiration),
        ],
    )?;

    let db = config.runner.run_background(
        ExecutionContext::Postgres,
        vec!["POSTGRES_HOST_AUTH_METHOD=trust"],
        false,
        Vec::new(),
    )?;

    let server = config.runner.run_background(
        ExecutionContext::Build,
        Vec::new(),
        true,
        vec![
            "sh",
            "-c",
            &format!(r"
                set -xe
                sleep 10s
                RUST_LOG=info ./target/debug/wiki --postgres-host={}
            ", &db.addr()),
        ],
    )?;

    config.runner.run(
        ExecutionContext::E2E,
        vec![
            &format!("CYPRESS_API_URL=http://{}:8080/api/v1", server.addr()),
            "CYPRESS_REQUIRE_CLEAN_PERSISTENCE=true",
        ],
        true,
        vec![
            "sh",
            "-c",
            &cypress_script(expiration, &server.addr(), "rust-dev", &BROWSERS),
        ],
    )
}

fn cypress_script(expiration: u32, server_addr: &str, stage_name: &str, browsers: &[&str]) -> String {
    format!(r"
        set -xe
        ln -s /ci/node_modules ./ui/node_modules || true
        cd ui
        node tools/configure.js --user-expiration {0} --api-expiration {0}
        trap 'mv *-e2e.xml ../test_results/' EXIT
        for b in {1}; do
            CYPRESS_MOCHA_FILE={3}-$b-e2e.xml npx cypress run \
                --browser $b \
                --config baseUrl=http://{2}:8080
        done
    ", expiration, browsers.join(" "), server_addr, stage_name)
}
