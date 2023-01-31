/* Many lines of un-idiomatic Rust
 */

use rand::Rng;
use std::io::Write;

use termcolor::{ColorChoice, StandardStream};

mod commands;

fn main() -> std::io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let mut vals: Vec<String> = vec![];
    let funcs = [commands::git_grep, commands::mypy];

    for func in funcs {
        match func() {
            Ok(vs) => vals.extend(vs),
            Err(_) => println!("log io error for git_gstrsrep"),
        }
    }

    let total_len = vals.len();

    let lucky_number = rand::thread_rng().gen_range(0..total_len);
    let line = &vals[lucky_number];
    writeln!(&mut stdout, "{}", line)?;

    Ok(())
}
