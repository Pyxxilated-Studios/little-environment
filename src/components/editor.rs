use wasm_bindgen::JsCast;
use yew::prelude::*;

pub enum Messages {
    Input(String),
    Download,
}

pub struct Editor {
    link: ComponentLink<Self>,
    text: String,
    onchange: Callback<String>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub onchange: Callback<String>,
}

impl Component for Editor {
    type Message = Messages;
    type Properties = Props;

    fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            text: String::new(),
            onchange: properties.onchange,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Self::Message::Input(i) => {
                self.text = i;
                self.onchange.emit(self.text.clone());
            }
            Self::Message::Download => {
                let content = format!("data:text/plain;charset=utf-8,{}", self.text);

                let window = web_sys::window().expect("Unable to get window");
                let document = window.document().expect("Unable to get document");
                let save = document
                    .get_element_by_id("save-button")
                    .expect("Unable to find element")
                    .unchecked_into::<web_sys::HtmlAnchorElement>();

                save.set_href(&content);
                save.set_download("temp.asm");
                save.click();
            }
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div style="display: grid; grid-template-columns: 0 1fr; grid-template-rows: 1fr auto; overflow: auto">
                    <label for="code-area" style="display: none">{"Editor"}</label>
                    <textarea id="code-area" aria-label="editor" spellcheck="false"
                        oninput=self.link.callback(|s: InputData| Self::Message::Input(s.value)) value=self.text.clone()
                    />
                    <button onclick=self.link.callback(|_| Self::Message::Download) class="download-button">
                        {"Download"}
                    </button>
                    <a href="#save" id="save-button" style="display: none">{"Save"}</a>
                </div>
            </>
        }
    }
}
