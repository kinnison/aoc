//! Intcode VM for 2019 AoC
//!
//!

/// Errors this stuff can return
#[derive(Debug)]
pub enum Error {
    ParseError(String),
    BadAddress(i64),
    BadOpCode(i64),
    UnknownOpCode(i64),
    UnknownAddressingMode(i64),
    NoMoreInput(i64),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn std::fmt::Debug).fmt(f)
    }
}

impl std::error::Error for Error {}

/// A VM Result
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum OpCode {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    // Last opcode
    Terminate,
}

impl OpCode {
    pub fn convert(from: i64) -> Result<Self> {
        if from < 1 {
            Err(Error::BadOpCode(from))
        } else {
            let opval = from % 100;
            match opval {
                1 => Ok(Self::Add),
                2 => Ok(Self::Mul),
                3 => Ok(Self::Input),
                4 => Ok(Self::Output),
                5 => Ok(Self::JumpIfTrue),
                6 => Ok(Self::JumpIfFalse),
                7 => Ok(Self::LessThan),
                8 => Ok(Self::Equals),
                99 => Ok(Self::Terminate),

                _ => Err(Error::UnknownOpCode(opval)),
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VMState {
    Runnable,
    WaitingOnInput,
    GaveOutput(i64),
    Halted,
}

/// The VM itself
#[derive(Debug, Clone)]
pub struct VM {
    ram: Vec<i64>,
    pc: i64,
    curstate: VMState,
}

impl VM {
    pub fn peek(&self, addr: i64) -> Result<i64> {
        if addr < 0 || (addr as usize) >= self.ram.len() {
            Err(Error::BadAddress(addr))
        } else {
            Ok(self.ram[addr as usize])
        }
    }

    pub fn poke(&mut self, addr: i64, value: i64) -> Result<()> {
        if addr < 0 || (addr as usize) >= self.ram.len() {
            Err(Error::BadAddress(addr))
        } else {
            self.ram[addr as usize] = value;
            Ok(())
        }
    }

    pub fn opcode(&self) -> Result<OpCode> {
        let opval = self.peek(self.pc)?;
        OpCode::convert(opval)
    }

    pub fn addr_for(&self, operand: i64) -> Result<i64> {
        let opval = self.peek(self.pc)?;
        let divisor = 10i64.pow((operand + 2) as u32);
        let shifted = opval / divisor;
        let mode = shifted % 10;
        match mode {
            0 => self.peek(self.pc + 1 + operand),
            1 => Ok(self.pc + 1 + operand),
            _ => Err(Error::UnknownAddressingMode(mode)),
        }
    }

    pub fn debug_instr(&self, args: i64) -> Result<()> {
        if cfg!(debug_assertions) {
            print!("PC={} OpVal={} ", self.pc, self.peek(self.pc)?);
            print!("OpCode={:?} Args=", self.opcode()?);
            for arg in 0..args {
                let argval = self.peek(self.pc + 1 + arg)?;
                let argaddr = self.addr_for(arg)?;
                let argres = self.peek(self.addr_for(arg)?)?;
                print!(" {}[@{} => {}]", argval, argaddr, argres);
            }
            println!();
        }
        Ok(())
    }

    pub fn run_add(&mut self) -> Result<i64> {
        let arg1 = self.peek(self.addr_for(0)?)?;
        let arg2 = self.peek(self.addr_for(1)?)?;
        self.poke(self.addr_for(2)?, arg1 + arg2)?;
        Ok(self.pc + 4)
    }

    pub fn run_mul(&mut self) -> Result<i64> {
        let arg1 = self.peek(self.addr_for(0)?)?;
        let arg2 = self.peek(self.addr_for(1)?)?;
        self.poke(self.addr_for(2)?, arg1 * arg2)?;
        Ok(self.pc + 4)
    }

    pub fn run_input(&mut self, input: i64) -> Result<i64> {
        self.poke(self.addr_for(0)?, input)?;
        Ok(self.pc + 2)
    }

    pub fn run_output(&self) -> Result<(i64, i64)> {
        Ok((self.pc + 2, self.peek(self.addr_for(0)?)?))
    }

    pub fn run_jump_if_true(&self) -> Result<i64> {
        self.debug_instr(2)?;
        let arg = self.peek(self.addr_for(0)?)?;
        if arg == 0 {
            // false
            Ok(self.pc + 3)
        } else {
            Ok(self.peek(self.addr_for(1)?)?)
        }
    }

    pub fn run_jump_if_false(&self) -> Result<i64> {
        self.debug_instr(2)?;
        let arg = self.peek(self.addr_for(0)?)?;
        if arg != 0 {
            // true
            Ok(self.pc + 3)
        } else {
            Ok(self.peek(self.addr_for(1)?)?)
        }
    }

    pub fn run_less_than(&mut self) -> Result<i64> {
        self.debug_instr(3)?;
        let arg1 = self.peek(self.addr_for(0)?)?;
        let arg2 = self.peek(self.addr_for(1)?)?;
        if arg1 < arg2 {
            self.poke(self.addr_for(2)?, 1)?;
        } else {
            self.poke(self.addr_for(2)?, 0)?;
        }
        Ok(self.pc + 4)
    }

    pub fn run_equals(&mut self) -> Result<i64> {
        self.debug_instr(3)?;
        let arg1 = self.peek(self.addr_for(0)?)?;
        let arg2 = self.peek(self.addr_for(1)?)?;
        if arg1 == arg2 {
            self.poke(self.addr_for(2)?, 1)?;
        } else {
            self.poke(self.addr_for(2)?, 0)?;
        }
        Ok(self.pc + 4)
    }

    pub fn interpreter_step(&mut self, input: Option<i64>) -> Result<VMState> {
        loop {
            match self.curstate {
                VMState::Runnable => {
                    let new_pc = match self.opcode()? {
                        OpCode::Add => self.run_add(),
                        OpCode::Mul => self.run_mul(),
                        OpCode::Input => {
                            self.curstate = VMState::WaitingOnInput;
                            Ok(self.pc)
                        }
                        OpCode::Output => {
                            let (pc, out) = self.run_output()?;
                            self.curstate = VMState::GaveOutput(out);
                            Ok(pc)
                        }
                        OpCode::JumpIfTrue => self.run_jump_if_true(),
                        OpCode::JumpIfFalse => self.run_jump_if_false(),
                        OpCode::LessThan => self.run_less_than(),
                        OpCode::Equals => self.run_equals(),
                        OpCode::Terminate => {
                            self.curstate = VMState::Halted;
                            Ok(self.pc)
                        }
                    }?;
                    self.pc = new_pc;
                }
                VMState::WaitingOnInput => {
                    assert_eq!(self.opcode()?, OpCode::Input);
                    if let Some(input) = input {
                        self.pc = self.run_input(input)?;
                        self.curstate = VMState::Runnable;
                    } else {
                        // Do nothing, we want input, none was given
                    }
                }
                VMState::GaveOutput(_) => {
                    self.curstate = VMState::Runnable;
                }
                VMState::Halted => {
                    // Do nothing
                }
            }
            if self.curstate != VMState::Runnable {
                break Ok(self.curstate);
            }
        }
    }

    pub fn full_interpret(&mut self, input: &[i64], output: &mut Vec<i64>) -> Result<()> {
        let mut input_cursor = 0;
        let mut vmstate = self.interpreter_step(None)?;
        while vmstate != VMState::Halted {
            match vmstate {
                VMState::Runnable => vmstate = self.interpreter_step(None)?,
                VMState::WaitingOnInput => {
                    if input_cursor == input.len() {
                        return Err(Error::NoMoreInput(self.pc));
                    }
                    vmstate = self.interpreter_step(Some(input[input_cursor]))?;
                    input_cursor += 1;
                }
                VMState::GaveOutput(v) => {
                    output.push(v);
                    vmstate = self.interpreter_step(None)?;
                }
                VMState::Halted => {}
            }
        }
        Ok(())
    }

    pub fn interpret(&mut self) -> Result<()> {
        self.full_interpret(&[], &mut Vec::new())
    }
}

impl std::str::FromStr for VM {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let ram: Vec<i64> =
            super::line_as_list(s).map_err(|e| Error::ParseError(format!("{}", e)))?;
        Ok(Self {
            ram,
            pc: 0,
            curstate: VMState::Runnable,
        })
    }
}
