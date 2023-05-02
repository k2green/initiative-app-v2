pub mod creatures_hook;
pub mod sr_state;

pub mod prelude {
    pub use crate::hooks::creatures_hook::*;
    pub use crate::hooks::sr_state::*;
}