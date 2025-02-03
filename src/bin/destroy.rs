use std::env;
use std::fs::remove_file;

fn main(){
    let args: Vec<String> = env::args().skip(1).collect();
    for arg in args{
        match remove_file(&arg){
            Ok(_) => println!("Succesfully Deleted: {}", arg),
            Err(e)=> println!("Could not delete {} because{}", arg, e),

        }
    }
}