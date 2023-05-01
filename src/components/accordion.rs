use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct AccordionProps {
    pub title: String,
    #[prop_or_default]
    pub children: Children
}

#[function_component(Accordion)]
pub fn accordion(props: &AccordionProps) -> Html {
    let AccordionProps { title, children } = props;
    let is_open = use_state_eq(|| false);
    let toggle_is_open = {
        let is_open = is_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_open.set(!*is_open);
        })
    };

    html! {
        <div class="accordion flex-column">
            <button class="blank" onclick={toggle_is_open}>
                <div class="flex-row">
                    <h3 class="flex-grow-1">{title}</h3>
                    <p>{"-"}</p>
                </div>
            </button>
            if *is_open {
                {children.clone()}
            }
        </div>
    }
}