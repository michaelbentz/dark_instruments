use std::process::{Command, Stdio};

pub(crate) struct Process;

impl Process {
    pub(crate) fn exec(program: &str, args: &[&str]) -> Option<(String, String)> {
        Self::exec_command_and_capture_output_as_strings(program, args)
    }

    pub(crate) fn exec_and_return_stdout_bytes(program: &str, args: &[&str]) -> Option<Vec<u8>> {
        Self::exec_command_and_return_stdout_bytes(program, args)
    }

    fn exec_command_and_return_stdout_bytes(
        program: &str, args: &[&str],
    ) -> Option<Vec<u8>> {
        Self::exec_command_and_capture_output(
            program, args,
        ).map(|output| output.0)
    }

    fn exec_command_and_capture_output_as_strings(
        program: &str, args: &[&str],
    ) -> Option<(String, String)> {
        Self::exec_command_and_capture_output(
            program, args,
        ).map(|(stdout, stderr)| {
            (
                String::from_utf8_lossy(&stdout).trim().to_string(),
                String::from_utf8_lossy(&stderr).trim().to_string(),
            )
        })
    }

    fn exec_command_and_capture_output(
        program: &str, args: &[&str],
    ) -> Option<(Vec<u8>, Vec<u8>)> {
        println!(
            "EXEC: program='{}', arguments='{:#?}'",
            program,
            args.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" ")
        );
        let output = Command::new(program)
            .args(args)
            .stdout(Stdio::piped())
            .spawn()
            .and_then(|process| process.wait_with_output())
            .map(|output| (output.stdout, output.stderr))
            .unwrap_or_else(|err| {
                eprintln!("Error executing command: {}", err);
                (Vec::new(), Vec::new())
            });

        Some(output)
    }
}