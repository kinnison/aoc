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
    IOError(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn std::fmt::Debug).fmt(f)
    }
}

impl std::error::Error for Error {}

impl std::convert::From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IOError(value)
    }
}

/// A VM Result
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum OpCode {
    Add = 1,
    Mul = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    SetRelativeBase = 9,
    // Last opcode
    Terminate = 99,
}

impl OpCode {
    fn convert(from: i64) -> Result<Self> {
        if cfg!(debug_assertions) && from < 1 {
            return Err(Error::BadOpCode(from));
        }
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
            9 => Ok(Self::SetRelativeBase),
            99 => Ok(Self::Terminate),

            _ => Err(Error::UnknownOpCode(opval)),
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
    relative_base: i64,
}

impl VM {
    #[inline]
    pub fn peek(&self, addr: i64) -> Result<i64> {
        if cfg!(debug_assertions) && addr < 0 {
            return Err(Error::BadAddress(addr));
        }
        let addr = addr as usize;
        Ok(if addr < self.ram.len() {
            self.ram[addr]
        } else {
            0
        })
    }

    #[inline]
    pub fn poke(&mut self, addr: i64, value: i64) -> Result<()> {
        if cfg!(debug_assertions) {
            if addr < 0 {
                Err(Error::BadAddress(addr))
            } else {
                let addr = addr as usize;
                while addr >= self.ram.len() {
                    self.ram.resize(addr + 1024, 0);
                }
                self.ram[addr] = value;
                Ok(())
            }
        } else {
            let addr = addr as usize;
            while addr >= self.ram.len() {
                self.ram.resize(addr + 1024, 0);
            }
            self.ram[addr] = value;
            Ok(())
        }
    }

    fn opcode(&self) -> Result<OpCode> {
        let opval = self.peek(self.pc)?;
        OpCode::convert(opval)
    }

    fn addr_for(&self, operand: i64) -> Result<i64> {
        let opval = self.peek(self.pc)?;
        let divisor = 10i64.pow((operand + 2) as u32);
        let shifted = opval / divisor;
        let mode = shifted % 10;
        match mode {
            0 => self.peek(self.pc + 1 + operand),
            1 => Ok(self.pc + 1 + operand),
            2 => Ok(self.peek(self.pc + 1 + operand)? + self.relative_base),
            _ => Err(Error::UnknownAddressingMode(mode)),
        }
    }

    #[cfg(feature = "debug_intcode")]
    fn debug_instr(&self, args: i64) -> Result<()> {
        print!(
            "RB={} PC={} OpVal={} ",
            self.relative_base,
            self.pc,
            self.peek(self.pc)?
        );
        print!("OpCode={:?} Args=", self.opcode()?);
        for arg in 0..args {
            let argval = self.peek(self.pc + 1 + arg)?;
            let argaddr = self.addr_for(arg)?;
            let argres = self.peek(self.addr_for(arg)?)?;
            print!(" {}[@{} => {}]", argval, argaddr, argres);
        }
        println!();
        Ok(())
    }

    #[cfg(not(feature = "debug_intcode"))]
    fn debug_instr(&self, _args: i64) -> Result<()> {
        Ok(())
    }

    fn run_add(&mut self) -> Result<i64> {
        self.debug_instr(3)?;
        let arg1 = self.peek(self.addr_for(0)?)?;
        let arg2 = self.peek(self.addr_for(1)?)?;
        self.poke(self.addr_for(2)?, arg1 + arg2)?;
        Ok(self.pc + 4)
    }

    fn run_mul(&mut self) -> Result<i64> {
        self.debug_instr(3)?;
        let arg1 = self.peek(self.addr_for(0)?)?;
        let arg2 = self.peek(self.addr_for(1)?)?;
        self.poke(self.addr_for(2)?, arg1 * arg2)?;
        Ok(self.pc + 4)
    }

    fn run_input(&mut self, input: i64) -> Result<i64> {
        self.debug_instr(1)?;
        self.poke(self.addr_for(0)?, input)?;
        Ok(self.pc + 2)
    }

    fn run_output(&self) -> Result<(i64, i64)> {
        self.debug_instr(1)?;
        Ok((self.pc + 2, self.peek(self.addr_for(0)?)?))
    }

    fn run_jump_if_true(&self) -> Result<i64> {
        self.debug_instr(2)?;
        let arg = self.peek(self.addr_for(0)?)?;
        if arg == 0 {
            // false
            Ok(self.pc + 3)
        } else {
            Ok(self.peek(self.addr_for(1)?)?)
        }
    }

    fn run_jump_if_false(&self) -> Result<i64> {
        self.debug_instr(2)?;
        let arg = self.peek(self.addr_for(0)?)?;
        if arg != 0 {
            // true
            Ok(self.pc + 3)
        } else {
            Ok(self.peek(self.addr_for(1)?)?)
        }
    }

    fn run_less_than(&mut self) -> Result<i64> {
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

    fn run_equals(&mut self) -> Result<i64> {
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

    fn run_relative_base(&mut self) -> Result<i64> {
        self.debug_instr(1)?;
        let arg1 = self.peek(self.addr_for(0)?)?;
        self.relative_base += arg1;
        Ok(self.pc + 2)
    }

    pub fn interpreter_step(&mut self, mut input: Option<i64>) -> Result<VMState> {
        'outer: loop {
            match self.curstate {
                VMState::Runnable => {
                    let new_pc = match self.opcode()? {
                        OpCode::Add => self.run_add(),
                        OpCode::Mul => self.run_mul(),
                        OpCode::Input => {
                            self.curstate = VMState::WaitingOnInput;
                            if input.is_some() {
                                continue 'outer;
                            }
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
                        OpCode::SetRelativeBase => self.run_relative_base(),
                        OpCode::Terminate => {
                            self.curstate = VMState::Halted;
                            Ok(self.pc)
                        }
                    }?;
                    self.pc = new_pc;
                }
                VMState::WaitingOnInput => {
                    assert_eq!(self.opcode()?, OpCode::Input);
                    if let Some(input) = input.take() {
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

    /// ASCII machines run until they output a non-ASCII value
    /// at which point that is saved and they run to termination
    pub fn run_ascii_machine(&mut self) -> Result<i64> {
        use std::io::{Read, Write};
        let mut ret = 0;
        let mut input = None;
        loop {
            match self.interpreter_step(input.take())? {
                VMState::Halted => break,
                VMState::Runnable => continue,
                VMState::GaveOutput(o) => {
                    if o > 0 && o < 255 {
                        std::io::stdout().write_all(&[o as u8])?;
                        if o == b'\n' as i64 {
                            std::io::stdout().flush()?;
                        }
                    } else {
                        ret = o
                    }
                }
                VMState::WaitingOnInput => {
                    if input.is_none() {
                        let mut ch: [u8; 1] = [0];
                        std::io::stdin().read_exact(&mut ch)?;
                        input = Some(ch[0] as i64);
                    }
                }
            }
        }

        Ok(ret)
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
            relative_base: 0,
        })
    }
}
