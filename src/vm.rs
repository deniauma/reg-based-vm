use crate::instruction::Opcode;

pub struct VM {
    pub registers: [i32; 32],
    heap: [u8; 1000],
    pc: usize,
    pub program: Vec<u8>,
    remainder: u32,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            heap: [0; 1000],
            pc: 0,
            program: vec![],
            remainder: 0,
        }
    }

    pub fn add_program_byte(&mut self, byte: u8) {
        self.program.push(byte);
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        return result;
    }

    fn load_word_from_heap(&self, addr: usize) -> Result<u32, String> {
        match self.heap.get(addr..addr+4) {
            Some(v) => {
                let result: u32 = ((v[0] as u32) << 3*8) | ((v[1] as u32) << 2*8) | ((v[2] as u32) << 8) | v[3] as u32;
                Ok(result)
            }
            None => Err(format!("Error, memory addr ({}) is out of bounds!", addr))
        }
    }

    fn store_word_into_heap(&mut self, value: i32, addr: usize) {
        let mut bytes: Vec<u8> = vec!();
        bytes.push((value >> 24) as u8);
        bytes.push((value >> 16) as u8);
        bytes.push((value >> 8) as u8);
        bytes.push(value as u8);
        for i in 0..4 {
            self.heap[addr + i] = bytes[i];
        }
    }

    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    /// Executes one instruction. Meant to allow for more controlled execution of the VM
    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }
        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u32;
                self.registers[register] = number as i32;
            }
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            }
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            }
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
            }
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
                self.remainder = (register1 % register2) as u32;
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            }
            Opcode::JMPF => {
                let value = self.registers[self.next_8_bits() as usize] as usize;
                self.pc += value;
            }
            Opcode::JMPB => {
                let value = self.registers[self.next_8_bits() as usize] as usize;
                self.pc -= value;
            }
            Opcode::EQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                let result = self.next_8_bits() as usize;
                if register1 == register2 {
                    self.registers[result] = 1;
                } else {
                    self.registers[result] = 0;
                }
            }
            Opcode::NEQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                let result = self.next_8_bits() as usize;
                if register1 != register2 {
                    self.registers[result] = 1;
                } else {
                    self.registers[result] = 0;
                }
            }
            Opcode::GT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                let result = self.next_8_bits() as usize;
                if register1 > register2 {
                    self.registers[result] = 1;
                } else {
                    self.registers[result] = 0;
                }
            }
            Opcode::LT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                let result = self.next_8_bits() as usize;
                if register1 < register2 {
                    self.registers[result] = 1;
                } else {
                    self.registers[result] = 0;
                }
            }
            Opcode::GTQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                let result = self.next_8_bits() as usize;
                if register1 >= register2 {
                    self.registers[result] = 1;
                } else {
                    self.registers[result] = 0;
                }
            }
            Opcode::LTQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                let result = self.next_8_bits() as usize;
                if register1 <= register2 {
                    self.registers[result] = 1;
                } else {
                    self.registers[result] = 0;
                }
            }
            Opcode::JEQ => {
                let target = self.registers[self.next_8_bits() as usize];
                let compare_value = self.registers[self.next_8_bits() as usize];
                if compare_value == 1 {
                    self.pc = target as usize;
                } else {
                    self.next_8_bits();
                }
            }
            Opcode::LW => { // lw $1, 100($2)
                let reg_dst = self.next_8_bits() as usize;
                let addr = self.registers[self.next_8_bits() as usize] as usize;
                let offset = self.next_8_bits() as usize;
                self.registers[reg_dst] = self.load_word_from_heap(addr + offset).unwrap() as i32;
            }
            Opcode::SW => { // sw $1, 100($2)
                let value = self.registers[self.next_8_bits() as usize];
                let addr = self.registers[self.next_8_bits() as usize] as usize;
                let offset = self.next_8_bits() as usize;
                self.store_word_into_heap(value, addr + offset);
            }
            Opcode::HLT => {
                println!("HLT encountered");
                return false;
            }
            Opcode::IGL => {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 1, 244]; // Remember, this is how we represent 500 using two u8s in little endian format
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 0, 0, 6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_eq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![9, 0, 1, 2, 9, 0, 1, 2];
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 1);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 0);
    }

    #[test]
    fn test_jeq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 7;
        test_vm.registers[1] = 1;
        test_vm.program = vec![15, 0, 1, 2, 15, 0, 1, 2];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 7);
        test_vm.pc = 4;
        test_vm.registers[1] = 0;
        test_vm.run_once();
        println!("{}", test_vm.pc);
        assert_eq!(test_vm.pc, 8);
    }

    #[test]
    fn test_lw_sw_opcodes() {
        let mut test_vm = VM::new();
        test_vm.registers[1] = 1589;
        test_vm.registers[2] = 32;
        test_vm.program = vec![17, 1, 2, 8, 16, 3, 2, 8]; // sw $1, 8($2) then lw $3, 8($2)
        test_vm.run_once();
        assert_eq!(test_vm.registers[3], 0);
        test_vm.run_once();
        assert_eq!(test_vm.registers[3], 1589);
    }
}
