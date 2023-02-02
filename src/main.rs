use std::collections::HashMap;
use std::io::Write;

use clap::Parser;
use rand::Rng;
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};

mod commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Commands to run (any of grep, mypy, ruff, flake8) - defaults to all of them
    #[arg(short, long, num_args = 0..)]
    commands: Option<Vec<String>>,
    /// Optional keywords for grep: defaults to "TODO" and "FIXME"
    #[arg(short, long, num_args = 0..)]
    grep_keywords: Option<Vec<String>>,
    /// Supported Commands
    #[arg(short, long, action=clap::ArgAction::SetTrue)]
    supported_commands: bool
}


fn process_commands(args: Cli, funcs: HashMap<String, commands::TypeCommand>) -> Result<Vec<String>, String> {
    let mut vals: Vec<String> = vec![];

    let command_list = match args.commands {
        Some(command_list) => command_list,
        None => funcs.keys().cloned().collect()
    };

    for func in command_list.iter() {
        if !funcs.contains_key(func) {
            return Err(format!("func {} not in supported commands", func));
        }
    }

    // TODO run funcs concurrently?
    for func in command_list {
        match funcs[&func](None) {
            Ok(vs) => vals.extend(vs),
            Err(_) => {
                // TODO deal w/ this error case
                ()
            },
        }
    }


    Ok(vals)
}

fn process_command_outputs(vals: Vec<String>) -> std::io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

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

fn main() -> std::io::Result<()> {
    let mut funcs: HashMap<String, commands::TypeCommand> = HashMap::new();
    funcs.insert("grep".to_string(), commands::git_grep);
    funcs.insert("mypy".to_string(), commands::mypy);
    funcs.insert("ruff".to_string(), commands::ruff);
    funcs.insert("flake8".to_string(), commands::flake8);

    let cli = Cli::parse();

    if cli.supported_commands {
        for k in funcs.keys() { print!("{} ", k); }
        println!("");
        return Ok(());
    }

    let vals = process_commands(cli, funcs).unwrap();

    process_command_outputs(vals)
}
