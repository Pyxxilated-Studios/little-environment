#![recursion_limit = "256"]

#[macro_use]
extern crate log;
extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use lc3lib::assembler::Assembler;
use lc3lib::notifier;

struct Model {
    link: ComponentLink<Self>,
    text: String,
    assembled: String,
}

enum Msg {
    Assemble,
    Input(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            text: String::new(),
            assembled: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Assemble => {
                notifier::push(notifier::Notifier::Stringify(Vec::new()));

                Assembler::from_string(self.text.clone())
                    .assemble(false)
                    .and_then(|(_, _, tokens)| {
                        self.assembled = String::new();
                        tokens
                            .iter()
                            .for_each(|(_, s)| self.assembled.push_str(&format!("{}\n", s)));
                        Some(())
                    })
                    .or_else(|| {
                        self.assembled = String::from("There were errors during assembly:\n");
                        notifier::notifications()
                            .iter()
                            .for_each(|error| self.assembled.push_str(&format!("{}", error)));
                        Some(())
                    });
                info!("Done!");

                notifier::clear();
            }
            Msg::Input(i) => {
                self.text = i;
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
            <div class="pure-g" style="height: 100%;">
                <span class="pure-u-1-24" />
                <label for="code-area" style="display: none;">{"Editor"}</label>
                <textarea id="code-area" class="pure-u-10-24" spellcheck="false" oninput=self.link.callback(|s: InputData| Msg::Input(s.value)) value=self.text />
                <div class="pure-u-2-24">
                    <button onclick=self.link.callback(|_| Msg::Assemble)>{"Assemble!"}</button>
                </div>
                <label for="assembler-output-pane" style="display: none;">{"Assembler Output Pane"}</label>
                <textarea id="assembler-output-pane" class="pure-u-10-24" spellcheck="false" readonly=true value=self.assembled />
                <span class="pure-u-1-24" />
            </div>
        }
    }
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(start)]
pub fn run_app() {
    init_panic_hook();
    wasm_logger::init(wasm_logger::Config::default());
    App::<Model>::new().mount_to_body();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
