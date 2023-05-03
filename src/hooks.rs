pub mod creatures_hook;
pub mod conflicts_hook;
pub mod drag_and_drop_hook;
pub mod general_hooks;
pub mod sr_state_hook;

pub mod prelude {
    pub use crate::hooks::creatures_hook::*;
    pub use crate::hooks::conflicts_hook::*;
    pub use crate::hooks::drag_and_drop_hook::*;
    pub use crate::hooks::general_hooks::*;
    pub use crate::hooks::sr_state_hook::*;
}