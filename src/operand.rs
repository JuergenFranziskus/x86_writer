use std::ops::Add;

use crate::{
    label::Label,
    register::{Reg, RegSize},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum OperandSize {
    Register(RegSize),
}
impl From<RegSize> for OperandSize {
    fn from(value: RegSize) -> Self {
        Self::Register(value)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Operand {
    Immediate(i128),
    Label(Label),
    Memory(Memory),
    Register(Reg),
}
impl Operand {
    pub fn size(self) -> Option<OperandSize> {
        match self {
            Self::Immediate(_) => None,
            Self::Label(_) => None,
            Self::Memory(mem) => mem.size,
            Self::Register(reg) => Some(reg.size.into()),
        }
    }
}
impl From<Reg> for Operand {
    fn from(value: Reg) -> Self {
        Self::Register(value)
    }
}
impl From<Label> for Operand {
    fn from(value: Label) -> Self {
        Self::Label(value)
    }
}
impl From<Memory> for Operand {
    fn from(value: Memory) -> Self {
        Self::Memory(value)
    }
}
impl From<u8> for Operand {
    fn from(value: u8) -> Self {
        Self::Immediate(value as i128)
    }
}
impl From<u16> for Operand {
    fn from(value: u16) -> Self {
        Self::Immediate(value as i128)
    }
}
impl From<u32> for Operand {
    fn from(value: u32) -> Self {
        Self::Immediate(value as i128)
    }
}
impl From<u64> for Operand {
    fn from(value: u64) -> Self {
        Self::Immediate(value as i128)
    }
}
impl From<i8> for Operand {
    fn from(value: i8) -> Self {
        Self::Immediate(value as i128)
    }
}
impl From<i16> for Operand {
    fn from(value: i16) -> Self {
        Self::Immediate(value as i128)
    }
}
impl From<i32> for Operand {
    fn from(value: i32) -> Self {
        Self::Immediate(value as i128)
    }
}
impl From<i64> for Operand {
    fn from(value: i64) -> Self {
        Self::Immediate(value as i128)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Memory {
    pub kind: MemKind,
    pub label: Option<Label>,
    pub offset: i32,
    pub size: Option<OperandSize>,
}
impl Memory {
    pub fn rip(label: Label) -> Self {
        Self {
            kind: MemKind::Rip,
            label: Some(label),
            offset: 0,
            size: None,
        }
    }

    pub fn index(mut self, index: Reg) -> Self {
        let MemKind::Sib(sib) = &mut self.kind else {
            panic!()
        };
        sib.index = Some((index, Scale::One));

        self
    }
    pub fn scaled(mut self, index: Reg, scale: Scale) -> Self {
        let MemKind::Sib(sib) = &mut self.kind else {
            panic!()
        };
        sib.index = Some((index, scale));

        self
    }
    pub fn label(mut self, label: Label) -> Self {
        self.label = Some(label);
        self
    }
    pub fn size(mut self, size: OperandSize) -> Self {
        self.size = Some(size);
        self
    }
    pub fn offset(mut self, offset: i32) -> Self {
        self.offset = offset;
        self
    }
}
impl Add<u8> for Memory {
    type Output = Self;
    fn add(mut self, rhs: u8) -> Self::Output {
        self.offset = self.offset.wrapping_add_unsigned(rhs as u32);
        self
    }
}
impl Add<u16> for Memory {
    type Output = Self;
    fn add(mut self, rhs: u16) -> Self::Output {
        self.offset = self.offset.wrapping_add_unsigned(rhs as u32);
        self
    }
}
impl Add<u32> for Memory {
    type Output = Self;
    fn add(mut self, rhs: u32) -> Self::Output {
        self.offset = self.offset.wrapping_add_unsigned(rhs as u32);
        self
    }
}
impl Add<i8> for Memory {
    type Output = Self;
    fn add(mut self, rhs: i8) -> Self::Output {
        self.offset = self.offset.wrapping_add(rhs as i32);
        self
    }
}
impl Add<i16> for Memory {
    type Output = Self;
    fn add(mut self, rhs: i16) -> Self::Output {
        self.offset = self.offset.wrapping_add(rhs as i32);
        self
    }
}
impl Add<i32> for Memory {
    type Output = Self;
    fn add(mut self, rhs: i32) -> Self::Output {
        self.offset = self.offset.wrapping_add(rhs as i32);
        self
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum MemKind {
    Rip,
    Sib(Sib),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Sib {
    pub base: Option<Reg>,
    pub index: Option<(Reg, Scale)>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Scale {
    One,
    Two,
    Four,
    Eight,
}
impl Scale {
    pub fn numeric(self) -> u8 {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Four => 4,
            Self::Eight => 8,
        }
    }
}
