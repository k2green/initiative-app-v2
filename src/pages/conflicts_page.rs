use yew::prelude::*;

use crate::{app::AppPage, hooks::prelude::*, glue::{move_initiative_conflict_with_callback, finalize_initiative_order_with_callback}};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ConflictsPageProps {
    pub current_page: UseStateHandle<AppPage>
}

#[function_component(ConflictsPage)]
pub fn conflicts_page(props: &ConflictsPageProps) -> Html {
    let ConflictsPageProps { current_page } = props.clone();
    let conflicts = use_conflicts(current_page.clone());

    let back = {
        let current_page = current_page.clone();
        Callback::from(move |_: MouseEvent| {
            current_page.set(AppPage::WelcomePage);
        })
    };

    let finish = {
        let current_page = current_page.clone();

        Callback::from(move |_: MouseEvent| {
            let current_page = current_page.clone();
            finalize_initiative_order_with_callback(move |_| {
                current_page.set(AppPage::EncounterPage);
            });
        })
    };

    let groups = conflicts.iter()
        .enumerate()
        .map(|(idx, _)| {
            let conflicts = conflicts.clone();
            html! {
                <ConflictGroupElement conflicts={conflicts} group_index={idx} />
            }
        })
        .collect::<Html>();

    html! {
        <div class="flex-row stretch">
            <main class="flex-column no-scroll">
                <div class="flex-grow-1 scroll-y">
                    <p>{"Your encounter has some creatures that have the same initiative value. You will need to confirm which order these creatures will take their turns in. Drag and drop the creatures to reorder them."}</p>
                    {groups}
                </div>
                <div class="flex-row button-group">
                    <button class="flex-grow-1" onclick={back}>{"Back"}</button>
                    <button class="flex-grow-1" onclick={finish}>{"Finish"}</button>
                </div>
            </main>
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct ConflictGroupElementProps {
    conflicts: UseConflictsHandle,
    group_index: usize
}

#[function_component(ConflictGroupElement)]
fn conflict_group_element(props: &ConflictGroupElementProps) -> Html {
    let ConflictGroupElementProps { conflicts, group_index } = props.clone();
    let is_open = use_state_eq(|| true);
    let toggle_open = {
        let is_open = is_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_open.set(!*is_open);
        })
    };

    let conflict_group = &conflicts[group_index];
    let drag_state = use_drag_and_drop({
        let conflicts = conflicts.clone();
        move |args: DropArguments| {
            let conflicts = conflicts.clone();
            move_initiative_conflict_with_callback(group_index, args.dragging_index, args.target_index, move |_| {
                conflicts.update();
            });
        }
    });

    let items = conflict_group.creatures()
        .iter()
        .enumerate()
        .map(|(idx, creature)| {
            let drag_state = drag_state.clone();

            html! {
                <tr>
                    <td ondragenter={drag_state.on_drag_enter(idx)} ondragover={drag_state.on_drag_over(idx)} ondrop={drag_state.on_drop(idx)} draggable="false">
                        <p ondragstart={drag_state.on_drag_start(idx)} ondragend={drag_state.on_drag_end(idx)} draggable="true">{creature.name()}</p>
                    </td>
                </tr>
            }
        })
        .collect::<Html>();

    html! {
        <div class="conflict-accordion flex-column">
            <button class="flex-row blank" onclick={toggle_open}>
                <h3 class="flex-grow-1">{format!("Creatures with initiative {}", conflict_group.initiative())}</h3>
                <h3>{"-"}</h3>
            </button>
            if *is_open {
                <table class="stretch-width">
                    {items}
                </table>
            }
        </div>
    }
}