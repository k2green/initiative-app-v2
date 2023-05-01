use yew::prelude::*;

use crate::pages::welcome_page::WelcomePage;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AppPage {
    WelcomePage
}

#[function_component(App)]
pub fn app() -> Html {
    let current_page = use_state_eq(|| AppPage::WelcomePage);

    match *current_page {
        AppPage::WelcomePage => render_welcome_page(current_page.clone())
    }
}

fn render_welcome_page(current_page: UseStateHandle<AppPage>) -> Html {
    html! {
        <WelcomePage current_page={current_page} />
    }
}
