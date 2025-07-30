use std::io;
use std::fs;
use std::process;
fn main() {
    match write_to_file(){
        Ok(file_name) => println!("Successfully wrote to file {file_name}"),
        Err(error) => {
            eprintln!("There was an error {error}");
            process::exit(1);
        }
    };
}

fn write_to_file()->io::Result<String>{

   let input= io::stdin();

   println!("What file would you like to write to?");
   let mut file_name = String::new();
   input.read_line(&mut file_name)?;
   
   println!("What would you like to write to the file?");
   let mut content = String::new();
   input.read_line(&mut content)?;
   
   fs::write(file_name.trim(), content.trim())?;

   Ok(file_name)

}
