mod ir;
mod jit;
mod lexer;
use std::{env, fs, process::ExitCode};

use lexer::Lexer;

fn main() -> ExitCode {
    let mut args = env::args();
    let prog = args.next().unwrap();
    let Some(input_path) = args.next() else {
        eprintln!("USAGE: {prog} <input.bf>");
        eprintln!("ERROR: require input file");
        return ExitCode::FAILURE;
    };
    let content = match fs::read(&input_path) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("ERROR: failed to read file {input_path}");
            eprintln!("       {err}");
            return ExitCode::FAILURE;
        }
    };

    let lexer = Lexer::new(&content);
    match ir::IR::new(lexer) {
        Ok(ires) => match ires.interprete() {
            Ok(_) => ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("ERROR: {err}");
                ExitCode::FAILURE
            }
        },
        Err(err) => {
            eprintln!("ERROR: {err}");

            ExitCode::FAILURE
        }
    }
}
