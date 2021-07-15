use crate::rom::{ROM, RomSize};
use std::io::{Result as IOResult, stdout, Write, stdin, BufRead};
use std::num::ParseIntError;
use std::str::{from_utf8};

const LINE_SIZE: usize = 200;
const ROM_LINE_COUNT: usize = 8;
const LINE_COUNT: usize = ROM_LINE_COUNT + 3;
const PRINT_INTERVAL: RomSize = 64;

pub struct ROMEditor {
    printed_count: usize,
    line: RomSize,
    rom: ROM,
    rom_size_length: usize,
    raw: bool
}

impl ROMEditor {
    pub fn new(rom: ROM) -> ROMEditor {
        ROMEditor {
            printed_count: 0,
            line: 0,
            rom_size_length: format!("{:x}",(&rom).size()).len(),
            rom,
            raw: false
        }
    }

    pub fn run(mut self) -> IOResult<()> {
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
                    self.line += PRINT_INTERVAL * ROM_LINE_COUNT as RomSize;
                    self.print_rom(0);
                }

                "u" => {
                    self.line -= PRINT_INTERVAL * ROM_LINE_COUNT as RomSize;
                    self.print_rom(0);
                }

                "raw" => { self.raw = true;  skip_prompt = true; }
                "eng" => { self.raw = false; skip_prompt = true; }

                "goto" => {
                    self.println_str("where to?");
                    self.print_str("0x");
                    self.flush();

                    let result = i32::from_str_radix(self.input_from_stdin().to_lowercase().trim(), 16);
                    match result {
                        Ok(i) => {
                            self.line = ((i as RomSize) / PRINT_INTERVAL) * PRINT_INTERVAL;
                            skip_prompt = true;
                        }
                        Err(_) => self.println_str("bad hex")
                    }
                }

                "size" => {
                    self.println(&format!("rom size: {:#x}", self.rom.size()))
                }

                "save" => {
                    self.println_str("where to?");
                    self.print_str("src: ");
                    self.flush();

                    let src = self.input_from_stdin().trim().to_string();

                    match self.rom.write_to_disk(&src) {
                        Ok(_)        => self.println(&format!("successfully wrote to {}", src)),
                        Err(e) => self.println(&format!("error could not write: {:?}",e))
                    }
                }

                "write" => {
                    // needs to use closure to be able to use ?
                    let res = (|| -> Result<(),ParseIntError> {
                        self.print_rom(4);

                        self.println_str("where to?");
                        self.print_str("0x");
                        self.flush();

                        let mut i = i32::from_str_radix(self.input_from_stdin().to_lowercase().trim(), 16)?;

                        self.println_str("what byte(s)?");
                        self.print_str("0x");
                        self.flush();
                        // let b= i32::from_str_radix(self.input_from_stdin().to_lowercase().trim(), 16)? as u8;
                        let bytes = self.input_from_stdin().to_lowercase().trim().to_string();
                        for b in bytes.as_bytes().chunks(2) {
                            let b = from_utf8(b).unwrap();
                            let b = i32::from_str_radix(b, 16)? as u8;
                            self.rom.set_byte(i,b);
                            i += 1;
                        }

                        skip_prompt = true;

                        Ok(())
                    })();

                    if let Err(e) = res {
                        self.println(&format!("{:?}",e));
                    }
                }

                _ => {
                    self.print_rom(0);
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

    fn print_rom(&mut self, lines_off: usize) {
        // begin printing in bold
        self.print(&String::from("\x1B[1m"));

        // print the hex from 0-PRINT_INTERVAL as well as a gap
        let t: String = (0..PRINT_INTERVAL).map(|x| format!("{:02x} ", x)).collect();
        self.println(&format!("{} {}", " ".repeat(self.rom_size_length), t));

        // no more bold
        self.print(&String::from("\x1B[0m"));

        let mut rom_iter = self.rom.iterator_from(self.line);
        let mut lines_to_print: Vec<String> = Vec::new();
        // current line on the display
        let mut display_line = self.line;
        for _ in 0..ROM_LINE_COUNT - lines_off {
            let mut line_hex = String::new();
            for _ in 0..PRINT_INTERVAL {
                // share same iterator across all loop iterators so its continuous
                // does however ignore bounds checking ie whether iterator has any left
                let next = rom_iter.next().unwrap();
                let next = if self.raw {
                    format!("{:02x} ", next).to_uppercase()
                } else {
                    if let Some(chars) = self.rom.dict.get(*next) {
                        format!("{} ", chars)
                    } else {
                        format!("{:02x} ", next).to_uppercase()
                    }
                };
                line_hex.push_str(next.as_str());
            }
            lines_to_print.push(format!("{:01$x} {2}", display_line, self.rom_size_length, line_hex));
            display_line += PRINT_INTERVAL;
        }

        lines_to_print.iter().map(|line| self.println(line)).for_each(drop);
    }

    // wrapper for print as well for consistency
    fn print(&self, s: &String) {
        print!("{}",s);
    }
    fn print_str(&self, s: &str) { self.print(&String::from(s)) }

    // println except it also tracks printed lines
    fn println(&mut self, s: &String) {
        println!("{}",s);
        self.printed_count += 1;
    }
    fn println_str(&mut self, s: &str) { self.println(&String::from(s)) }

    fn input_from_stdin(&mut self) -> String {
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