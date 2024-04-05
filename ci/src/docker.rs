use std::process::Command;

use crate::*;

pub struct Docker {
    pub context: Rc<Context>,
}

impl Builder for Docker {
    fn build(&self, tag: &str, dockerfile: &str, context: &str) -> Result<(), Error> {
        let cmd = Command::new("docker")
            .args([
                "build",
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
    fn run(&self, context: &str, script: &str) -> Result<(), Error> {
        let cmd = Command::new("docker")
            .args([
                "run",
                "--rm",
                "-v",
                &format!("{}:{}", self.context.cwd, self.context.cwd),
                "-w",
                &self.context.cwd,
                &format!("{}:{}-{}", "wiki-ci", context, self.context.id),
                "sh",
                "-c",
                script,
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

    fn run_background_server(&self, context: &str, script: &str) -> Result<Box<dyn BackgroundServer>, Error> {
        let output = Command::new("docker")
            .args([
                "run",
                "--rm",
                "-d",
                "-v",
                &format!("{}:{}", self.context.cwd, self.context.cwd),
                "-w",
                &self.context.cwd,
                &format!("{}:{}-{}", "wiki-ci", context, self.context.id),
                "sh",
                "-c",
                script,
            ])
            .output();

        match output {
            Ok(o) => {
                let s = std::str::from_utf8(&o.stdout).unwrap().trim_end();
                Ok(Box::new(DockerBackgroundServer{ id: s.to_string() }))
            }
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
