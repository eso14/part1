use std::env;
use std::fs::copy;

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() != 3{
        eprintln!("Usage: duplicate <filetocopy> <copiedfilename>")
    }


    if let Err(e)= copy(&args[1], &args[2]){
        eprintln!("Could not copy file {}: {}", &args[1], e);

    }
}