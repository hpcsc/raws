use std::process::Child;
use std::process::{Command, Stdio};
use std::io::{Write};

fn to_string_without_whitespace(input: Vec<u8>) -> Result<String, String> {
    Ok(String::from(String::from_utf8(input).unwrap().trim_end()))
}

fn spawn_fzf_command() -> Result<Child, String> {
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
            .or(Err(String::from("failed to invoke fzf command")))
}

fn write_to_fzf_stdin(fzf_command: &mut Child, profiles: Vec<String>) -> Result<(), String> {
    let fzf_stdin = fzf_command.stdin.as_mut().ok_or(String::from("failed to access fzf stdin"))?;
    fzf_stdin.write_all(profiles.join("\n").as_bytes()).or(Err(String::from("failed to pass input to fzf command")))
}

pub fn choose_profile(profiles: Vec<String>) -> Result<String, String> {
    let mut fzf_command = spawn_fzf_command()?;
    write_to_fzf_stdin(&mut fzf_command, profiles)?;

    let output = fzf_command.wait_with_output().or(Err(String::from("error while executing fzf")))?;
    let output_value = if output.status.success()  { output.stdout } else { output.stderr };
    to_string_without_whitespace(output_value)
}
