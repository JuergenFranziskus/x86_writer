use std::io::{self, Write};

use crate::{
    label::{Label, Labels},
    operand::{MemKind, Operand, OperandSize, Scale},
    register::RegSize,
};

pub struct FasmWriter<O> {
    labels: Labels,
    out: O,
}
impl<O: Write> FasmWriter<O> {
    pub fn new(out: O) -> Self {
        Self {
            labels: Labels::new(),
            out,
        }
    }

    pub fn add_label(&mut self, label: impl Into<String>) -> Label {
        self.labels.add(label)
    }
    pub fn emit_label(&mut self, label: Label) -> io::Result<()> {
        let text = &self.labels[label];
        writeln!(self.out, "    {text}:")?;
        Ok(())
    }

    pub fn prelude(&mut self) -> io::Result<()> {
        writeln!(self.out, "format ELF64 executable")?;
        Ok(())
    }
    pub fn segment(&mut self, readable: bool, writable: bool, executable: bool) -> io::Result<()> {
        write!(self.out, "segment")?;
        if readable {
            write!(self.out, " readable")?;
        }
        if writable {
            write!(self.out, " writable")?;
        }
        if executable {
            write!(self.out, " executable")?;
        }
        writeln!(self.out)?;

        Ok(())
    }
    pub fn entry(&mut self, label: Label) -> io::Result<()> {
        writeln!(self.out, "entry {}", &self.labels[label])?;
        Ok(())
    }

    fn print_operand(&mut self, op: Operand) -> io::Result<()> {
        match op {
            Operand::Immediate(value) => write!(self.out, "{value}")?,
            Operand::Label(id) => write!(self.out, "{}", &self.labels[id])?,
            Operand::Register(reg) => write!(self.out, "{reg}")?,
            Operand::Memory(mem) => {
                if let Some(size) = mem.size {
                    self.print_operand_size(size)?;
                    write!(self.out, " ")?;
                }
                write!(self.out, "[")?;
                let has_base = match mem.kind {
                    MemKind::Rip => {
                        write!(self.out, "rip")?;
                        true
                    }
                    MemKind::Sib(sib) => {
                        if let Some(base) = sib.base {
                            write!(self.out, "{base}")?;
                        }
                        if let Some((index, scale)) = sib.index {
                            if sib.base.is_some() {
                                write!(self.out, " + ")?;
                            }
                            write!(self.out, "{index}")?;
                            if scale != Scale::One {
                                write!(self.out, " * {}", scale.numeric())?;
                            }
                        }

                        sib.base.is_some() || sib.index.is_some()
                    }
                };
                let has_label = if let Some(label) = mem.label {
                    if has_base {
                        write!(self.out, " + ")?;
                    }
                    write!(self.out, "{}", &self.labels[label])?;
                    true
                } else {
                    false
                };
                if mem.offset != 0 {
                    if (has_label || has_base) && mem.offset < 0 {
                        write!(self.out, " - ")?;
                    } else if has_label || has_base {
                        write!(self.out, " + ")?;
                    }
                    write!(self.out, "{}", mem.offset.unsigned_abs())?;
                }
                write!(self.out, "]")?;
            }
        }

        Ok(())
    }
    fn print_operand_size(&mut self, size: OperandSize) -> io::Result<()> {
        match size {
            OperandSize::Register(RegSize::Byte) => write!(self.out, "byte")?,
            OperandSize::Register(RegSize::Word) => write!(self.out, "word")?,
            OperandSize::Register(RegSize::DWord) => write!(self.out, "dword")?,
            OperandSize::Register(RegSize::QWord) => write!(self.out, "qword")?,
        }
        Ok(())
    }

    fn emit_unary_instruction(&mut self, name: &str, to: impl Into<Operand>) -> io::Result<()> {
        let to: Operand = to.into();
        let _to_size = to.size().expect("operand does not have a known size");
        write!(self.out, "    {name} ")?;
        self.print_operand(to)?;
        writeln!(self.out)?;

        Ok(())
    }
    fn emit_binary_instruction(
        &mut self,
        name: &str,
        to: impl Into<Operand>,
        from: impl Into<Operand>,
    ) -> io::Result<()> {
        let to: Operand = to.into();
        let from: Operand = from.into();
        let to_size = to.size();
        let from_size = from.size();
        let _size = common_size(to_size, from_size).expect("operands do not have a common size");

        write!(self.out, "    {name} ")?;
        self.print_operand(to)?;
        write!(self.out, ", ")?;
        self.print_operand(from)?;
        writeln!(self.out)?;

        Ok(())
    }

    pub fn call(&mut self, to: impl Into<Operand>) -> io::Result<()> {
        self.emit_unary_instruction("call", to)
    }
    pub fn mov(&mut self, to: impl Into<Operand>, from: impl Into<Operand>) -> io::Result<()> {
        self.emit_binary_instruction("mov", to, from)
    }
    pub fn pop(&mut self, to: impl Into<Operand>) -> io::Result<()> {
        self.emit_unary_instruction("pop", to)
    }
    pub fn push(&mut self, to: impl Into<Operand>) -> io::Result<()> {
        self.emit_unary_instruction("push", to)
    }
    pub fn ret(&mut self) -> io::Result<()> {
        writeln!(self.out, "    ret")
    }
}

fn common_size(a: Option<OperandSize>, b: Option<OperandSize>) -> Option<OperandSize> {
    match (a, b) {
        (None, None) => None,
        (Some(s), None) | (None, Some(s)) => Some(s),
        (Some(a), Some(b)) if a == b => Some(a),
        (Some(_), Some(_)) => None,
    }
}
