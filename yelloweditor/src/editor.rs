use crate::poke::ROM;
use std::io::{Result, stdout, Write, stdin};

const LINE_SIZE: usize = 200;
const ROM_LINE_COUNT: usize = 8;
const LINE_COUNT: usize = ROM_LINE_COUNT + 3;
const PRINT_INTERVAL: u32 = 64;

pub struct ROMEditor<'a> {
    printed_count: usize,
    line: u32,
    rom: &'a ROM<'a>,
    rom_size_length: usize
}

impl<'a> ROMEditor<'a> {
    pub fn new(rom: &'a ROM) -> ROMEditor<'a> {
        ROMEditor {
            printed_count: 0,
            line: 0,
            rom,
            rom_size_length: format!("{:x}",rom.size()).len()
        }
    }

    pub fn run(mut self) -> Result<()> {
        // dump a bunch of blank lines
        self.init();

        let mut command = String::new();


        loop {
            // whether to prompt for a new command after this iteration, e.g. if user has already inputted
            let mut skip_prompt = false;

            if command.eq("quit") { break }

            self.clear();

            self.println(&format!("POKEMON YELLOW ROM EDITOR"));

            match command.as_str() {
                "d" => {
                    self.line += PRINT_INTERVAL * ROM_LINE_COUNT as u32;
                    self.print_rom();
                }

                "u" => {
                    self.line -= PRINT_INTERVAL * ROM_LINE_COUNT as u32;
                    self.print_rom();
                }

                "goto" => {
                    self.println(&String::from("where to?"));
                    self.print(&String::from("0x"));
                    self.flush();

                    let result = i32::from_str_radix(self.stdin().to_lowercase().trim(),16);
                    match result {
                        Ok(i) => {
                            self.line = ((i as u32) / PRINT_INTERVAL) * PRINT_INTERVAL;
                            skip_prompt = true;
                        }
                        Err(_) => self.println(&String::from("bad hex"))
                    }
                }

                "size" => {
                    self.println(&format!("rom size: {:#x}", self.rom.size()))
                }

                _ => {
                    self.print_rom();
                }
            }

            self.flush();

            command = match skip_prompt {
                true =>  String::new(),
                false => self.command_prompt().trim().to_lowercase(),
            }
        }

        Ok(())
    }

    fn print_rom(&mut self) {
        // begin printing in bold
        self.print(&String::from("\x1B[1m"));

        // print the hex from 0-PRINT_INTERVAL as well as a gap
        let t: String = (0..PRINT_INTERVAL).map(|x| format!("{:02x} ", x)).collect();
        self.println(&format!("{} {}", " ".repeat(self.rom_size_length), t));

        // no more bold
        self.print(&String::from("\x1B[0m"));

        let mut rom_iter = self.rom.iterator_from(self.line);
        // current line on the display
        let mut display_line = self.line;
        for _ in 0..ROM_LINE_COUNT {
            let mut line_hex = String::new();
            for _ in 0..PRINT_INTERVAL {
                let next = rom_iter.next().unwrap();
                let next = match self.rom.dict.get(*next) {
                    Some(chars) => format!("{:} ", chars),
                    None => format!("{:02x} ", next).to_uppercase()
                };
                line_hex.push_str(next.as_str());
            }
            self.println(&format!("{:01$x} {2}", display_line, self.rom_size_length, line_hex));
            display_line += PRINT_INTERVAL;
        }
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

    fn stdin(&mut self) -> String {
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        self.printed_count += 1;
        s
    }

    fn init(&self) {
        // +1 since pressing enter makes an extra extra line
        print!("{}","\n".repeat(LINE_COUNT+1));
        print!("{}", "\x1B[1F".repeat(LINE_COUNT+1));
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
    fn command_prompt(&mut self) -> String {
        print!(">>> ");
        self.flush();
        self.printed_count += 1;

        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        s
    }

    #[inline]
    fn flush(&self) {
        // flush should always work, else very bad
        stdout().flush().unwrap();
    }
}