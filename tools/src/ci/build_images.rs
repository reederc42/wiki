pub struct BuildImages {}

impl crate::ci::Stage for BuildImages {
    fn name(&self) -> String {
        String::from("build_images")
    }

    // run builds test and build images
    fn run(&self, context: &crate::ci::Context, config: &crate::ci::Config) -> Option<Box<dyn std::error::Error>> {
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
