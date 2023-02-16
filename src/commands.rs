use std::borrow::Cow;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;

use regex::Regex;
use shell_escape::unix::escape;

use crate::cli;

fn synth_or(strs: &[String]) -> String {
    let or_expr = strs.iter().fold("".to_string(), |cur, nxt| {
        let escaped_next = escape(Cow::Owned(nxt.clone()));
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
        .split('\n')
        .map(String::from)
        .filter(|s| !s.is_empty())
        .collect()
}

// Not sure if this is the most "rustonic" way to do this!
pub type TypeCommand = fn(&cli::Cli) -> io::Result<Vec<String>>;

pub fn git_grep(cli: &cli::Cli) -> io::Result<Vec<String>> {
    // git grep vs. grep? Prefer git grep, else grep
    let strs = ["FIXME", "TODO"].map(String::from).to_vec();
    let grep_str = synth_or(&cli.grep_keywords.clone().unwrap_or(strs));

    let command_output = Command::new("git")
        .arg("grep")
        .arg("--color=always")
        .arg("-niE")
        .arg(grep_str)
        .output()?;

    Ok(convert_output_to_vec_of_strs(command_output))
}

pub fn rip_grep(cli: &cli::Cli) -> io::Result<Vec<String>> {
    let strs = ["FIXME", "TODO"].map(String::from).to_vec();
    let grep_str = synth_or(&cli.grep_keywords.clone().unwrap_or(strs));

    let binding = cli.path.clone().unwrap_or(PathBuf::from("."));
    let path = binding.to_str().expect("invalid path");

    let command_output = Command::new("rg")
        .arg("--no-heading")
        .arg(path)
        .arg("-e")
        .arg(grep_str)
        .output()?;

    Ok(convert_output_to_vec_of_strs(command_output))
}

pub fn grep(cli: &cli::Cli) -> io::Result<Vec<String>> {
    let strs = ["FIXME", "TODO"].map(String::from).to_vec();
    let grep_str = synth_or(&cli.grep_keywords.clone().unwrap_or(strs));

    let binding = cli.path.clone().unwrap_or(PathBuf::from("."));
    let path = binding.to_str().expect("invalid path");

    let command_output = Command::new("grep")
        .arg("-rnw")
        .arg(path)
        .arg("-e")
        .arg(grep_str)
        .output()?;

    Ok(convert_output_to_vec_of_strs(command_output))
}

pub fn mypy(cli: &cli::Cli) -> io::Result<Vec<String>> {
    let binding = cli.path.clone().unwrap_or(PathBuf::from("."));
    let path = binding.to_str().expect("invalid path");

    let command_output = Command::new("mypy")
        .arg(path)
        .arg("--no-error-summary")
        .arg("--color-output")
        .arg("--pretty")
        .output()?;

    let stdout_str =
        String::from_utf8(command_output.stdout).expect("non-utf8 output from terminal");

    // e.g.
    // path/to/file.rs:107: error: blah blah blah not defined
    // path/to/file.rs:107: note: what is a note but a thought in time?
    //                     blah blah blah
    //                     ^
    //
    // We want to collect the error, note, explanation, and error marker into one 'unit'.
    let mypy_line_output_regex =
        Regex::new(r"(?P<file_and_line>/?[a-zA-Z0-9_\-\./ ]+:\d+:) (?P<mypy_type>error|note):.*")
            .expect("invalid regex!");

    let mut current_line: String = String::new();
    let mut collected_lines: Vec<String> = vec![];

    for line in stdout_str.split('\n').map(String::from) {
        match mypy_line_output_regex.captures(&line) {
            Some(c) => {
                if &c["mypy_type"] == "error" {
                    collected_lines.push(current_line.clone());
                    current_line = line;
                } else {
                    current_line = [current_line, line].join("\n")
                }
            }
            None => current_line = [current_line, line].join("\n"),
        }
    }

    // TODO maybe this is horribly inefficient?
    Ok(collected_lines[1..].to_vec())
}

pub fn ruff(cli: &cli::Cli) -> io::Result<Vec<String>> {
    let binding = cli.path.clone().unwrap_or(PathBuf::from("."));
    let path = binding.to_str().expect("invalid path");

    let command_output = Command::new("ruff").arg(path).arg("-q").output()?;
    Ok(convert_output_to_vec_of_strs(command_output))
}

pub fn flake8(cli: &cli::Cli) -> io::Result<Vec<String>> {
    let binding = cli.path.clone().unwrap_or(PathBuf::from("."));
    let path = binding.to_str().expect("invalid path");

    let command_output = Command::new("flake8").arg(path).output()?;
    Ok(convert_output_to_vec_of_strs(command_output))
}

pub fn cargo_clippy(_: &cli::Cli) -> io::Result<Vec<String>> {
    let command_output = Command::new("cargo")
        .arg("clippy")
        .arg("--color")
        .arg("always")
        .output()?;

    let stderr_str =
        String::from_utf8(command_output.stderr).expect("non-utf8 output from terminal");

    let warnings_or_errors_regex =
        Regex::new(r"(?P<issue>/?(?:warning|error).*)\n\n").expect("invalid regex!");

    let warnings_and_errors: Vec<String> = stderr_str
        .split("\n\n")
        .map(String::from)
        .filter(|line| {
            // TODO add 'DEBUG' option to program and log this case!
            warnings_or_errors_regex.captures(line).is_none()
        })
        .collect();

    // split_last returns Result<last, rest>, and we just want the rest, hence `.1`
    let the_good_stuff = warnings_and_errors.split_last().unwrap().1;

    Ok(the_good_stuff.to_vec())
}

#[cfg(test)]
mod main_tests {
    use crate::commands::synth_or;

    #[test]
    fn basic_keyword_escaping() {
        let vs = ["FIXME", "TODO", "test", ":)"].map(String::from).to_vec();
        let res = synth_or(vs);
        assert_eq!(res, "(FIXME|TODO|test|':)')");
    }
}
