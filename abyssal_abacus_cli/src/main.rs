//! Terminal front end. Two modes:
//!
//!   abyssal_abacus_cli              -> interactive REPL (type expressions, 'exit' to quit)
//!   abyssal_abacus_cli "3 + 4"       -> one-shot: evaluate and print, then exit
//!
//! Both modes call into `abyssal_abacus_core`, the same crate the GUI uses.

use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if !args.is_empty() {
        run_one_shot(&args.join(" "));
        return;
    }

    run_repl();
}

fn run_one_shot(expr: &str) {
    match abyssal_abacus_core::evaluate_expr(expr) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_repl() {
    println!("=== Abyssal Abacus (CLI) ===");
    println!("Type an expression like 3+4, or 'exit' to quit.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input).unwrap();
        if bytes_read == 0 {
            // EOF, e.g. Ctrl-D
            println!();
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            println!("Goodbye!");
            break;
        }

        match abyssal_abacus_core::evaluate_expr(input) {
            Ok(result) => println!("{}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
