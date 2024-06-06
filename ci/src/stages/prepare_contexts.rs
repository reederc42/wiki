use crate::*;

pub struct PrepareContexts {}

impl Stage for PrepareContexts {
    fn name(&self) -> &'static str {
        "Prepare_Contexts"
    }

    // run builds test and build images
    fn run(&self, context: &Context, config: &Config) -> Result<(), Error> {
        let exec_contexts: [ExecutionContext; 3] = [
            ExecutionContext::Build,
            ExecutionContext::E2E,
            ExecutionContext::Postgres,
        ];

        for exec_context in exec_contexts {
            config.builder.build(exec_context, context)?;
        }

        Ok(())
    }
}
