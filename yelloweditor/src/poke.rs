use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead, Read, Write};
use std::io::Result;
use std::time::Duration;

const LINE_SIZE: usize = 64;
const LINE_COUNT: usize = 9;

pub struct ROM<'a> {
    src: &'a str,
    dict: CharDictionary,
    file: File,
    buff: Vec<u8>
}

impl<'a> ROM<'a> {
    pub fn new(src: &str) -> Result<ROM> {
        let mut dict = CharDictionary::new()?;
        let file = File::open(src)?;
        let mut buff:Vec<u8> = Vec::new();
        let mut reader = BufReader::new(&file);

        reader.read_to_end(&mut buff)?;

        Ok(ROM {
            src,
            dict,
            file,
            buff
        })
    }

    pub fn run(mut self) {
        use std::io::stdout;

        print!(".\n.\n.\n");

        for i in 0..10 {
            ROM::clear();

            println!("LINE {}",i);
            print!("{}",".\n".repeat(8));
            stdout().flush();

            std::thread::sleep(Duration::from_secs(1));
        }
    }

    fn clear() {
        print!("{}", "\x1B[1F".repeat(LINE_COUNT));
        let clear = format!("{}\r\n"," ".repeat(LINE_SIZE));
        print!("{}", clear.repeat(LINE_COUNT));
        print!("{}","\x1B[1F".repeat(LINE_COUNT));
        std::io::stdout().flush();
    }

    // fn print(&self) {
    //     let mut i:u8 = 0;
    //     for num in &self.buff {
    //         match self.dict.get(*num) {
    //             Some(entry) => print!("{}",entry),
    //             None => print!("{}",num)
    //         }
    //         i+=1;
    //         if i == LINE_SIZE { println!(""); i = 0; }
    //     }
    // }
}

struct CharDictionary {
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

    fn get(&self, i:u8) -> Option<String> {
        let mut s = String::new();
        let chars = self.dict.get(&i).unwrap();
        s.push(chars[0].into());
        s.push(chars[1].into());
        if s != ".." { Some(s) } else { None }
    }
}