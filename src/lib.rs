#![recursion_limit = "256"]

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate console_error_panic_hook;
extern crate log;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use lc3lib::assembler::Assembler;
use lc3lib::notifier;

pub mod components;

use components::editor::Editor;

struct Model {
    link: ComponentLink<Self>,
    assembled: String,
}

static NOTIFIER_NAME: &'static str = "Online Assembler";

enum Msg {
    Assemble(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            assembled: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Assemble(code) => {
                Assembler::from_string(code)
                    .assemble(false)
                    .and_then(|(_, _, tokens)| {
                        notifier::notifications()
                            .iter()
                            .for_each(|warning| self.assembled.push_str(&format!("{}", warning)));
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

                notifier::clear(Some(&NOTIFIER_NAME));
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
                <Editor onchange=self.link.callback(|code| Msg::Assemble(code)) />
                <span class="pure-u-1-24" />
                <label for="assembler-output-pane" style="display: none;">{"Assembler Output Pane"}</label>
                <textarea id="assembler-output-pane" class="pure-u-lg-10-24 pure-u-22-24" aria-label="output pane" spellcheck="false" readonly=true value=self.assembled />
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

    notifier::register(
        NOTIFIER_NAME.to_owned(),
        notifier::Notifier::Stringify(Vec::new()),
    );

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
