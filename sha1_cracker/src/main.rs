use hex;
use sha1::{Digest, Sha1};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const SHA1_HASH_LENGTH: usize = 40;
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprint!("args:<wordllist.txt> <SHA1_HASH>");
        return Ok(());
    }
    let file_handler = File::open(&args[1])?;
    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HASH_LENGTH {
        return Err("Invalid Hash".into());
    }

    let buffer = BufReader::new(&file_handler);

    for lines in buffer.lines() {
        let line = lines?.trim().to_string();
        if  hash_to_crack == &hex::encode(Sha1::digest(line.as_bytes())){
          println!("hashfound :{line}");
          return Ok(());
        }
        }
    
    println!("not found");

    Ok(())
}
