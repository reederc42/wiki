use std::process::Command;

use crate::*;

// Latest postgres version: https://hub.docker.com/_/postgres/tags
const POSTGRES_IMAGE: &str = "postgres:16-alpine";

pub struct Docker {
    pub context: Rc<Context>,
}

impl Docker {
    fn build_docker_run_args(&self, background: bool, context: ExecutionContext, env: Vec<&str>, include_source: bool, script: &str, remove: bool) -> Vec<String> {
        let mut args: Vec<String> = vec![
            "run",
        ].into_iter().map(|a| a.to_string()).collect();

        if remove {
            args.push("--rm".to_string());
        }

        if background {
            args.push("-d".to_string());
        }

        if include_source {
            args.append(&mut vec![
                "-v",
                &format!("{}:{}", self.context.cwd, self.context.cwd),
                "-w",
                &self.context.cwd,
            ].into_iter().map(|a| a.to_string()).collect());
        }

        for e in env {
            args.push("-e".into());
            args.push(e.into());
        }

        match context {
            ExecutionContext::Build => args.push(format!("wiki-ci:build-{}", self.context.id)),
            ExecutionContext::E2E => args.push(format!("wiki-ci:e2e-{}", self.context.id)),
            ExecutionContext::Postgres => args.push(POSTGRES_IMAGE.into()),
        }

        if !script.is_empty() {
            args.extend([
                "sh",
                "-c",
                script,
            ].into_iter().map(|a| a.to_string()).collect::<Vec<String>>());
        }

        args
    }

    fn build_docker_image(&self, tag: &str, dockerfile: &str, context: &str) -> Result<(), Error> {
        let mut prog = Command::new("docker");
        let cmd = prog.args([
                "build",
                "-t",
                tag,
                "-f",
                dockerfile,
                context,
            ]);

        let mut finished_print = None;
        if self.context.verbose {
            finished_print = Some(shell::print_command(cmd, false));
        }

        shell::spawn_result_to_result(cmd.spawn(), finished_print)
    }

    fn pull_postgres_image(&self, image: &str) -> Result<(), Error> {
        let mut prog = Command::new("docker");
        let cmd = prog.args([
            "pull",
            image,
        ]);

        let mut finished_print = None;
        if self.context.verbose {
            finished_print = Some(shell::print_command(cmd, false));
        }

        shell::spawn_result_to_result(cmd.spawn(), finished_print)
    }
}

impl Runner for Docker {
    fn build(&self, exec_context: ExecutionContext, build_context: &Context) -> Result<(), Error> {
        match exec_context {
            ExecutionContext::Build => {
                self.build_docker_image(
                    &format!("wiki-ci:build-{}", build_context.id),
                    &format!("{}/images/build.Dockerfile", build_context.cwd),
                    &build_context.cwd,
                )?;
                self.run(
                    ExecutionContext::Build,
                    vec![],
                    true,
                    "ln -s /ci/ui/node_modules ui/node_modules || true",
                )
            },
            ExecutionContext::E2E => {
                self.build_docker_image(
                    &format!("wiki-ci:e2e-{}", build_context.id),
                    &format!("{}/images/e2e.Dockerfile", build_context.cwd),
                    &build_context.cwd,
                )?;
                self.run(
                    ExecutionContext::Build,
                    vec![],
                    true,
                    "ln -s /ci/ui/node_modules ui/node_modules || true",
                )
            },
            ExecutionContext::Postgres => self.pull_postgres_image(POSTGRES_IMAGE),
        }
    }

    fn run(&self, context: ExecutionContext, env: Vec<&str>, include_source: bool, script: &str) -> Result<(), Error> {
        let args = self.build_docker_run_args(false, context, env, include_source, script, true);

        let mut prog = Command::new("docker");
        let cmd = prog.args(args);

        let mut finished_print = None;
        if self.context.verbose {
            finished_print = Some(shell::print_command(cmd, false));
        }

        shell::spawn_result_to_result(cmd.spawn(), finished_print)
    }

    fn run_background(&self, context: ExecutionContext, env: Vec<&str>, include_source: bool, script: &str) -> Result<Box<dyn BackgroundServer>, Error> {
        let args = self.build_docker_run_args(true, context, env, include_source, script, false);

        let mut prog = Command::new("docker");
        let cmd = prog.args(args);

        let mut finished_print = None;
        if self.context.verbose {
            finished_print = Some(shell::print_command(cmd, true));
        }

        let output = cmd.output();

        match output {
            Ok(o) => {
                let s = std::str::from_utf8(&o.stdout).unwrap().trim_end();
                Ok(Box::new(DockerBackgroundServer{
                    id: s.to_string(),
                    finished_print,
                }))
            },
            Err(e) => Err(Box::new(e)),
        }
    }
}

struct DockerBackgroundServer {
    id: String,
    finished_print: Option<Box<dyn FnOnce()>>,
}

impl Drop for DockerBackgroundServer {
    fn drop(&mut self) {
        Command::new("docker")
            .args([
                "stop",
                &self.id,
            ])
            .spawn()
            .unwrap_or_else(|_| panic!("Failed to stop container: {}", self.id))
            .wait()
            .unwrap_or_else(|_| panic!("Failed to stop container: {}", self.id));

        Command::new("docker")
            .args([
                "logs",
                &self.id,
            ])
            .spawn()
            .unwrap_or_else(|_| panic!("Failed to collect logs: {}", self.id))
            .wait()
            .unwrap_or_else(|_| panic!("Failed to collect logs: {}", self.id));

        Command::new("docker")
            .args([
                "rm",
                &self.id,
            ])
            .spawn()
            .unwrap_or_else(|_| panic!("Failed to remove container: {}", self.id))
            .wait()
            .unwrap_or_else(|_| panic!("Failed to remove container: {}", self.id));

        if let Some(f) = self.finished_print.take() {
            f();
        }
    }
}

impl BackgroundServer for DockerBackgroundServer {
    fn addr(&self) -> String {
        let output = Command::new("docker")
            .args([
                "inspect",
                &self.id,
            ])
            .output()
            .unwrap_or_else(|_| panic!("Failed to inspect: {}", self.id))
            .stdout;

        let container: serde_json::Value =
            serde_json::from_slice(&output).unwrap();

        String::from(
            container[0]["NetworkSettings"]["IPAddress"].as_str().unwrap()
        )
    }
}
