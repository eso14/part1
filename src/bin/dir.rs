use std::fs::read_dir;

fn main(){
    let files = read_dir(".").unwrap();
    for file in files{
        let file = file.unwrap();
        println!("{}", file.file_name().to_string_lossy());
    }
}

