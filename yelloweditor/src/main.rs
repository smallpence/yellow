mod poke;

use std::fs::File;
use std::io::{BufReader, Read};
use std::io::Result;

fn main() -> Result<()> {
    let rom = poke::ROM::new("../yellow.gbc")?;

    rom.print();

    Ok(())
}
