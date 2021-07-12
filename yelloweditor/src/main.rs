use std::fs::File;
use std::io::{Write, BufReader, Read};
use std::io::Result;

fn main() -> Result<()> {
    let f = File::open("../yellow.gbc")?;
    let mut reader = BufReader::new(f);
    let mut buff:Vec<u8> = Vec::new();

    reader.read_to_end(&mut buff);

    let mut i:u8 = 0;
    for num in buff {
        print!("{} ", num);
        i+=1;
        if i == 8 { println!(""); i = 0; }
    }

    Ok(())
}
