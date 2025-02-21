mod type_def;
mod trait_def;
mod lifttime;

pub use self::type_def::{get_largest, struct_type};

pub use self::trait_def::{trait_def,trait_as_param};

pub use self::trait_def::Summary;
pub use self::trait_def::Tweet;

pub use self::lifttime::{lifetime_demo};