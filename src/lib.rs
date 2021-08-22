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
                    .map(|(_, tokens)| {
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
                <div id="content">
                    <main style="height: 100%">
                        <div style="display: grid; grid-template-columns: repeat(2, auto); gap: 1em; height: 100%;">
                            <Editor onchange=self.link.callback(Msg::Assemble) />

                            <label for="assembler-output-pane" style="display: none;">
                                {"Assembler Output Pane"}
                            </label>
                            <textarea id="assembler-output-pane" aria-label="output pane"
                                spellcheck="false" readonly=true value=self.assembled.clone() />
                        </div>
                    </main>

                </div>
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
