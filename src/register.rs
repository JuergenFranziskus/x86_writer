use std::fmt::Display;

use crate::operand::{MemKind, Memory, Sib};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Reg {
    pub name: RegName,
    pub size: RegSize,
}
impl Reg {
    pub const fn new(name: RegName, size: RegSize) -> Self {
        Self { name, size }
    }
    pub const fn with_name(self, name: RegName) -> Self {
        Self::new(name, self.size)
    }
    pub const fn with_size(self, size: RegSize) -> Self {
        Self::new(self.name, size)
    }

    pub fn full_name(self) -> &'static str {
        match self {
            AL => "al",
            AX => "ax",
            EAX => "eax",
            RAX => "rax",
            BL => "bl",
            BX => "bx",
            EBX => "ebx",
            RBX => "rbx",
            CL => "cl",
            CX => "cx",
            ECX => "ecx",
            RCX => "rcx",
            DL => "dl",
            DX => "dx",
            EDX => "edx",
            RDX => "rdx",
            DIL => "dil",
            DI => "di",
            EDI => "edi",
            RDI => "rdi",
            SIL => "sil",
            SI => "si",
            ESI => "esi",
            RSI => "rsi",
            SPL => "spl",
            SP => "sp",
            ESP => "esp",
            RSP => "rsp",
            BPL => "bpl",
            BP => "bp",
            EBP => "ebp",
            RBP => "rbp",
            R9B => "r9b",
            R9W => "r9w",
            R9D => "r9d",
            R9 => "r9",
            R10B => "r10b",
            R10W => "r10w",
            R10D => "r10d",
            R10 => "r10",
            R11B => "r11b",
            R11W => "r11w",
            R11D => "r11d",
            R11 => "r11",
            R12B => "r12b",
            R12W => "r12w",
            R12D => "r12d",
            R12 => "r12",
            R13B => "r13b",
            R13W => "r13w",
            R13D => "r13d",
            R13 => "r13",
            R14B => "r14b",
            R14W => "r14w",
            R14D => "r14d",
            R14 => "r14",
            R15B => "r15b",
            R15W => "r15w",
            R15D => "r15d",
            R15 => "r15",
            _ => todo!(),
        }
    }

    pub fn mem(self) -> Memory {
        Memory {
            kind: MemKind::Sib(Sib {
                base: Some(self),
                index: None,
            }),
            label: None,
            offset: 0,
            size: None,
        }
    }
}
impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_name())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum RegName {
    A,
    B,
    C,
    D,
    SI,
    DI,
    SP,
    BP,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}
impl RegName {
    pub const fn with_size(self, size: RegSize) -> Reg {
        Reg::new(self, size)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum RegSize {
    Byte,
    Word,
    DWord,
    QWord,
}
impl RegSize {
    pub const fn with_name(self, name: RegName) -> Reg {
        Reg::new(name, self)
    }
}

pub const AL: Reg = Reg::new(RegName::A, RegSize::Byte);
pub const AX: Reg = Reg::new(RegName::A, RegSize::Word);
pub const EAX: Reg = Reg::new(RegName::A, RegSize::DWord);
pub const RAX: Reg = Reg::new(RegName::A, RegSize::QWord);
pub const BL: Reg = Reg::new(RegName::B, RegSize::Byte);
pub const BX: Reg = Reg::new(RegName::B, RegSize::Word);
pub const EBX: Reg = Reg::new(RegName::B, RegSize::DWord);
pub const RBX: Reg = Reg::new(RegName::B, RegSize::QWord);
pub const CL: Reg = Reg::new(RegName::C, RegSize::Byte);
pub const CX: Reg = Reg::new(RegName::C, RegSize::Word);
pub const ECX: Reg = Reg::new(RegName::C, RegSize::DWord);
pub const RCX: Reg = Reg::new(RegName::C, RegSize::QWord);
pub const DL: Reg = Reg::new(RegName::D, RegSize::Byte);
pub const DX: Reg = Reg::new(RegName::D, RegSize::Word);
pub const EDX: Reg = Reg::new(RegName::D, RegSize::DWord);
pub const RDX: Reg = Reg::new(RegName::D, RegSize::QWord);
pub const DIL: Reg = Reg::new(RegName::DI, RegSize::Byte);
pub const DI: Reg = Reg::new(RegName::DI, RegSize::Word);
pub const EDI: Reg = Reg::new(RegName::DI, RegSize::DWord);
pub const RDI: Reg = Reg::new(RegName::DI, RegSize::QWord);
pub const SIL: Reg = Reg::new(RegName::SI, RegSize::Byte);
pub const SI: Reg = Reg::new(RegName::SI, RegSize::Word);
pub const ESI: Reg = Reg::new(RegName::SI, RegSize::DWord);
pub const RSI: Reg = Reg::new(RegName::SI, RegSize::QWord);
pub const SPL: Reg = Reg::new(RegName::SP, RegSize::Byte);
pub const SP: Reg = Reg::new(RegName::SP, RegSize::Word);
pub const ESP: Reg = Reg::new(RegName::SP, RegSize::DWord);
pub const RSP: Reg = Reg::new(RegName::SP, RegSize::QWord);
pub const BPL: Reg = Reg::new(RegName::BP, RegSize::Byte);
pub const BP: Reg = Reg::new(RegName::BP, RegSize::Word);
pub const EBP: Reg = Reg::new(RegName::BP, RegSize::DWord);
pub const RBP: Reg = Reg::new(RegName::BP, RegSize::QWord);
pub const R9B: Reg = Reg::new(RegName::R9, RegSize::Byte);
pub const R9W: Reg = Reg::new(RegName::R9, RegSize::Word);
pub const R9D: Reg = Reg::new(RegName::R9, RegSize::DWord);
pub const R9: Reg = Reg::new(RegName::R9, RegSize::QWord);
pub const R10B: Reg = Reg::new(RegName::R10, RegSize::Byte);
pub const R10W: Reg = Reg::new(RegName::R10, RegSize::Word);
pub const R10D: Reg = Reg::new(RegName::R10, RegSize::DWord);
pub const R10: Reg = Reg::new(RegName::R10, RegSize::QWord);
pub const R11B: Reg = Reg::new(RegName::R11, RegSize::Byte);
pub const R11W: Reg = Reg::new(RegName::R11, RegSize::Word);
pub const R11D: Reg = Reg::new(RegName::R11, RegSize::DWord);
pub const R11: Reg = Reg::new(RegName::R11, RegSize::QWord);
pub const R12B: Reg = Reg::new(RegName::R12, RegSize::Byte);
pub const R12W: Reg = Reg::new(RegName::R12, RegSize::Word);
pub const R12D: Reg = Reg::new(RegName::R12, RegSize::DWord);
pub const R12: Reg = Reg::new(RegName::R12, RegSize::QWord);
pub const R13B: Reg = Reg::new(RegName::R13, RegSize::Byte);
pub const R13W: Reg = Reg::new(RegName::R13, RegSize::Word);
pub const R13D: Reg = Reg::new(RegName::R13, RegSize::DWord);
pub const R13: Reg = Reg::new(RegName::R13, RegSize::QWord);
pub const R14B: Reg = Reg::new(RegName::R14, RegSize::Byte);
pub const R14W: Reg = Reg::new(RegName::R14, RegSize::Word);
pub const R14D: Reg = Reg::new(RegName::R14, RegSize::DWord);
pub const R14: Reg = Reg::new(RegName::R14, RegSize::QWord);
pub const R15B: Reg = Reg::new(RegName::R15, RegSize::Byte);
pub const R15W: Reg = Reg::new(RegName::R15, RegSize::Word);
pub const R15D: Reg = Reg::new(RegName::R15, RegSize::DWord);
pub const R15: Reg = Reg::new(RegName::R15, RegSize::QWord);
