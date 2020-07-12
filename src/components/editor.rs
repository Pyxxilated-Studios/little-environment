use yew::prelude::*;

pub enum Messages {
    Input(String),
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
                <label for="code-area" style="display: none;">{"Editor"}</label>
                <textarea id="code-area" class="pure-u-21-24 shadow bordered" aria-label="editor" spellcheck="false"
                    oninput=self.link.callback(|s: InputData| Self::Message::Input(s.value)) value=self.text />
            </>
        }
    }
}
