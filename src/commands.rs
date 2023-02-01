use std::borrow::Cow;
use std::io;
use std::process::Command;
use std::process::Output;

use regex::Regex;
use shell_escape::unix::escape;

fn synth_or(strs: Vec<String>) -> String {
    let or_expr = strs.into_iter().fold("".to_string(), |cur, nxt| {
        let escaped_next = escape(Cow::Owned(nxt));
        cur + "|" + &escaped_next
    });

    format!("({})", &or_expr[1..])
}

fn convert_output_to_vec_of_strs(output: Output) -> Vec<String> {
    /* Converts Command Output to vector of strings, splitting output by newlines
     *
     * This should handle Output's Vec<u8> to a string type. Right now we assume
     * all output is utf-8, but I don't think that this is necessarily true.
     * On *nix, I *think* we can rely on $LANG.
     */
    // TODO when will we get non-utf-8? Can we detect
    // the lang and change this accordingly?
    // output.stdout is Vec<u8>, so maybe we output
    // a Vec<u8> too?
    // TODO return an iterable so we can do further operations
    // on the output. If no further operations are needed, the
    // calling function can `collect`. Is this a good idea?
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

    // e.g.
    // path/to/file.rs:107: error: blah blah
    // path/to/file.rs:107: note: friend is a four letter word
    //
    // We want to keep the 'error' lines, but get rid of the 'note' lines
    // TODO add option for `--pretty` cause prettier is better

    let mypy_line_output_regex =
        Regex::new(r"(?P<file_and_line>/?[a-zA-Z0-9_\-\./]+:\d+:) (?P<mypy_type>error|note):")
            .expect("invalid regex!");

    Ok(convert_output_to_vec_of_strs(command_output)
        .into_iter()
        .filter(|line| {
            let captures = match mypy_line_output_regex.captures(&line) {
                Some(c) => c,
                None => return false, // TODO add 'DEBUG' option to program and log this case!
            };
            &captures["mypy_type"] == "error"
        })
        .collect())
}

pub fn ruff() -> io::Result<Vec<String>> {
    let command_output = Command::new("ruff").arg(".").arg("-q").output()?;
    Ok(convert_output_to_vec_of_strs(command_output))
}

pub fn flake8() -> io::Result<Vec<String>> {
    let command_output = Command::new("flake8").arg(".").output()?;
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
