mod call_frame;
pub mod error;
pub mod vm;

pub use error::Error;
pub use error::Result;

pub use vm::Vm;

use call_frame::CallFrame;
