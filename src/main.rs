use std::path::PathBuf;

use clap::Parser;
use ini::Ini;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Origional File for changes to be applied to
    origin: PathBuf,
    /// File output
    output: PathBuf,
    /// File to source changes from
    modifiers: Vec<PathBuf>,
}

const PARSE_OPTION: ini::ParseOption = ini::ParseOption {
    enabled_quote: true,
    enabled_escape: false,
};

fn main() {
    let args = Args::parse();
    println!("Source File: {:?}", args.origin);
    for modifier in &args.modifiers {
        println!("Modifier Input: {:?}", modifier);
    }
    println!("Output: {:?}", args.output);

    println!("Reading source file: {:?}", args.origin);
    let mut ini =
        Ini::load_from_file_opt(args.origin, PARSE_OPTION).expect("Failed to read original config");
    let modifiers = args.modifiers.into_iter().map(|f| {
        let msg = format!("Failed to read modifier {:?}", &f);
        println!("Reading modifier: {:?}", f);
        Ini::load_from_file_opt(f, PARSE_OPTION).expect(&msg)
    });
    println!("Merging configs");
    for modifier in modifiers {
        for (sec, prop) in modifier {
            for (k, v) in prop.iter() {
                ini.with_section(sec.clone()).set(k, v);
            }
        }
    }
    println!("Saving result to {:?}", args.output);
    ini.write_to_file_policy(args.output, ini::EscapePolicy::Basics)
        .expect("Failed to write file")
}
