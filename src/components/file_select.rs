use web_sys::File;
use yew::prelude::*;
use yewtil::NeqAssign;

pub struct FileSelect(Props);

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub label: String,
    pub icon: String,
    #[prop_or_else(Callback::noop)]
    pub onchange: Callback<ChangeData>,
    pub file: Option<File>,
}

impl Component for FileSelect {
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
            <div class="file has-name is-boxed is-centered is-fullwidth">
                <label class="file-label">
                    <input
                        class="file-input"
                        type="file"
                        name="file"
                        onchange=self.0.onchange.clone()
                    />
                    <span class="file-cta">
                        <span class="file-icon">{&self.0.icon}</span>
                        <span class="file-label">{&self.0.label}</span>
                    </span>
                    <span class="file-name">
                        {self.0.file.as_ref().map(File::name).unwrap_or_default()}
                    </span>
                </label>
            </div>
        }
    }
}
