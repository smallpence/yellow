use crate::poke::ROM;
use std::io::{Result, stdout, Write, stdin};

const LINE_SIZE: usize = 64;
const ROM_LINE_COUNT: usize = 8;
const LINE_COUNT: usize = ROM_LINE_COUNT + 2;

pub struct ROMEditor<'a> {
    printed_count: usize,
    line: u32,
    rom: &'a ROM<'a>
}

impl<'a> ROMEditor<'a> {
    pub fn new(rom: &'a ROM) -> ROMEditor<'a> {
        ROMEditor {
            printed_count: 0,
            line: 0,
            rom
        }
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
}