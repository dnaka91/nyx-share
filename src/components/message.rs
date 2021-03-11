use yew::prelude::*;
use yewtil::NeqAssign;

pub struct Message(Props);

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub content: String,
}

impl Component for Message {
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
        if self.0.content.is_empty() {
            html! {}
        } else {
            html! {
                <div class="message is-link">
                    <div class="message-body">
                        { &self.0.content }
                    </div>
                </div>
            }
        }
    }
}
