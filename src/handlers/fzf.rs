use std::process::Child;
use std::process::{Command, Stdio};
use std::io::{Write};
use std::error::Error;

fn to_string_without_whitespace(input: Vec<u8>) -> Result<String, Box<Error>> {
    Ok(String::from(String::from_utf8(input).unwrap().trim_end()))
}

fn spawn_fzf_command() -> std::io::Result<Child> {
    Command::new("fzf")
            .args(&[
                "--height", "30%",
                "--reverse",
                "-1",
                "-0",
                "--header", "'Select AWS profile'"
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
}

fn write_to_fzf_stdin(fzf_command: &mut Child, profiles: Vec<String>) -> Result<(), Box<Error>> {
    let fzf_stdin = fzf_command.stdin.as_mut().ok_or(String::from("failed to access fzf stdin"))?;
    fzf_stdin.write_all(profiles.join("\n").as_bytes()).map_err(|e| e.into())
}

pub fn choose_profile(profiles: Vec<String>) -> Result<String, Box<Error>> {
    let mut fzf_command = spawn_fzf_command()?;
    write_to_fzf_stdin(&mut fzf_command, profiles)?;

    let output = fzf_command.wait_with_output()?;
    let output_value = if output.status.success()  { output.stdout } else { output.stderr };
    to_string_without_whitespace(output_value)
}
