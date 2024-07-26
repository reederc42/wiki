use crate::*;

pub struct NodeJSChecks {}

impl Stage for NodeJSChecks {
    fn name(&self) -> &'static str {
        "NodeJS_Checks"
    }

    // run runs unit tests and linters for Node.js source
    fn run(&self, _context: &Context, runner: &dyn Runner) -> Result<(), Error> {
        runner.run(
            ExecutionContext::Build,
            vec![],
            true,
            r"
                set -xe
                cd ui
                npm run lint
                npm run test -- \
                    --test-reporter=spec \
                    --test-reporter-destination=stdout \
                    --test-reporter=junit \
                    --test-reporter-destination=../test_results/nodejs-unit-test.xml
            ",
        )
    }
}
