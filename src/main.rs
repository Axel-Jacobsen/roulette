use std::io::Write;

use rand::Rng;
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};

mod commands;


fn get_output(funcs: &Vec<fn() -> std::io::Result<Vec<String>>>) -> Vec<String> {
    let mut vals: Vec<String> = vec![];

    // TODO run funcs concurrently?
    for func in funcs {
        match func() {
            Ok(vs) => vals.extend(vs),
            Err(_) => (),
        }
    }

    vals
}

fn main() -> std::io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // TODO filter funcs somehow!
    let funcs: Vec<fn() -> std::io::Result<Vec<String>>> =
        vec![commands::git_grep, commands::mypy, commands::ruff];

    let vals = get_output(&funcs);

    let total_len = vals.len();
    if total_len == 0 {
        writeln!(
            &mut stdout,
            "{}",
            "there were no issues found, the roulette wheel is missing!"
        )?;
        return Ok(());
    }

    let mut color_spec = ColorSpec::new();

    write!(&mut stdout, "Number of 'pockets': ")?;

    color_spec.set_bold(true);
    stdout.set_color(&color_spec)?;

    writeln!(&mut stdout, "{}", total_len)?;

    color_spec.clear();
    stdout.set_color(&color_spec)?;

    let lucky_number = rand::thread_rng().gen_range(0..total_len);

    let line = &vals[lucky_number];

    writeln!(&mut stdout, "{}", line)?;

    Ok(())
}
