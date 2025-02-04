use std::{env, i32, usize};
use std::fs::read_to_string;

fn main() {
    let mut args = env::args();
    let my_name = args.next().unwrap();
    let mut lines_used: i32 = 10;

    for arg in args {
        if arg.starts_with("-"){
            let lines_used: i32 = arg[1..].parse();
        }else{
            let lines_used= 10;
        }
        match print_lines(arg.as_str(), lines_used.try_into().unwrap()) {
            Ok(_) => {}
            Err(e) => {
                println!("{my_name}: {arg}: {e}");
            }
        }
    }
}

fn print_lines(filename: &str, numlines: usize) -> std::io::Result<()> {
    let file_str = read_to_string(filename)?;
    println!("Contents of {filename}:");
    for (line_num, line) in file_str.lines().take(numlines).enumerate() {
        println!("{}: {}", line_num + 1, line);
    }
    Ok(())
}