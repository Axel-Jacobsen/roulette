use std::borrow::Cow;
use std::io;
use std::process::Command;
use std::process::Output;

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

pub fn git_grep(strs: Vec<String>) -> io::Result<Output> {
    let grep_str = synth_or(strs);
    Command::new("git")
        .arg("grep")
        .arg("-niE")
        .arg(grep_str)
        .output()
}

pub fn handle_git_grep<'a>(stdout: &'a String) -> std::str::Split<&'a str> {
    // the output is from stdout, so I *think* it is safe to assume
    // that it is valid utf-8... for now...
    stdout.split("\n")
}
