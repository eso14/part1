use std::{env, io, process};
use std::ffi::CString;
use std::io::{stdin, Write};
use std::path::Path;
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{chdir, dup2, execvp, fork, ForkResult};
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

        let mut parts: Vec<&str> = input.split_whitespace().collect();
        let mut background = false;
        let mut input_file = None;
        let mut output_file = None;

        if parts.last() == Some(&"&"){
            background = true;
            parts.pop();
        }

        let mut i = 0;
        while i< parts.len(){
            if parts[i] == "<" && i + 1 < parts.len(){
                input_file = Some(parts.remove(i+1));
                parts.remove(i);
            }else if parts[i] == ">" && i+1 < parts.len(){
                output_file = Some(parts.remove(i+1));
                parts.remove(i);
            }else{
                i +=1;
            }
        }


        
       

        match unsafe { fork() } {
            Err(e) => eprintln!("Fork failed: {}", e),
            Ok(fork_value) => match fork_value {
                ForkResult::Parent { child } => {
                    if !background{
                        let _ = waitpid(child, None);
                    }else{
                        println!("Background process started: {}", child);

                    }
                    
                }
                ForkResult::Child => {
                    let mut argv = Vec::new();
                    for arg in [input, "src/bin/vssh.rs"] {
                    argv.push(CString::new(arg).unwrap());
                    }
                    if argv.is_empty() {
                        continue;
                    }
                    if let Err(e) = execvp(&argv[0], &argv) {
                        println!("Error executing {}: {}", input, e);
                        process::exit(1);
                    }
                    if let Some(file) = input_file {
                        if let Ok(fd) = open(file, nix::fcntl::OFlag::O_RDONLY, Mode::empty()) {
                            let _ = dup2(fd, 0);
                        } else {
                            eprintln!("Failed to open input file");
                            process::exit(1);
                        }
                    }
                    if let Some(file) = output_file {
                        if let Ok(fd) = open(file, OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC, Mode::S_IRUSR | Mode::S_IWUSR) {
                            let _ = dup2(fd, 1);
                        } else {
                            eprintln!("Failed to open output file");
                            process::exit(1);
                        }
                    }
                    
                }
            }
        }
    }
}
