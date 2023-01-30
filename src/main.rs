/* Many lines of un-idiomatic Rust
 */

use rand::Rng;
use std::io::Write;

use termcolor::{ColorChoice, StandardStream};

mod commands;

fn main() -> std::io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let vs = ["FIXME", "TODO"].map(String::from).to_vec();

    match commands::git_grep(vs) {
        Ok(vals) => {
            let lucky_number = rand::thread_rng().gen_range(0..vals.len());
            let line = &vals[lucky_number];
            writeln!(&mut stdout, "{}", line)?;
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
