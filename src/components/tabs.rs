use yew::prelude::*;
use yewtil::NeqAssign;

pub struct Tabs(Props);

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

impl Component for Tabs {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self(props)
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.0.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <div class="tabs is-toggle is-fullwidth">
                <ul>
                    {self.0.children.clone()}
                </ul>
            </div>
        }
    }
}

pub struct Tab(TabProps);

#[derive(Clone, PartialEq, Properties)]
pub struct TabProps {
    pub label: String,
    pub icon: String,
    pub selected: bool,
    pub onclick: Callback<MouseEvent>,
}

impl Component for Tab {
    type Message = ();
    type Properties = TabProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self(props)
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.0.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <li
                class=if self.0.selected { "is-active" } else{ "" }
                onclick=self.0.onclick.clone()
            >
                <a>
                    <span class="icon is-small">{&self.0.icon}</span>
                    <span>{&self.0.label}</span>
                </a>
            </li>
        }
    }
}
