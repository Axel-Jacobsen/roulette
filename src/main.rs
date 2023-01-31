use rand::Rng;
use std::io::Write;
use termcolor::{Color, WriteColor, ColorSpec, ColorChoice, StandardStream};

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

    let mut color_spec = ColorSpec::new();
    color_spec.set_fg(Some(Color::Blue));
    stdout.set_color(&color_spec)?;

    write!(&mut stdout, "Number of 'pockets': ")?;

    stdout.set_color(ColorSpec::new().set_bold(true))?;

    writeln!(&mut stdout, "{}", total_len)?;

    color_spec.clear();
    stdout.set_color(&color_spec)?;

    let lucky_number = rand::thread_rng().gen_range(0..total_len);
    let line = &vals[lucky_number];
    writeln!(&mut stdout, "{}", line)?;

    Ok(())
}
