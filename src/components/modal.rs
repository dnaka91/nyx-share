use yew::prelude::*;
use yewtil::NeqAssign;

pub struct Modal(Props);

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub visible: bool,
}

impl Component for Modal {
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
            <div class=if self.0.visible { "modal is-active" } else { "modal" }>
                <div class="modal-background"></div>
                <div class="modal-content">
                    <div class="block">
                        <span class="is-size-4">{"Uploading..."}</span>
                    </div>
                    <progress class="progress is-link is-large" max=100></progress>
                </div>
            </div>
        }
    }
}
