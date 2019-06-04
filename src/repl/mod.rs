use std;
use std::io;
use std::io::Write;
use crate::vm::VM;

/// Core structure for the REPL for the Assembler
pub struct REPL {
    command_buffer: Vec<String>,
    // The VM the REPL will use to execute code
    vm: VM,
}

impl REPL {
    /// Creates and returns a new assembly REPL
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![]
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to Iridium! Let's be productive!");
        loop {
            // This allocates a new String in which to store whatever the user types each iteration.
            let mut buffer = String::new();

            // Blocking call until the user types in a command
            let stdin = io::stdin();

            // Annoyingly, `print!` does not automatically flush stdout like `println!` does, so we
            // have to do that there for the user to see our `>>> ` prompt.
            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");

            // Here we'll look at the string the user gave us.
            stdin.read_line(&mut buffer).expect("Unable to read line from user");
            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());
            match buffer {
                ".quit" => {
                    println!("Farewell! Have a great day!");
                    std::process::exit(0);
                },
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                },
                ".program" => {
                    println!("Listing instructions currently in VM's program vector:");
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                    println!("End of Program Listing");
                },
                ".registers" => {
                    println!("Listing registers and all contents:");
                    println!("{:#?}", self.vm.registers);
                    println!("End of Register Listing")
                },
                _ => {
                    match self.parse_hex(buffer) {
                        Ok(bytes) => {
                            for b in bytes {
                                self.vm.add_program_byte(b);
                            }
                        },
                        Err(_) => println!("Unable to parse hex string (it should be 4 bytes).")
                    }
                    self.vm.run_once();
                }
            }
        }
    }

    fn parse_hex(&mut self, c: &str) -> Result<Vec<u8>, &str> {
        let split: Vec<&str> = c.split(" ").collect();
        if split.is_empty() {
            return Err("Error parsing the command!")
        }
        let mut results: Vec<u8> = vec![];
        for hex in split {
            let byte = u8::from_str_radix(hex, 16);
            match byte {
                Ok(res) => results.push(res),
                Err(_) => return Err("Error parsing the command!")
            }
        }
        Ok(results)
    }
}
