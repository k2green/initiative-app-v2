use yew::prelude::*;

use crate::pages::{welcome_page::WelcomePage, conflicts_page::ConflictsPage};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AppPage {
    WelcomePage,
    ConflictsPage,
    EncounterPage,
}

#[function_component(App)]
pub fn app() -> Html {
    let current_page = use_state_eq(|| AppPage::WelcomePage);

    match *current_page {
        AppPage::WelcomePage => render_welcome_page(current_page.clone()),
        AppPage::ConflictsPage => render_conflicts_page(current_page.clone()),
        AppPage::EncounterPage => render_encounter_page(current_page.clone())
    }
}

fn render_welcome_page(current_page: UseStateHandle<AppPage>) -> Html {
    html! {
        <WelcomePage current_page={current_page} />
    }
}

fn render_conflicts_page(current_page: UseStateHandle<AppPage>) -> Html {
    html! {
        <ConflictsPage current_page={current_page} />
    }
}

fn render_encounter_page(current_page: UseStateHandle<AppPage>) -> Html {
    html! {
        
    }
}