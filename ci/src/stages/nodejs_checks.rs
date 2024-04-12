use crate::*;

pub struct NodeJSChecks {}

impl Stage for NodeJSChecks {
    fn name(&self) -> &'static str {
        "NodeJS_Checks"
    }

    // run runs unit tests and linters for Node.js source
    fn run(&self, _context: &Context, config: &Config) -> Result<(), Error> {
        config.runner.run("build", r"
            set -xe
            ln -s /ci/node_modules ./ui/node_modules || true
            export ESLINT_USE_FLAT_CONFIG=false
            cd ui
            npm run lint
            npm run test
        ")
    }
}
