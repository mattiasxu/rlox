use std::{fs, env, io::{self, stdout, Write, stdin}, process};
mod token;
mod scanner;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: jlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1])?;
        println!("{:?}", &args[1]);
    } else {
        run_prompt()?;
        println!("run prompt");
    }

    Ok(())
}

fn run_file(path: &str) -> io::Result<()> {
    let content = fs::read_to_string(path)?;
    run(&content);
    Ok(())
}

fn run_prompt() -> io::Result<()> {
    loop {
        let mut line = String::new();
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut line).expect("Failed to read line");

        run(&line);
        println!("You entered: {}", line.trim());
    }
}

fn run(source: &str) -> io::Result<()> {
    let scanner = Scanner::new(source);
    tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}

pub mod errors {
    pub fn error(line: usize, message: &str) {
        report(line, "", message);
    }

    fn report(line: usize, where_: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, where_, message);
    }
}
