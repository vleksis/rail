use std::fmt::Display;

macro_rules! define_opcodes {
   ( $( $name:ident = $val:expr, )* ) => {
       #[repr(u8)]
       #[derive(Clone, Copy, Debug, PartialEq, Eq)]
       pub enum OpCode {
           $( $name = $val, )*
       }

       impl OpCode {
           pub fn from_byte(byte: u8) -> Option<Self> {
               match byte {
                   $( $val => Some(OpCode::$name), )*
                   _ => None,
               }
           }

           pub fn name(&self) -> &'static str {
               match self {
                   $( OpCode::$name => stringify!($name), )*
               }
           }

           pub fn to_byte(self) -> u8 {
                self as u8
           }
       }
   }
}

define_opcodes! {
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
    I64Equal = 34,
    I64NotEqual = 35,
    I64Less = 36,
    I64LessEqual = 37,
    I64Greater = 38,
    I64GreaterEqual = 39,

    U64Add = 40,
    U64Sub = 41,
    U64Mul = 42,
    U64Div = 43,
    U64Equal = 44,
    U64NotEqual = 45,
    U64Less = 46,
    U64LessEqual = 47,
    U64Greater = 48,
    U64GreaterEqual = 49,

    F64Add = 50,
    F64Sub = 51,
    F64Mul = 52,
    F64Div = 53,
    F64Equal = 54,
    F64NotEqual = 55,
    F64Less = 56,
    F64LessEqual = 57,
    F64Greater = 58,
    F64GreaterEqual = 59,

    BoolNot = 80,

    Pop = 90,
    Return = 91,
    Call = 92,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#04x} {:<12}", *self as u8, self.name())
    }
}
