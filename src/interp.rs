use std::io::{stdin, Stdin, stdout, Stdout, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use preproc::Token;
use Token::*;
use crate::preproc;

#[derive(Clone)]
struct Code {
    instr_arr: Vec<Token>,
    instr_ptr: usize
}

pub struct Interpreter {
    cells: Vec<u8>,
    cell_ptr: usize,
    root_code: Code,
    stdout: Stdout,
    auto_flush_stdout: bool,
    stdin: Stdin,
    stdin_buffer: Vec<u8>,
    executed_instr_count_buf: u64,
    pub executed_instr_count: Arc<AtomicU64>,
}


impl Code {
    fn get_current_instr(&mut self) -> &Token { &self.instr_arr[self.instr_ptr] }
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
            root_code: Code {
                instr_arr: code,
                instr_ptr: 0
            },
            auto_flush_stdout,
            ..Default::default()
        }
    }


    fn get_cell_value(&mut self) -> u8 { self.cells[self.cell_ptr] }

    fn set_cell_value(&mut self, value: u8) { self.cells[self.cell_ptr] = value; }

    fn inc_cell_ptr(&mut self, value: usize) { self.cell_ptr += value; }

    fn dec_cell_ptr(&mut self, value: usize) { self.cell_ptr -= value; }

    fn inc_curr_cell(&mut self, value: u8) {
        let new_val = self.get_cell_value().wrapping_add(value);
        self.set_cell_value(new_val);
    }

    fn dec_curr_cell(&mut self, value: u8) {
        let new_val = self.get_cell_value().wrapping_sub(value);
        self.set_cell_value(new_val);
    }

    fn put_char(&mut self) {
        let chr = self.get_cell_value();
        self.stdout.write(&[chr; 1]).unwrap();
        if self.auto_flush_stdout {
            self.stdout.flush().unwrap();
        }
    }

    fn get_char(&mut self) {
        if self.stdin_buffer.is_empty() {
            let mut line = String::new();
            self.stdin.read_line(&mut line).unwrap();
            line.chars()
                .filter(|c| c.is_ascii())
                .for_each(|c| self.stdin_buffer.push(c as u8))
        }
        let buffered_value = self.stdin_buffer.pop().unwrap();
        self.set_cell_value(buffered_value);
    }

    fn process(&mut self, code: &mut Code) {
        while code.instr_ptr < code.instr_arr.len() {
            let instr = code.get_current_instr();
            match instr {
                MoveFwd(n) => self.inc_cell_ptr(*n as usize),
                MoveBack(n) => self.dec_cell_ptr(*n as usize),
                Inc(n) => self.inc_curr_cell(*n as u8),
                Dec(n) => self.dec_curr_cell(*n as u8),
                PutCh => self.put_char(),
                GetCh => self.get_char(),
                Loop(inner_instr) => {
                    let mut inner_code = Code {
                        instr_arr: inner_instr.clone(),
                        instr_ptr: 0
                    };
                    self.process(&mut inner_code)
                },
                LoopL => {
                    if self.get_cell_value() == 0 {
                        code.instr_ptr = code.instr_arr.len();
                    }
                }
                LoopR => {
                    if self.get_cell_value() != 0 {
                        code.instr_ptr = 0;
                    }
                }
            }
            self.inc_executed_instr_count();
            code.instr_ptr += 1;
        }
    }

    fn inc_executed_instr_count(&mut self) {
        if self.executed_instr_count_buf > 100000 {
            self.executed_instr_count.fetch_add(self.executed_instr_count_buf + 1, Ordering::SeqCst);
            self.executed_instr_count_buf = 0;
        } else {
            self.executed_instr_count_buf += 1;
        }

    }

    pub fn main_loop(&mut self) {
        let mut root_code = self.root_code.clone();
        self.process(&mut root_code);
    }
}


impl Default for Interpreter {
    fn default() -> Self {
        Self {
            cells: Vec::new(),
            cell_ptr: 0,
            root_code: Code {
                instr_arr: Vec::new(),
                instr_ptr: 0,
            },
            stdout: stdout(),
            auto_flush_stdout: false,
            stdin: stdin(),
            stdin_buffer: Vec::new(),
            executed_instr_count_buf: 0,
            executed_instr_count: Arc::new(AtomicU64::new(0)),
        }
    }
}
