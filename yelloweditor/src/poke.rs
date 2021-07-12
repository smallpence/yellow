use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::io::Result;

pub struct CharDictionary {
    dict: HashMap<u8, [u8;2]>
}

impl CharDictionary {
    pub fn new() -> Result<CharDictionary> {
        let mut dict = HashMap::new();
        let f = File::open("dictionary.txt")?;
        let reader = BufReader::new(f);

        let mut i = 0;
        for line in reader.lines() {
            for chunk in line?.as_bytes().chunks(2) {
                // skip # chunk
                if chunk.ends_with(&[35]) { continue }
                dict.insert(i,[
                    chunk[0],
                    if chunk.len() == 2 { chunk[1] }
                    else { 32 }
                ]);
                println!("#{} {}",i, char::from(chunk[0]));
                if i < 255 { i = i + 1; }
            }
        }

        Ok(CharDictionary { dict })
    }

    pub fn get(&mut self, i:u8) -> Option<String> {
        let mut s = String::new();
        let chars = self.dict.get(&i).unwrap();
        s.push(chars[0].into());
        s.push(chars[1].into());
        if s != ".." { Some(s) } else { None }
    }
}