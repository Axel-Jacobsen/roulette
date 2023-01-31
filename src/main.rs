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
