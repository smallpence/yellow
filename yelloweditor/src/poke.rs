use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead, Read, Write, stdout, stdin, Result};
use std::time::Duration;

const LINE_SIZE: usize = 64;
const ROM_LINE_COUNT: usize = 8;
const LINE_COUNT: usize = ROM_LINE_COUNT + 2;

pub struct ROM<'a> {
    src: &'a str,
    dict: CharDictionary,
    file: File,
    buff: Vec<u8>,
    printed_count: usize,
    line: u32
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
            buff,
            printed_count: 0,
            line: 0
        })
    }

    pub fn run(mut self) -> Result<()> {
        // dump a bunch of blank lines
        self.init();

        let mut command = String::new();

        // each branch MUST print 8 lines
        for i in 0..10 {
            self.clear();

            self.println(&format!("POKEMON YELLOW ROM EDITOR | LINE: {}",self.line));

            match command.to_lowercase().trim() {
                "help" => {
                    self.println(&String::from("hello world!"));
                }
                _ => ()
            }

            self.flush();
            command = self.command_prompt()?;
        }

        Ok(())
    }

    // wrapper for print as well for consistency
    fn print(&self, s: &String) {
        print!("{}",s);
    }

    // println except it also tracks printed lines
    fn println(&mut self, s: &String) {
        println!("{}",s);
        self.printed_count += 1;
    }

    fn init(&self) {
        print!("{}","\n".repeat(LINE_COUNT));
        print!("{}", "\x1B[1F".repeat(LINE_COUNT));
    }

    fn clear(&mut self) {
        // backtrack to start of gui using knowledge of printed lines
        print!("{}", "\x1B[1F".repeat(self.printed_count));

        // generated string that clears 1 line
        let clear = format!("{}\r\n"," ".repeat(LINE_SIZE));

        // print line_count amount of line clears
        print!("{}", clear.repeat(self.printed_count));

        // backtrack to start again
        print!("{}","\x1B[1F".repeat(self.printed_count));

        self.printed_count = 0;
    }

    // 1 line
    fn command_prompt(&mut self) -> Result<String> {
        print!(">>> ");
        self.flush();
        self.printed_count += 1;

        let mut s = String::new();
        stdin().read_line(&mut s)?;
        Ok(s)
    }

    #[inline]
    fn flush(&self) {
        stdout().flush().unwrap();
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