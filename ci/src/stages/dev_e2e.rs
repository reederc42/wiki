use crate::*;

pub struct DevE2E {}

impl Stage for DevE2E {
    fn name(&self) -> String {
        String::from("dev_e2e")
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
            ln -s /ci/node_modules ./ui/node_modules || true
            cd ui
            node tools/configure.js --user-expiration {1} --api-expiration {1}
            npx cypress run --browser firefox --config baseUrl=http://{0}:8080
            npx cypress run --browser chrome --config baseUrl=http://{0}:8080
        ", server.addr(), expiration))
}

fn rust_dev_e2e(expiration: u32, config: &Config) -> Result<(), Error> {
    let server = config.runner.run_background_server("build", &format!(r"
            set -xe
            ln -s /ci/node_modules ./ui/node_modules || true
            cd ui
            npm run build -- --build dev --user-expiration {0} --api-expiration {0}
            cd ..
            cargo run --bin wiki
        ", expiration))?;

        config.runner.run("e2e", &format!(r"
            set -xe
            ln -s /ci/node_modules ./ui/node_modules || true
            cd ui
            node tools/configure.js --user-expiration {1} --api-expiration {1}
            npx cypress run --browser firefox --config baseUrl=http://{0}:8080
            npx cypress run --browser chrome --config baseUrl=http://{0}:8080
        ", server.addr(), expiration))
}
