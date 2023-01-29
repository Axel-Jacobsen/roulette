/* Many lines of un-idiomatic Rust
 */

use std::io;
use std::borrow::Cow;
use std::process::Output;
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

fn git_grep(strs: Vec<String>) -> io::Result<Output> {
    let grep_str = synth_or(strs);
    Command::new("git").arg("grep").arg("-niE").arg(grep_str).output()
}

fn main() {
    let vs = ["FIXME", "TODO"].map(String::from).to_vec();
    let res = git_grep(vs);
    println!("{:?}", res);
}

#[cfg(test)]
mod main_tests {
    use crate::synth_or;

    #[test]
    fn basic_symbol_freq() {
        let vs = ["FIXME", "TODO", "test", ":)"].map(String::from).to_vec();
        let res = synth_or(vs);
        assert_eq!(res, "(FIXME|TODO|test|':)')");
    }
}
