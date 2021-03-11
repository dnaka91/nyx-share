use yew::prelude::*;
use yewtil::NeqAssign;

pub struct CheckBox(Props);

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub label: String,
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_else(Callback::noop)]
    pub onchange: Callback<ChangeData>,
}

impl Component for CheckBox {
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
            <label class="checkbox">
                <input
                    type="checkbox"
                    checked=self.0.checked
                    onchange=self.0.onchange.clone()
                />
                {&self.0.label}
            </label>
        }
    }
}
