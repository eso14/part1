use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut show_words = true;
    let mut show_chars = true;
    let mut show_lines= true;

    let my_name = env::args().next().unwrap();
    for arg in args {
        if arg.starts_with("-"){
            show_words = arg.contains("w");
            show_lines = arg.contains("l");
            show_chars = arg.contains("c");
        }
      
        match print_line_nums(arg.as_str(), show_words, show_lines, show_chars) {
            Ok(_) => {}
            Err(e) => {
                println!("{my_name}: {arg}: {e}");
            }
        }
    }
}

fn print_line_nums(filename: &str, show_words:bool, show_lines:bool, show_chars:bool) -> std::io::Result<()> {
    let file_str = read_to_string(filename)?;
    let charint:i32 = file_str.len().try_into().unwrap();
    let  lineint = file_str.lines().count();
    let  wordint = file_str.split_whitespace().count();
    if show_words {println!("Words: {wordint}")};
    if show_lines {println!("Lines:{lineint}")};
    if show_chars{println!("Chars: {charint}")};
    //println!("Contents of {filename}: w {wordint}, l{lineint}, c{charint}");
    /*for (line_num, line) in file_str.lines().enumerate() {
        println!("{}: {}", line_num + 1, line);

    }*/
    Ok(())
}