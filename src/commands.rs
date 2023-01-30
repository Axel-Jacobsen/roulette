use std::borrow::Cow;
use std::io;
use std::process::Command;

use shell_escape::unix::escape;

fn synth_or(strs: Vec<String>) -> String {
    let or_expr = strs
        .into_iter()
        .reduce(|cur, nxt| {
            let escaped_next = escape(Cow::Owned(nxt));
            cur + "|" + &escaped_next
        })
        .expect("somehow failed to concat strings together!");

    format!("({})", or_expr)
}

pub fn git_grep(strs: Vec<String>) -> io::Result<Vec<String>> {
    let grep_str = synth_or(strs);
    let command_output = Command::new("git")
        .arg("grep")
        .arg("-niE")
        .arg(grep_str)
        .output()?;

    // assume stdout has valid utf8
    Ok(String::from_utf8(command_output.stdout)
        .unwrap()
        .split("\n")
        .map(String::from)
        .collect())
}
