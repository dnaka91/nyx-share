use yew::prelude::*;
use yewtil::NeqAssign;

pub struct Button(Props);

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub label: String,
    #[prop_or_default]
    pub icon: Option<String>,
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
}

impl Component for Button {
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
        let icon = if let Some(icon) = self.0.icon.as_deref() {
            html! { <span class="icon is-small">{icon}</span> }
        } else {
            html! {}
        };

        html! {
            <button
                class="button is-link"
                onclick=self.0.onclick.clone()
            >
                {icon}
                <span>{&self.0.label}</span>
            </button>
        }
    }
}
