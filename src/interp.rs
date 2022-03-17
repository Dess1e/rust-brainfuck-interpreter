use std::io::{stdin, Stdin, stdout, Stdout, Write};
use std::time::SystemTime;

use preproc::Token;
use Token::*;
use crate::preproc;

pub struct Interpreter {
    cells: Vec<u32>,
    cell_ptr: usize,
    code: Vec<Token>,
    code_ptr: usize,
    stdout: Stdout,
    auto_flush_stdout: bool,
    stdin: Stdin,
    stdin_buffer: Vec<u32>,
}


impl Interpreter {
    pub fn new(
        code: Vec<Token>,
        cell_array_size: usize,
        auto_flush_stdout: bool

    ) -> Self {
        Interpreter {
            cells: vec![0; cell_array_size],
            cell_ptr: cell_array_size / 2,
            code,
            code_ptr: 0,
            stdout: stdout(),
            stdin: stdin(),
            stdin_buffer: Vec::new(),
            auto_flush_stdout
        }
    }

    fn get_current_instr(&mut self) -> &Token { &self.code[self.code_ptr] }

    fn get_cell_value(&mut self) -> u32 { self.cells[self.cell_ptr] }

    fn set_cell_value(&mut self, value: u32) { self.cells[self.cell_ptr] = value; }

    fn inc_cell_ptr(&mut self) { self.cell_ptr += 1; }

    fn dec_cell_ptr(&mut self) { self.cell_ptr -= 1; }

    fn inc_curr_cell(&mut self) {
        let new_val = self.get_cell_value().wrapping_add(1);
        self.set_cell_value(new_val);
    }

    fn dec_curr_cell(&mut self) {
        let new_val = self.get_cell_value().wrapping_sub(1);
        self.set_cell_value(new_val);
    }

    fn put_char(&mut self) {
        let chr = self.get_cell_value();
        self.stdout.write(&[chr as u8; 1]).unwrap();
        if self.auto_flush_stdout {
            self.stdout.flush();
        }
    }

    fn get_char(&mut self) {
        if self.stdin_buffer.is_empty() {
            let mut line = String::new();
            self.stdin.read_line(&mut line).unwrap();
            line.chars()
                .filter(|c| c.is_ascii())
                .for_each(|c| self.stdin_buffer.push(c as u32))
        }
        let buffered_value = self.stdin_buffer.pop().unwrap();
        self.set_cell_value(buffered_value);
    }

    fn loop_start(&mut self) {
        if self.get_cell_value() == 0 {
            while self.get_current_instr() != &LoopEnd {
                self.code_ptr += 1;
            }
        }
    }

    fn loop_end(&mut self) {
        if self.get_cell_value() != 0 {
            while self.get_current_instr() != &LoopStart {
                self.code_ptr -= 1;
            }
        }
    }

    pub fn main_loop(&mut self) {
        let mut time = SystemTime::now();
        let mut instr_count: u64 = 0;
        while self.code_ptr < self.code.len() {
            let instr = self.get_current_instr();
            match instr {
                MoveFwd => self.inc_cell_ptr(),
                MoveBack => self.dec_cell_ptr(),
                Inc => self.inc_curr_cell(),
                Dec => self.dec_curr_cell(),
                PutCh => self.put_char(),
                GetCh => self.get_char(),
                LoopStart => self.loop_start(),
                LoopEnd => self.loop_end(),
            }
            self.code_ptr += 1;

            instr_count += 1;
            if SystemTime::now().duration_since(time).unwrap().as_secs() > 1 {
                println!("Instructions per second: {}", instr_count);
                instr_count = 0;
                time = SystemTime::now();
            }
        }
    }
}

