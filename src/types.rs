use std::{io, process};

use serde::Deserialize;

#[derive(Default, Deserialize)]
pub(crate) struct Task {
    pub name: String,
    pub module: String,
    pub language: String,
    pub input: Option<Box<Input>>,
}

#[derive(Default, Deserialize)]
pub(crate) struct Input {
    pub value: Option<u8>,
    pub from: Option<String>,
}

#[derive(Default, Deserialize)]
pub(crate) struct Pipeline {
    pub name: String,
    pub runtime: String,
    pub tasks: Vec<Task>,
}

impl Pipeline {
    pub fn execute(&self) -> Result<(), io::Error> {
        println!("[info] Pipeline started: '{}'", &self.name);

        let mut output: Option<String> = None;
        for task in &self.tasks {
            let mut args = vec![
                String::from(format!("tasks/modules/{}.wasm", task.module)),
                String::from("--invoke"),
                String::from("run"),
            ];

            let mut cmd = process::Command::new(self.runtime.as_str());
            if let Some(input) = &task.input {
                args.push(String::from("--args"));

                if let Some(v) = input.value {
                    args.push(v.to_string());
                }

                if input.from != None {
                    if let Some(o) = &output {
                        args.push(o.to_string());
                    }
                }
            }

            println!("  [info] Executing '{}' ({})", &task.name, &task.language);
            let out = cmd.args(args).output()?;
            if out.stderr.len() > 0 {
                println!("  [error] Executing '{}'", &self.name);

                let err = String::from_utf8(out.stderr).unwrap();
                return Err(io::Error::new(io::ErrorKind::Other, err));
            }

            if out.stdout.len() > 0 {
                let res = String::from_utf8(out.stdout).unwrap();
                output = Some(res.trim().to_string());
            }
        }

        if let Some(o) = output {
            println!("  [info] Final result: {}", o);
        }

        println!("[info] Pipeline finished!");

        Ok(())
    }
}
