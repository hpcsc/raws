use std::process::{Command, Stdio};
use std::io::{Write};

fn to_string_without_whitespace(input: Vec<u8>) -> String {
    String::from(String::from_utf8(input).unwrap().trim_end())
}

pub fn choose_profile(profiles: Vec<String>) -> String {
    let mut fzf_command = Command::new("fzf")
            .args(&[
                "--height",
                "30%",
                "--reverse",
                "-1",
                "-0",
                "--header",
                "'Select AWS profile'"
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

    fzf_command.stdin
        .as_mut()
        .unwrap()
        .write_all(profiles.join("\n").as_bytes());

    let output = fzf_command.wait_with_output().unwrap();
    if output.status.success() {
        to_string_without_whitespace(output.stdout)
    }
    else {
        to_string_without_whitespace(output.stderr)
    }
}
