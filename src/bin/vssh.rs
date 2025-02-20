use std::{env, io, process};
use std::ffi::CString;
use std::io::{stdin, Write};
use std::path::Path;
use std::process::{exit, ExitCode};
use nix::unistd::{chdir, execvp, fork, getpid, ForkResult};
use nix::sys::wait::waitpid;

fn main() {
    loop {
        print!("{}$ ", env::current_dir().unwrap().display());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        if input == "exit" {
            break;
        } else if input.starts_with("cd ") {
            if let Err(e) = chdir(Path::new(&input[3..])) {
                println!("cd error: {}", e);
            }
            continue;
        }
        
        let mut argv = Vec::new();
        for arg in [input, "src/bin/vssh.rs"] {
        argv.push(CString::new(arg).unwrap());
        }
        if argv.is_empty() {
            continue;
        }

        match unsafe { fork() } {
            Err(e) => eprintln!("Fork failed: {}", e),
            Ok(fork_value) => match fork_value {
                ForkResult::Parent { child } => {
                    match waitpid(child, None) {
                        Ok(status) => {
                            println!("parent of {child} ({}) status: {status:?}", getpid());
                        }
                        Err(e) => {
                            println!("I can't wait! {e}");
                        }
                    }
                }
                ForkResult::Child => {
                    if let Err(e) = execvp(&argv[0], &argv) {
                        println!("Error executing {}: {}", input, e);
                        process::exit(1);
                    }
                }
            }
        }
    }
}
