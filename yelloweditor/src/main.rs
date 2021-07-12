mod poke;

use std::fs::File;
use std::io::{BufReader, Read};
use std::io::Result;

fn main() -> Result<()> {
    const LINE_SIZE: u8 = 128;

    let mut poke_dictionary = poke::CharDictionary::new()?;
    let f = File::open("../yellow.gbc")?;
    let mut reader = BufReader::new(f);
    let mut buff:Vec<u8> = Vec::new();

    reader.read_to_end(&mut buff)?;

    let mut i:u8 = 0;
    for num in buff {
        match poke_dictionary.get(num) {
            Some(entry) => print!("{}",entry),
            None => print!("{}",num)
        }
        i+=1;
        if i == LINE_SIZE { println!(""); i = 0; }
    }

    Ok(())
}
