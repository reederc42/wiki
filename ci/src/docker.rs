use std::process::Command;

use crate::*;

// Latest postgres image: https://hub.docker.com/_/postgres/tags
const POSTGRES_IMAGE: &str = "postgres:16-alpine";

pub struct Docker {
    pub context: Rc<Context>,
    pub user: String,
}

impl Docker {
    fn build_docker_args(&self, background: bool, context: ExecutionContext, env: Vec<&str>, include_source: bool, cmd: Vec<&str>) -> Vec<String> {
        let mut args: Vec<String> = vec![
            "run",
            "--rm",
        ].into_iter().map(|a| a.to_string()).collect();

        // Postgres cannot change user
        if !self.user.is_empty() && !matches!(context, ExecutionContext::Postgres) {
            args.push("-u".into());
            args.push(self.user.clone());
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

        args.extend(cmd.into_iter().map(|a| a.to_string()).collect::<Vec<String>>());

        args
    }
}

impl Builder for Docker {
    fn build(&self, tag: &str, dockerfile: &str, context: &str) -> Result<(), Error> {
        let cmd = Command::new("docker")
            .args([
                "build",
                "--build-arg",
                "CI_USER=1000:1000",
                "-t",
                tag,
                "-f",
                dockerfile,
                context,
            ])
            .spawn();

        match cmd {
            Ok(mut c) => {
                match c.wait() {
                    Ok(s) => {
                        match s.code() {
                            Some(code) => {
                                if code != 0 {
                                    Err(error!("Received non-zero exit status: {}", code))
                                } else {
                                    Ok(())
                                }
                            },
                            None => Err(error!("Process terminated with signal")),
                        }
                    },
                    Err(e) => Err(Box::new(e)),
                }
            },
            Err(e) => Err(Box::new(e)),
        }
    }
}

impl Runner for Docker {
    fn run(&self, context: ExecutionContext, env: Vec<&str>, include_source: bool, cmd: Vec<&str>) -> Result<(), Error> {
        let args = self.build_docker_args(false, context, env, include_source, cmd);

        let cmd = Command::new("docker")
            .args(args)
            .spawn();

        match cmd {
            Ok(mut c) => {
                match c.wait() {
                    Ok(s) => {
                        match s.code() {
                            Some(code) => {
                                if code != 0 {
                                    Err(error!("Received non-zero exit status: {}", code))
                                } else {
                                    Ok(())
                                }
                            },
                            None => Err(error!("Process terminated with signal")),
                        }
                    },
                    Err(e) => Err(Box::new(e)),
                }
            },
            Err(e) => Err(Box::new(e)),
        }
    }

    fn run_background(&self, context: ExecutionContext, env: Vec<&str>, include_source: bool, cmd: Vec<&str>) -> Result<Box<dyn BackgroundServer>, Error> {
        let args = self.build_docker_args(true, context, env, include_source, cmd);

        let output = Command::new("docker")
            .args(args)
            .output();

        match output {
            Ok(o) => {
                let s = std::str::from_utf8(&o.stdout).unwrap().trim_end();
                Ok(Box::new(DockerBackgroundServer{ id: s.to_string() }))
            },
            Err(e) => Err(Box::new(e)),
        }
    }
}

struct DockerBackgroundServer {
    id: String,
}

impl Drop for DockerBackgroundServer {
    fn drop(&mut self) {
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
                "stop",
                &self.id,
            ])
            .spawn()
            .unwrap_or_else(|_| panic!("Failed to stop container: {}", self.id))
            .wait()
            .unwrap_or_else(|_| panic!("Failed to stop container: {}", self.id));
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
