use yew::{prelude::*, html::ChildrenRenderer, virtual_dom::VNode};
use yew_icons::{Icon, IconId};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MenuProps {
    pub is_open: UseStateHandle<bool>,
    pub children: Children
}

#[function_component(Menu)]
pub fn menu(props: &MenuProps) -> Html {
    let MenuProps { is_open, children } = props.clone();

    let set_menu_open = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(true);
        })
    };

    let set_menu_closed = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(false);
        })
    };

    html! {
        <>
            {render_menu_closed(set_menu_open)}
            if *is_open {
                {render_menu_open(set_menu_closed, children)}
            }
        </>
    }
}

fn render_menu_closed(set_menu_open: Callback<MouseEvent>) -> Html {
    html! {
        <div class="menu">
            <button class="blank" onclick={set_menu_open}>
                <Icon class="margin fill-color" icon_id={IconId::BootstrapList} width="20px" height="20px" />
            </button>
        </div>
    }
}

fn render_menu_open(set_menu_closed: Callback<MouseEvent>, children: ChildrenRenderer<VNode>) -> Html {
    html! {
        <div class="overlay flex-row">
            <div class="menu flex-row">
                <div class="menu-content flex-column flex-grow-1">
                    {children}
                </div>
                <div class="menu">
                    <button class="blank" onclick={set_menu_closed.clone()}>
                        <Icon class="margin fill-color" icon_id={IconId::BootstrapList} width="20px" height="20px" />
                    </button>
                </div>
            </div>
            <div class="empty" onclick={set_menu_closed} />
        </div>
    }
}