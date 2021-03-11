use web_sys::File;
use yew::{
    events::ChangeData,
    format::Binary,
    prelude::*,
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
};

use crate::{
    components::{Button, CheckBox, Field, FileSelect, Message, Modal, Tab, Tabs},
    languages,
    multipart::MultiPart,
};

pub struct App {
    link: ComponentLink<Self>,
    input: String,
    file: Option<File>,
    public: bool,
    protected: bool,
    no_preview: bool,
    language: usize,
    content: String,
    task: Option<FetchTask>,
    tab: SelectedTab,
    modal: bool,
}

pub enum Msg {
    SwitchTab(SelectedTab),
    InputChanged(String),
    FileSelected(ChangeData),
    TogglePublic,
    ToggleProtected,
    ToggleNoPreview,
    LanguageSelected(ChangeData),
    Submit,
    SubmitResult(Result<String, String>),
}

#[derive(PartialEq, Eq)]
pub enum SelectedTab {
    Text,
    File,
}

impl App {
    fn multipart<'a>(
        input: &'a str,
        public: bool,
        protected: bool,
        no_preview: bool,
        language: &'a str,
    ) -> MultiPart<'a> {
        let mut mp = MultiPart::new();
        mp.add("file", input);

        if public {
            mp.add("public", "on");
        }

        if protected {
            mp.add("protected", "on");
        }

        if no_preview {
            mp.add("no_preview", "on");
        }

        mp.add("language", language);
        mp
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            input: String::new(),
            file: None,
            public: true,
            protected: false,
            no_preview: false,
            language: languages::PLAIN_TEXT_INDEX,
            content: String::new(),
            task: None,
            tab: SelectedTab::Text,
            modal: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SwitchTab(tab) => {
                if self.tab == tab {
                    return false;
                }

                self.tab = tab;
            }
            Msg::InputChanged(input) => self.input = input,
            Msg::FileSelected(data) => {
                if let ChangeData::Files(list) = data {
                    self.file = list.get(0);
                } else {
                    return false;
                }
            }
            Msg::TogglePublic => self.public = !self.public,
            Msg::ToggleProtected => self.protected = !self.protected,
            Msg::ToggleNoPreview => self.no_preview = !self.no_preview,
            Msg::LanguageSelected(data) => {
                if let ChangeData::Select(element) = data {
                    self.language = element.selected_index() as usize;
                } else {
                    return false;
                }
            }
            Msg::Submit => {
                if self.input.is_empty() && self.file.is_none() {
                    self.content = "NO CONTENT!".to_owned();
                    return true;
                }

                self.modal = true;

                let mp = Self::multipart(
                    &self.input,
                    self.public,
                    self.protected,
                    self.no_preview,
                    languages::LIST[self.language].0,
                );

                let callback = self.link.callback(|resp: Response<Binary>| {
                    if resp.status().is_success() {
                        Msg::SubmitResult(Ok(String::from_utf8(resp.into_body().unwrap()).unwrap()))
                    } else {
                        Msg::SubmitResult(Err(resp.status().to_string()))
                    }
                });

                let req = Request::post("https://share.nyx.xyz/upload")
                    .header("content-type", mp.content_type())
                    .body(Ok(mp.build()))
                    .unwrap();

                match FetchService::fetch_binary(req, callback) {
                    Ok(task) => self.task = Some(task),
                    Err(e) => self
                        .link
                        .send_message(Msg::SubmitResult(Err(e.to_string()))),
                }
            }
            Msg::SubmitResult(res) => {
                self.modal = false;

                match res {
                    Ok(content) => {
                        self.task = None;
                        self.content = format!("SUCCESS: {}", content);
                    }
                    Err(content) => {
                        self.task = None;
                        self.content = format!("ERROR: {}", content);
                    }
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let input = if self.tab == SelectedTab::Text {
            html! {
                <textarea
                    class="textarea"
                    rows="10"
                    placeholder="Enter text to upload..."
                    oninput=self.link.callback(|e: InputData| Msg::InputChanged(e.value))
                ></textarea>
            }
        } else {
            html! {
                <FileSelect
                    label="Choose a file..."
                    icon="📤"
                    file=&self.file
                    onchange=self.link.callback(|value: ChangeData| Msg::FileSelected(value))
                />
            }
        };

        html! {
            <>
            <section class="section">
                <div class="container">
                    <Message content=&self.content />
                    <div class="block">
                        <h1 class="title">{"Nyx Share"}</h1>
                        <p class="subtitle">{"Share code and files with your friends!"}</p>
                    </div>
                    <div class="block">
                        <Tabs>
                            <Tab
                                label="Text"
                                icon="📝"
                                selected=self.tab == SelectedTab::Text
                                onclick=self.link.callback(|_| Msg::SwitchTab(SelectedTab::Text))
                            />
                            <Tab
                                label="File"
                                icon="📁"
                                selected=self.tab == SelectedTab::File
                                onclick=self.link.callback(|_| Msg::SwitchTab(SelectedTab::File))
                            />
                        </Tabs>
                        { input }
                    </div>
                    <div class="block">
                        <Field>
                            <CheckBox
                                label=" Public - Add this content to the public repository."
                                checked=self.public
                                onchange=self.link.callback(|_| Msg::TogglePublic)
                            />
                        </Field>
                        <Field>
                            <CheckBox
                                label=" Protected - Lock access to the content with a random password."
                                checked=self.protected
                                onchange=self.link.callback(|_| Msg::ToggleProtected)
                            />
                        </Field>
                        <Field>
                            <CheckBox
                                label=" No preview - Don't show the content in the browser and force a direct download instead."
                                checked=self.no_preview
                                onchange=self.link.callback(|_| Msg::ToggleNoPreview)
                            />
                        </Field>
                        <Field label="Language">
                            <div class="select">
                                <select onchange=self.link.callback(|value: ChangeData| Msg::LanguageSelected(value))>
                                    {
                                        for languages::LIST.iter().enumerate().map(|(i, (value, name))| html! {
                                            <option value=value selected={i == self.language}>{name}</option>
                                        })
                                    }
                                </select>
                            </div>
                        </Field>
                        <Field>
                            <Button
                                label="Submit"
                                icon="✉️"
                                onclick=self.link.callback(|_| Msg::Submit)
                            />
                        </Field>
                    </div>
                </div>
            </section>
            <Modal visible=self.modal />
            </>
        }
    }
}
