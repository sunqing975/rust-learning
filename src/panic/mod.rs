mod panic;
mod result;

pub use self::panic::{panic_def};

pub use self::result::{result_def, result_unwrap, result_expect,result_propagating_panic};