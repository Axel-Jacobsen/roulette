/* Many lines of un-idiomatic Rust
 */

mod commands;

use std::io::Write;
use termcolor::{ColorChoice, StandardStream};


fn main() -> std::io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let vs = ["FIXME", "TODO"].map(String::from).to_vec();

    match commands::git_grep(vs) {
        Ok(vals) => {
            for line in vals {
                writeln!(&mut stdout,"{}", line)?;
            }
        }
        Err(_) => println!("log io error for git_grep"),
    }

    Ok(())
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
