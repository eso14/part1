use std::env;
use std::fs::read_to_string;

fn main() {
    let mut args = env::args();
    let mut lines_used = 10;
    let my_name = args.next().unwrap();
    for arg in args {
        match print_lines(arg.as_str()) {
            Ok(_) => {}
            Err(e) => {
                println!("{my_name}: {arg}: {e}");
            }
        }
    }
}

fn print_lines(filename: &str) -> std::io::Result<()> {
    let file_str = read_to_string(filename)?;
    println!("Contents of {filename}:");
    for (line_num, line) in file_str.lines().enumerate() {
        println!("{}: {}", line_num + 1, line);
    }
    Ok(())
}