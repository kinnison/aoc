//! Intcode VM for 2019 AoC
//!

/// Errors this stuff can return
#[derive(Debug)]
pub enum Error {
    ParseError(String),
    BadAddress(i64),
    BadOpCode(i64),
    UnknownOpCode(i64),
    UnknownAddressingMode(i64),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn std::fmt::Debug).fmt(f)
    }
}

impl std::error::Error for Error {}

/// A VM Result
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum OpCode {
    Add,
    Mul,
    Input,
    Output,

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

                99 => Ok(Self::Terminate),

                _ => Err(Error::UnknownOpCode(opval)),
            }
        }
    }
}

/// The VM itself
#[derive(Debug, Clone)]
pub struct VM {
    ram: Vec<i64>,
    pc: i64,
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
        let mode = (opval / ((operand + 2) * 10)) % 10;
        match mode {
            0 => self.peek(self.pc + 1 + operand),
            1 => Ok(self.pc + 1 + operand),
            _ => Err(Error::UnknownAddressingMode(mode)),
        }
    }

    pub fn debug_instr(&self, args: i64) -> Result<()> {
        if cfg!(test) {
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
        self.debug_instr(3)?;
        let arg1 = self.peek(self.addr_for(0)?)?;
        let arg2 = self.peek(self.addr_for(1)?)?;
        self.poke(self.addr_for(2)?, arg1 + arg2)?;
        Ok(self.pc + 4)
    }

    pub fn run_mul(&mut self) -> Result<i64> {
        self.debug_instr(3)?;
        let arg1 = self.peek(self.addr_for(0)?)?;
        let arg2 = self.peek(self.addr_for(1)?)?;
        self.poke(self.addr_for(2)?, arg1 * arg2)?;
        Ok(self.pc + 4)
    }

    pub fn interpret(&mut self) -> Result<()> {
        loop {
            let new_pc = match self.opcode()? {
                OpCode::Add => self.run_add(),
                OpCode::Mul => self.run_mul(),
                OpCode::Input => unimplemented!(),
                OpCode::Output => unimplemented!(),
                OpCode::Terminate => break Ok(()),
            }?;
            self.pc = new_pc;
        }
    }
}

impl std::str::FromStr for VM {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let ram: Vec<i64> =
            super::line_as_list(s).map_err(|e| Error::ParseError(format!("{}", e)))?;
        Ok(Self { ram, pc: 0 })
    }
}
