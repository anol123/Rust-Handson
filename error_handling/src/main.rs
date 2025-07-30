use std::fs::*;
use std::process;

fn main() {
   let result = File::open("story.txt");

   let result_content = match result{
    Ok(sample)=> sample,
    Err(error) => {
        eprintln!("Op failed {error}");
       process::exit(1) 
    }
   };

   println!("Result cont i.e. file : {:#?}",result_content);
}
