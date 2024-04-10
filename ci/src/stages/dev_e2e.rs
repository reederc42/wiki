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
        let expiration = 1000;

        node_dev_e2e(expiration, config)?;

        rust_dev_e2e(expiration, config)
    }
}

fn node_dev_e2e(expiration: u32, config: &Config) -> Result<(), Error> {
    let server = config.runner.run_background_server("build", &format!(r"
        set -xe
        ln -s /ci/node_modules ./ui/node_modules || true
        cd ui
        npm run dev -- --user-expiration {0} --api-expiration {0}
    ", expiration))?;

    config.runner.run("e2e", &format!(r"
        set -xe
        {}
    ", make_cypress_script(expiration, &server.addr(), &BROWSERS)))
}

fn rust_dev_e2e(expiration: u32, config: &Config) -> Result<(), Error> {
    config.runner.run("build", &format!(r"
        set -xe
        ln -s /ci/node_modules ./ui/node_modules || true
        cd ui
        npm run build -- --build dev --user-expiration {0} --api-expiration {0}
        cd ..
        cargo build --bin wiki
    ", expiration))?;

    let server = config.runner.run_background_server("build", r"
        set -xe
        ./target/debug/wiki
    ")?;

    config.runner.run("e2e", &format!(r"
        set -xe
        {}
    ", make_cypress_script(expiration, &server.addr(), &BROWSERS)))
}

fn make_cypress_script(expiration: u32, server_addr: &str, browsers: &[&str]) -> String {
    format!(r"
        ln -s /ci/node_modules ./ui/node_modules || true
        cd ui
        node tools/configure.js --user-expiration {0} --api-expiration {0}
        for b in {1}; do
            npx cypress run --browser $b --config baseUrl=http://{2}:8080
        done
    ", expiration, browsers.join(" "), server_addr)
}
