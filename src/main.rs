use std::io::Write;

use rand::Rng;
use termcolor::{WriteColor, ColorSpec, ColorChoice, StandardStream};

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

    let mut color_spec = ColorSpec::new();

    write!(&mut stdout, "Number of 'pockets': ")?;

    color_spec.set_bold(true);
    stdout.set_color(&color_spec)?;

    let total_len = vals.len();
    writeln!(&mut stdout, "{}", total_len)?;

    color_spec.clear();
    stdout.set_color(&color_spec)?;

    let lucky_number = rand::thread_rng().gen_range(0..total_len);
    let line = &vals[lucky_number];
    writeln!(&mut stdout, "{}", line)?;

    Ok(())
}
