use std::ops::Deref;

use common_data_lib::creatures::ConflictGroup;
use yew::prelude::*;

use crate::{app::AppPage, glue::{get_initiative_conflicts_with_callback, finalize_initiative_order_with_callback}};

#[derive(Debug, Clone)]
pub struct UseConflictsHandle {
    conflicts: UseStateHandle<Vec<ConflictGroup>>,
    update_state: UseStateHandle<bool>
}

impl PartialEq for UseConflictsHandle {
    fn eq(&self, other: &Self) -> bool {
        self.conflicts == other.conflicts
    }
}

impl Deref for UseConflictsHandle {
    type Target = Vec<ConflictGroup>;

    fn deref(&self) -> &Self::Target {
        &*self.conflicts
    }
}

impl UseConflictsHandle {
    pub fn update(&self) {
        self.update_state.set(true);
    }
}

#[hook]
pub fn use_conflicts(app_page: UseStateHandle<AppPage>) -> UseConflictsHandle {
    let conflicts = use_state_eq(|| Vec::new());
    let is_first_state = use_state(|| true);
    let update_state = use_state(|| true);

    use_effect({
        let is_first_state = is_first_state.clone();
        let update_state = update_state.clone();
        let conflicts = conflicts.clone();
        let app_page = app_page.clone();
        move || {
            let app_page = app_page.clone();

            if *update_state {
                get_initiative_conflicts_with_callback(*is_first_state, move |new_conflicts: Vec<ConflictGroup>| {
                    let app_page = app_page.clone();
                    
                    if *is_first_state && new_conflicts.is_empty() {
                        finalize_initiative_order_with_callback(move |_| {
                            app_page.set(AppPage::EncounterPage);
                        });
                    } else {
                        conflicts.set(new_conflicts);
                        is_first_state.set(false);
                        update_state.set(false);
                    }
                });
            }
        }
    });

    UseConflictsHandle { conflicts, update_state }
}