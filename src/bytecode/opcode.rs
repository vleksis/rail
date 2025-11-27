use std::fmt::Display;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpCode {
    Const = 0,
    True = 1,
    False = 2,

    GetLocal = 10,
    SetLocal = 11,
    GetGlobal = 12,
    SetGlobal = 13,
    DefineGlobal = 14,

    Jump = 20,
    JumpIfFalse = 21,
    Loop = 22,

    I64Add = 30,
    I64Sub = 31,
    I64Mul = 32,
    I64Div = 33,

    U64Add = 40,
    U64Sub = 41,
    U64Mul = 42,
    U64Div = 43,

    F64Add = 50,
    F64Sub = 51,
    F64Mul = 52,
    F64Div = 53,

    I64Eq = 60,
    I64Lt = 61,
    I64Gt = 62,

    F64Eq = 70,
    F64Lt = 71,
    F64Gt = 72,

    BoolNot = 80,

    Pop = 90,
    Return = 91,
    Call = 92,
}

impl OpCode {
    pub fn from_byte(byte: u8) -> Option<OpCode> {
        // TODO(vleksis): add all opcodes
        let op = match byte {
            0 => OpCode::Const,
            1 => OpCode::True,
            2 => OpCode::False,

            10 => OpCode::GetLocal,
            11 => OpCode::SetLocal,
            12 => OpCode::GetGlobal,
            13 => OpCode::SetGlobal,
            14 => OpCode::DefineGlobal,

            20 => OpCode::Jump,
            21 => OpCode::JumpIfFalse,
            22 => OpCode::Loop,

            30 => OpCode::I64Add,
            31 => OpCode::I64Sub,
            32 => OpCode::I64Mul,
            33 => OpCode::I64Div,

            80 => OpCode::BoolNot,

            90 => OpCode::Pop,
            91 => OpCode::Return,
            92 => OpCode::Call,

            _ => return None,
        };

        Some(op)
    }

    pub fn name(&self) -> &'static str {
        match self {
            OpCode::Const => "Const",
            OpCode::True => "True",
            OpCode::False => "False",

            OpCode::GetLocal => "GetLocal",
            OpCode::SetLocal => "SetLocal",
            OpCode::GetGlobal => "GetGlobal",
            OpCode::SetGlobal => "SetGlobal",
            OpCode::DefineGlobal => "DefineGlobal",

            OpCode::Jump => "Jump",
            OpCode::JumpIfFalse => "JumpIfFalse",
            OpCode::Loop => "Loop",

            OpCode::I64Add => "I64Add",
            OpCode::I64Sub => "I64Sub",
            OpCode::I64Mul => "I64Mul",
            OpCode::I64Div => "I64Div",

            OpCode::U64Add => "U64Add",
            OpCode::U64Sub => "U64Sub",
            OpCode::U64Mul => "U64Mul",
            OpCode::U64Div => "U64Div",

            OpCode::F64Add => "F64Add",
            OpCode::F64Sub => "F64Sub",
            OpCode::F64Mul => "F64Mul",
            OpCode::F64Div => "F64Div",

            OpCode::I64Eq => "I64Eq",
            OpCode::I64Lt => "I64Lt",
            OpCode::I64Gt => "I64Gt",

            OpCode::F64Eq => "F64Eq",
            OpCode::F64Lt => "F64Lt",
            OpCode::F64Gt => "F64Gt",

            OpCode::BoolNot => "BoolNot",

            OpCode::Pop => "Pop",
            OpCode::Return => "Return",
            OpCode::Call => "Call",
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#04x} {:<12}", *self as u8, self.name())
    }
}
