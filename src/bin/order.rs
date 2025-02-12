use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main(){
    let args: Vec<String> = env::args().skip(1).collect();
    let mut reverse = false;
    let mut files = Vec::new();

    for arg in &args{
        if arg == "-r"{
            reverse = true;
        }else{
            files.push(arg.clone());
        }
    }

    let mut lines = Vec::new();
    for file in files{
        match File::open(&file){
            Ok(file) => {
                let reader = BufReader::new(file);
                for line in reader.lines(){
                    if let Ok(line) = line{
                        lines.push(line);
                    }
                }

            },
            Err(e) => {
                println!("Error : {e}")
            },
        }
        
    }
    if reverse{
        lines.sort_by(|a,b| b.cmp(a));
    }else{
        lines.sort();
    }

    for line in lines{
        println!("{line}");
    }

    



}