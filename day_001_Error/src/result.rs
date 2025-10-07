use std::{fs::File, io::{self, Read}, task::Context};

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents); 
    Ok(contents)

}

fn main(){
    match  read_file("fdkfj.txt") {
       Ok(content) => println!("File contents: {}", content),
        Err(e) => println!("Error: {}", e),
        
    }
}
