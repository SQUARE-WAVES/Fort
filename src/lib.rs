pub mod traits;
pub use traits::Fort;

pub mod text;
pub use text::*;
pub mod values;
pub use values::*;
pub mod frame_stack;
pub use frame_stack::FrameStack;
pub mod tokens;
pub mod dictionary;
pub use dictionary::*;
pub mod functions;
pub use functions::F;
pub mod parser;
pub mod bifs;
pub mod vm;
pub use vm::Thread;
