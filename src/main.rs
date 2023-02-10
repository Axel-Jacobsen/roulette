use std::collections::HashMap;
use std::io::Write;

use clap::Parser;
use rand::Rng;
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};

mod cli;
mod commands;

fn process_commands(
    cli: &cli::Cli,
    funcs: HashMap<String, commands::TypeCommand>,
) -> Result<Vec<String>, String> {
    let mut vals: Vec<String> = vec![];

    let command_list: Vec<String> = match &cli.commands {
        Some(cs) => cs.clone(),
        None => vec!["git_grep".to_string(), "mypy".to_string()],
    };

    // TODO run funcs concurrently?
    for func in command_list {
        match funcs[&func](cli) {
            Ok(vs) => vals.extend(vs),
            Err(e) => {
                if cli.debug {
                    println!("command failed: {e:?}");
                }
            }
        }
    }

    Ok(vals)
}

fn process_command_outputs(cli: &cli::Cli, vals: Vec<String>) -> std::io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let total_len = vals.len();

    if total_len == 0 {
        writeln!(
            &mut stdout,
            "there were no issues found, the roulette wheel is missing!"
        )?;
        return Ok(());
    }

    let mut color_spec = ColorSpec::new();

    write!(&mut stdout, "Number of 'pockets': ")?;

    color_spec.set_bold(true);
    stdout.set_color(&color_spec)?;

    writeln!(&mut stdout, "{total_len}")?;

    color_spec.clear();
    stdout.set_color(&color_spec)?;

    if cli.all {
        for line in vals {
            writeln!(&mut stdout, "{line}")?;
        }
    } else {
        let lucky_number = rand::thread_rng().gen_range(0..total_len);
        let line = &vals[lucky_number];
        writeln!(&mut stdout, "{line}")?;
    }

    color_spec.clear();
    stdout.set_color(&color_spec)?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut funcs: HashMap<String, commands::TypeCommand> = HashMap::new();
    funcs.insert("git_grep".to_string(), commands::git_grep);
    funcs.insert("rip_grep".to_string(), commands::rip_grep);
    funcs.insert("grep".to_string(), commands::grep);
    funcs.insert("mypy".to_string(), commands::mypy);
    funcs.insert("ruff".to_string(), commands::ruff);
    funcs.insert("flake8".to_string(), commands::flake8);
    funcs.insert("clippy".to_string(), commands::cargo_clippy);

    let cli = cli::Cli::parse();

    if cli.supported {
        for k in funcs.keys() {
            print!("{k} ");
        }
        println!();
        return Ok(());
    }

    if let Some(path) = &cli.path {
        if !path.is_dir() {
            let path_display = path.display();
            let path_str = format!("path {path_display} doesn't exist, or it isn't a directory");
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, path_str));
        }
    }

    let vals = process_commands(&cli, funcs).unwrap();

    process_command_outputs(&cli, vals)
}
