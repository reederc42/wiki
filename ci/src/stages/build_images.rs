use crate::*;

pub struct BuildImages {}

impl Stage for BuildImages {
    fn name(&self) -> &'static str {
        "Build_Images"
    }

    // run builds test and build images
    fn run(&self, context: &Context, config: &Config) -> Result<(), Error> {
        config.builder.build(
            &format!("wiki-ci:build-{}", context.id),
            "images/build.Dockerfile",
            ".",
        )?;

        config.builder.build(
            &format!("wiki-ci:e2e-{}", context.id),
            "images/e2e.Dockerfile",
            ".",
        )
    }
}
