use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufRead, Read, Result, BufWriter, Write};
use core::slice::Iter;

pub struct ROM<'a> {
    src: &'a str,
    pub dict: CharDictionary,
    file: File,
    buff: Vec<u8>,
}

impl<'a> ROM<'a> {
    pub fn new(src: &str) -> Result<ROM> {
        let dict = CharDictionary::new()?;
        let file = File::open(src)?;
        let mut buff:Vec<u8> = Vec::new();
        let mut reader = BufReader::new(&file);

        reader.read_to_end(&mut buff)?;

        Ok(ROM {
            src,
            dict,
            file,
            buff,
        })
    }

    pub fn size(&self) -> u32 {
        self.buff.len() as u32
    }

    pub fn iterator_from(&self, i: u32) -> Iter<u8> {
        let mut buff_iter = self.buff.iter();
        // skip through i elements in the iterator
        for _ in 0..i { buff_iter.next().unwrap(); }
        buff_iter
    }

    pub fn write_to_disk(&self, src: &String) -> Result<()> {

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(src)?;

        let mut writer = BufWriter::new(&file);
        writer.write_all(self.buff.as_slice());

        Ok(())
    }
}

pub struct CharDictionary {
    dict: HashMap<u8, [u8;2]>
}

impl CharDictionary {
    fn new() -> Result<CharDictionary> {
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
                if i < 255 { i = i + 1; }
            }
        }

        Ok(CharDictionary { dict })
    }

    pub fn get(&self, i:u8) -> Option<String> {
        let mut s = String::new();
        let chars = self.dict.get(&i).unwrap();
        s.push(chars[0].into());
        s.push(chars[1].into());
        if s != ".." { Some(s) } else { None }
    }
}