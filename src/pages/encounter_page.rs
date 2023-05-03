use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::{app::AppPage, hooks::prelude::*, glue::{change_active_encounter_order_with_callback, add_creatures_to_active_encounter_with_callback, remove_from_active_encounter_with_callback}, components::modal::Modal};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct EncounterPageProps {
    pub current_page: UseStateHandle<AppPage>
}

#[function_component(EncounterPage)]
pub fn encounter_page(props: &EncounterPageProps) -> Html {
    let EncounterPageProps { current_page } = props.clone();
    let creatures = use_encounter_creatures();
    let is_modal_open = use_state_eq(|| false);
    let drag_state = use_drag_and_drop({
        let creatures = creatures.clone();
        move |args: DropArguments| {
            let creatures = creatures.clone();
            change_active_encounter_order_with_callback(args.dragging_index, args.target_index, move |_| {
                creatures.update();
            });
        }
    });

    let open_modal = {
        let is_modal_open = is_modal_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_modal_open.set(true);
        })
    };

    let finish = {
        let current_page = current_page.clone();

        Callback::from(move |_: MouseEvent| {
            current_page.set(AppPage::WelcomePage);
        })
    };

    let creature_items = creatures.iter()
        .enumerate()
        .filter_map(|(idx, c)| {
            if c.selected() {
                let creatures = creatures.clone();
                let drag_state = drag_state.clone();
                Some(html! {
                    <EncounterCreatureListing drag_state={drag_state} creatures={creatures} creature_index={idx} />
                })  
            } else {
                None
            }
        })
        .collect::<Html>();

    html! {
        <>
            <AddCreaturesModal creatures={creatures} is_open={is_modal_open} />
            <div class="flex-row stretch">
                <main class="flex-column no-scroll">
                    <div class="flex-grow-1 scroll-y">
                        <table class="encounter-table stretch-width">
                        {creature_items}
                        </ table>
                    </div>
                    <div class="flex-row button-group">
                        <button class="flex-grow-1" onclick={open_modal}>{"Add creatures"}</button>
                        <button class="flex-grow-1" onclick={finish}>{"Finish encounter"}</button>
                    </div>
                </main>
            </div>
        </>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct EncounterCreatureListingProps {
    drag_state: UseDragAndDropHandle,
    creatures: UseCreaturesHandle,
    creature_index: usize,
}

#[function_component(EncounterCreatureListing)]
fn encounter_creature_listing(props: &EncounterCreatureListingProps) -> Html {
    let EncounterCreatureListingProps { drag_state, creatures, creature_index } = props.clone();
    let hover_remove_state = use_sr_state_eq(false);
    let creature = &creatures[creature_index];

    let DragAndDropCallbacks {
        on_drag_start,
        on_drag_over,
        on_drag_enter,
        on_drag_end,
        on_drop,
        on_drag_leave: _,
    } = drag_state.callbacks(creature_index);

    let remove_creature = {
        let creatures = creatures.clone();
        let id = creature.id();
        Callback::from(move |_: MouseEvent| {
            let creatures = creatures.clone();

            remove_from_active_encounter_with_callback(id, move |_| {
                creatures.update();
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
        <tr>
            <td ondragover={on_drag_over} ondragenter={on_drag_enter} ondrop={on_drop} draggable="false">
                <div ondragend={on_drag_end} ondragstart={on_drag_start} class="flex-row" draggable="true">
                    <p class="flex-grow-1">{creature.name()}</p>
                    <button class="blank" onclick={remove_creature} onmouseover={on_mouse_over} onmouseout={on_mouse_out}>
                        <Icon class="fill-color" icon_id={if *hover_remove_state {IconId::BootstrapDashCircleFill} else {IconId::BootstrapDashCircle}} width="15px" height="15px" />
                    </button>
                </div>
            </td>
        </tr>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct AddCreaturesModalProps {
    creatures: UseCreaturesHandle,
    is_open: UseStateHandle<bool>
}

#[function_component(AddCreaturesModal)]
fn add_creatures_modal(props: &AddCreaturesModalProps) -> Html {
    let AddCreaturesModalProps { creatures, is_open } = props.clone();
    let creatures_text = use_state_eq(|| String::new());
    let update_text = {
        let creatures_text = creatures_text.clone();
        Callback::from(move |e: Event| {
            let target: HtmlInputElement = e.target_unchecked_into();
            creatures_text.set(target.value()); 
        })
    };

    let add_creatures = {
        let creatures_text = creatures_text.clone();
        let creatures = creatures.clone();
        let is_open = is_open.clone();
        Callback::from(move |_: MouseEvent| {
            let creatures = creatures.clone();
            let is_open = is_open.clone();

            add_creatures_to_active_encounter_with_callback(&*creatures_text, move |_| {
                creatures.update();
                is_open.set(false);
            });
        })
    };

    let cancel = {
        let is_open = is_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_open.set(false);
        })
    };

    let modal_html = html! {
        <Modal>
            <p>{"You can add new creatures to the encounter. You can add multiple creatures by separating them onto new lines."}</p>
            <p>{"Creatures will be added at the end of the round in the order that they are entered."}</p>
            <textarea onchange={update_text}>{&*creatures_text}</textarea>
            <div class="flex-row button-group">
                <button class="flex-grow-1" onclick={add_creatures}>{"Add Creatures"}</button>
                <button class="flex-grow-1" onclick={cancel}>{"Cancel"}</button>
            </div>
        </Modal>
    };

    html! {
        if *is_open {
            {modal_html}
        }
    }
}