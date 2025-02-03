use std::env;
use std::fs::rename;

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() != 3{
        println!("Usage: newname <oldname> <newname>")
    }


    if let Err(e)= rename(&args[1], &args[2]){
        eprintln!("Could not rename file {}: {}", &args[1], e);

    }
}