

pub(crate) mod result;

use std::fs::File;
use std::io::{self,Read};


fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;  // ? operator propagates Err early
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
fn main() {
    // let vec = vec![1, 2, 3];
    // let _ = vec[10];  // This will panic: index out of bounds

  match read_file("everest.txt"){
    Ok(content)=>println!("File content:{}",content),
    Err(e)=>println!("Error :{}",e)
  }
}


fn read_file!(path:&)