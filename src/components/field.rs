use yew::prelude::*;
use yewtil::NeqAssign;

pub struct Field(Props);

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub label: String,
    pub children: Children,
}

impl Component for Field {
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
            <div class="field is-horizontal">
                <div class="field-label is-normal">
                    {&self.0.label}
                </div>
                <div class="field-body">
                    <div class="field is-narrow">
                        <div class="control">
                            {self.0.children.clone()}
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
