use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main(){
    let mut args: Vec<String> = env::args().skip(1).collect();
    let texttbf = args.remove(0); 
    for arg in args{
        match File::open(&arg) {
            Ok(file) => {
                let reader = BufReader::new(file);
                for line in reader.lines(){
                    if let Ok(line) = line{
                        if line.contains(&texttbf){
                            println!("{line}");
                        }
                    }
                }

            },
            Err(e) => {
                println!("Could not find {texttbf} in {arg}: {e}")
            },
        }
    }

}