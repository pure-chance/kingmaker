pub mod core;
pub mod methods;
pub mod preferences;
pub mod tactics;

pub mod prelude {
    pub use crate::core::*;
    pub use crate::methods;
    pub use crate::preferences;
    pub use crate::tactics;
}
