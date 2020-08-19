#![recursion_limit = "512"]

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate console_error_panic_hook;
extern crate log;

use wasm_bindgen::prelude::*;
use yew::prelude::{html, App, Component, ComponentLink, Html, ShouldRender};

use lc3lib::assembler::Assembler;
use lc3lib::notifier;

pub mod components;

use components::editor::Editor;
use components::navigation::{urlfor, NavBar};

struct Model {
    link: ComponentLink<Self>,
    assembled: String,
}

static NOTIFIER_NAME: &str = "Online Assembler";

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
            Self::Message::Assemble(code) => {
                Assembler::from_string(code)
                    .assemble(false)
                    .map(|(_, _, tokens)| {
                        self.assembled = String::new();
                        notifier::notifications()
                            .iter()
                            .for_each(|warning| self.assembled.push_str(&format!("{}\n", warning)));

                        tokens
                            .iter()
                            .for_each(|(_, s)| self.assembled.push_str(&format!("{}\n", s)));
                    })
                    .or_else(|| {
                        self.assembled = String::from("There were errors during assembly:\n");
                        notifier::notifications()
                            .iter()
                            .for_each(|error| self.assembled.push_str(&format!("{}\n", error)));
                        Some(())
                    });

                notifier::clear(Some(NOTIFIER_NAME));
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
                <NavBar />

                <div class="pure-g" style="height: calc(100% - 11em); z-index: 2; position: relative; padding-top: 2em">
                    <span class="pure-u-1" style="height: 3.5em" />

                    <div class="pure-u-lg-1-2 pure-u-1" style="height: 100%; display: flex;">
                        <Editor onchange=self.link.callback(|code| Msg::Assemble(code)) />
                    </div>

                    <span class="pure-u-1 split" style="height: 2em" />

                    <div class="pure-u-lg-1-2 pure-u-1" style="height: 100%; display: flex; justify-content: center">
                        <label for="assembler-output-pane" style="display: none;">{"Assembler Output Pane"}</label>
                        <textarea id="assembler-output-pane" class="pure-u-22-24 bordered" aria-label="output pane" spellcheck="false"
                            readonly=true value=self.assembled />
                    </div>

                    <span class="pure-u-1" style="height: 2em" />

                    <div id="wcb" class="wcb carbonbadge wcb-d" style="z-index: 2; width: 100%; letter-spacing: 0;">
                        <div id="wcb_p">
                            <p id="wcb_g">{"Measuring CO"}<sub>{2}</sub>{"…"}</p>
                            <a target="_blank" rel="noopener" href="https://websitecarbon.com">{"Website Carbon"}</a>
                        </div>
                        <p id="wcb_2"></p>
                    </div>
                </div>

                <script src=urlfor("/static/js/main.js") defer=true></script>
            </>
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
