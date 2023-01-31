use std::borrow::Cow;
use std::io;
use std::process::Command;
use std::process::Output;

use shell_escape::unix::escape;

fn synth_or(strs: Vec<String>) -> String {
    let or_expr = strs
        .into_iter()
        .fold("".to_string(), |cur, nxt| {
            let escaped_next = escape(Cow::Owned(nxt));
            cur + "|" + &escaped_next
        });

    format!("({})", &or_expr[1..])
}

fn convert_output_to_vec_of_strs(output: Output) -> Vec<String> {
    // TODO when will we get non-utf-8? Can we detect
    // the lang and change this accordingly?
    // output.stdout is Vec<u8>, so maybe we output
    // a Vec<u8> too?
    String::from_utf8(output.stdout)
        .expect("non-utf8 output from terminal")
        .split("\n")
        .map(String::from)
        .filter(|s| s != "")
        .collect()
}

pub fn git_grep() -> io::Result<Vec<String>> {
    // git grep vs. grep? Prefer git grep, else grep
    let strs = ["FIXME", "TODO"].map(String::from).to_vec();
    let grep_str = synth_or(strs);
    let command_output = Command::new("git")
        .arg("grep")
        .arg("--color=always")
        .arg("-niE")
        .arg(grep_str)
        .output()?;

    // assume stdout has valid utf8
    Ok(convert_output_to_vec_of_strs(command_output))
}

pub fn mypy() -> io::Result<Vec<String>> {
    let command_output = Command::new("mypy")
        .arg(".")
        .arg("--no-error-summary")
        .output()?;

    Ok(convert_output_to_vec_of_strs(command_output))
}

#[cfg(test)]
mod main_tests {
    use crate::commands::synth_or;

    #[test]
    fn basic_symbol_freq() {
        let vs = ["FIXME", "TODO", "test", ":)"].map(String::from).to_vec();
        let res = synth_or(vs);
        assert_eq!(res, "(FIXME|TODO|test|':)')");
    }
}
