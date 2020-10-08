//! Processor core registers

//#![rustfmt::skip]

#[macro_use]
mod macros;

mod lr;
mod sp;
mod cpsr;

// Export only the R/W traits and the static reg definitions
pub use register::cpu::*;

pub use self::lr::LR;
pub use self::sp::SP;
pub use self::cpsr::CPSR;
