use common_data_lib::creatures::Creature;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew_icons::{IconId, Icon};

use crate::{app::AppPage, components::menu::Menu, glue::{get_creatures_with_callback, remove_creature_with_callback, set_creature_selected_with_callback, add_creatures_with_callback, set_all_creatures_selected_with_callback}, hooks::{use_creatures, UseCreaturesHandle, use_sr_state_eq, UseSrStateHandle}};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct WelcomePageProps {
    pub current_page: UseStateHandle<AppPage>
}

#[function_component(WelcomePage)]
pub fn welcome_page(props: &WelcomePageProps) -> Html {
    let WelcomePageProps { current_page } = props;
    let creatures = use_creatures();
    let is_menu_open = use_state_eq(|| false);
    let is_add_creatures_modal_open = use_sr_state_eq(false);

    let open_modal = {
        let is_add_creatures_modal_open = is_add_creatures_modal_open.clone();
        Callback::from(move |_| {
            is_add_creatures_modal_open.set();
        })
    };

    html! {
        <div class="flex-row stretch">
            <Menu is_open={is_menu_open.clone()}>
                <h1>{"Test Menu"}</h1>
            </Menu>
            <AddCreaturesModal creatures={creatures.clone()} is_visible={is_add_creatures_modal_open.clone()} />
            <main class="no-scroll flex-column">
                <h1 class="heading">{"Welcome!"}</h1>
                <p>{"This tool can be used to help track the initiative order of creatures in your encounters."}</p>
                {render_creatures(creatures.clone())}
                <button class="stretch-width" onclick={open_modal}>{"+"}</button>
            </main>
        </div>
    }
}

fn render_creatures(creatures: UseCreaturesHandle) -> Html {
    if creatures.is_empty() {
        render_empty_creatures()
    } else {
        render_non_empty_creatures(creatures)
    }
}

fn render_empty_creatures() -> Html {
    html! {
        <div class="flex-grow-1 scroll-y">
            <p>{"There are no creatures in this encounter. To begin, use the '+' button below to add some creatures to your encounter."}</p>
        </div>
    }
}

fn render_non_empty_creatures(creatures: UseCreaturesHandle) -> Html {
    html! {
        <>
            <SelectAllControl creatures={creatures.clone()} />
            <div class="flex-grow-1 scroll-y">
                {get_creatures_list(creatures.clone())}
            </div>
        </>
    }
}

fn get_creatures_list(creatures: UseCreaturesHandle) -> Html {
    creatures.iter()
        .map(|c| {
            let creatures = creatures.clone();

            html! {
                <CreatureListing creature={c.clone()} update={creatures.update_callback()} />
            }
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct CreatureListingProps {
    pub creature: Creature,
    pub update: Callback<()>
}

#[function_component(CreatureListing)]
fn creature_listing(props: &CreatureListingProps) -> Html {
    let CreatureListingProps { creature, update } = props;
    let hover_remove_state = use_sr_state_eq(false);

    let remove_creature = {
        let hover_remove_state = hover_remove_state.clone();
        let update = update.clone();
        let id = creature.id();
        Callback::from(move |_: MouseEvent| {
            let hover_remove_state = hover_remove_state.clone();
            let update = update.clone();
            remove_creature_with_callback(id, move |c: Creature| {
                log::info!("Removed '{}'", c.name());
                update.emit(());
                hover_remove_state.reset();
            });
        })
    };

    let set_selected = {
        let update = update.clone();
        let id = creature.id();
        Callback::from(move |e: Event| {
            let update = update.clone();
            let target: HtmlInputElement = e.target_unchecked_into();
            let new_value = target.checked();
            set_creature_selected_with_callback(id, new_value, move |_: ()| {
                log::info!("Set creature selected state to {}", new_value);
                update.emit(());
            });
        })
    };

    let on_mouse_over = {
        let hover_remove_state = hover_remove_state.clone();
        Callback::from(move |_: MouseEvent| {
            hover_remove_state.set();
        })
    };

    let on_mouse_out = {
        let hover_remove_state = hover_remove_state.clone();
        Callback::from(move |_: MouseEvent| {
            hover_remove_state.reset();
        })
    };

    html! {
        <div class="flex-row list-item">
            <input type="checkbox" checked={creature.selected()} onchange={set_selected} />
            <p class="flex-grow-1">{creature.name()}</p>
            <button class="blank" onclick={remove_creature} onmouseover={on_mouse_over} onmouseout={on_mouse_out}>
                <Icon class="fill-color" icon_id={if *hover_remove_state {IconId::BootstrapDashCircleFill} else {IconId::BootstrapDashCircle}} width="15px" height="15px" />
            </button>
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct SelectAllControlProps {
    creatures: UseCreaturesHandle
}

#[function_component(SelectAllControl)]
fn select_all_control(props: &SelectAllControlProps) -> Html {
    let SelectAllControlProps { creatures } = props.clone();
    let set_all_selected = {
        let creatures = creatures.clone();
        Callback::from(move |e: Event| {
            let creatures = creatures.clone();
            let target: HtmlInputElement = e.target_unchecked_into();
            set_all_creatures_selected_with_callback(target.checked(), move |_| {
                creatures.update();
            });
        })
    };

    let all_selected = creatures.are_all_selected();

    html! {
        <div class="flex-row">
            <input type="checkbox" checked={all_selected} onchange={set_all_selected} disabled={creatures.is_empty()} />
            <p class="no-margin">{ if all_selected {"Deselect all"} else {"Select all"}}</p>
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct AddCreaturesModalProps {
    creatures: UseCreaturesHandle,
    is_visible: UseSrStateHandle
}

#[function_component(AddCreaturesModal)]
fn add_creatures_modal(props: &AddCreaturesModalProps) -> Html {
    let AddCreaturesModalProps { creatures, is_visible } = props.clone();
    let creatures_text = use_state_eq(|| String::new());
    let update_text = {
        let creatures_text = creatures_text.clone();
        Callback::from(move |e: Event| {
            let target: HtmlTextAreaElement = e.target_unchecked_into();
            creatures_text.set(target.value());
        })
    };

    let add_creatures = {
        let is_visible = is_visible.clone();

        Callback::from(move |_: MouseEvent| {
            let creatures = creatures.clone();
            let is_visible = is_visible.clone();
            let creatures_text = creatures_text.clone();
            add_creatures_with_callback((*creatures_text).clone(), move |_| {
                creatures.update();
                is_visible.reset();
            });
        })
    };

    let cancel = {
        let is_visible = is_visible.clone();
        Callback::from(move |_: MouseEvent| {
            is_visible.reset();
        })
    };

    let content_html = html! {
        <div class="overlay flex-column center-main-axis center-cross-axis">
            <div class="modal flex-column">
                <p>{"This is where you can add new creatures to your encounter. Multiple creatures can be added by splitting names onto new lines."}</p>
                <textarea onchange={update_text}>{&*creatures_text}</textarea>
                <div class="flex-row">
                    <button class="flex-grow-1 space-right" onclick={add_creatures}>{"Add Creatures"}</button>
                    <button class="flex-grow-1 space-left" onclick={cancel}>{"Cancel"}</button>
                </div>
            </div>
        </div>
    };

    html! {
        if *is_visible {
            {content_html}
        }
    }
}