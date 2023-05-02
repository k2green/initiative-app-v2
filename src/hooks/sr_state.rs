use std::ops::Deref;

use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct UseSrStateHandle {
    state: UseStateHandle<bool>
}

impl Deref for UseSrStateHandle {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        self.state.deref()
    }
}

impl UseSrStateHandle {
    pub fn set(&self) {
        self.state.set(true);
    }

    pub fn reset(&self) {
        self.state.set(false);
    }

    pub fn set_callback<T>(&self) -> Callback<T> {
        let self_clone = self.clone();
        Callback::from(move |_| {
            self_clone.set()
        })
    }

    pub fn reset_callback<T>(&self) -> Callback<T> {
        let self_clone = self.clone();
        Callback::from(move |_| {
            self_clone.reset()
        })
    }
}

#[hook]
pub fn use_sr_state(default_state: bool) -> UseSrStateHandle {
    let state = use_state(|| default_state);

    UseSrStateHandle { state }
}

#[hook]
pub fn use_sr_state_eq(default_state: bool) -> UseSrStateHandle {
    let state = use_state_eq(|| default_state);

    UseSrStateHandle { state }
}