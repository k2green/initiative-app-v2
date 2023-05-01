use std::{ops::Deref, rc::Rc, sync::Arc};

use common_data_lib::creatures::Creature;
use yew::prelude::*;

use crate::glue::get_creatures_with_callback;

#[derive(Debug, Clone)]
pub struct UseCreaturesHandle {
    creatures: UseStateHandle<Vec<Creature>>,
    force_update_state: UseStateHandle<bool>
}

impl PartialEq for UseCreaturesHandle {
    fn eq(&self, other: &Self) -> bool {
        self.creatures == other.creatures
    }
}

impl Deref for UseCreaturesHandle {
    type Target = Vec<Creature>;

    fn deref(&self) -> &Self::Target {
        self.creatures.deref()
    }
}

impl UseCreaturesHandle {
    pub fn update(&self) {
        self.force_update_state.set(!*self.force_update_state)
    }

    pub fn update_callback<T>(&self) -> Callback<T> {
        let self_clone = self.clone();
        Callback::from(move |_| {
            self_clone.update();
        })
    }

    pub fn are_all_selected(&self) -> bool {
        self.creatures.len() > 0 && self.creatures.iter().all(|c| c.selected())
    }

    pub fn is_empty(&self) -> bool {
        self.creatures.is_empty()
    }
}

#[hook]
pub fn use_creatures() -> UseCreaturesHandle {
    let creatures = use_state_eq(|| Vec::new());
    let force_update_state = use_state_eq(|| false);

    use_effect_with_deps({
        let creatures_state = creatures.clone();
        move |_| {
            log::info!("Getting creatures");
            get_creatures_with_callback(Callback::from(move |creatures| {
                creatures_state.set(creatures);
            }));
        }
    }, force_update_state.clone());

    UseCreaturesHandle { creatures, force_update_state }
}

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