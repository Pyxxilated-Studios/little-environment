#![recursion_limit = "512"]

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

pub fn route(to: &str) -> String {
    format!("https://www.pyxxilated.studio{}", to)
}

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
            <>
                <div class="custom-wrapper pure-g shadow" id="menu">
                    <div class="pure-u-1 pure-u-md-6-24">
                        <div class="pure-menu">
                            <a href=route("/") class="pure-menu-heading custom-brand">
                                {"Pyxxilated Studios"}
                            </a>
                            <a href="#" aria-label="None" class="custom-toggle" id="toggle" style="height: 2.5em;">
                                <s class="bar" />
                                <s class="bar"/>
                            </a>
                        </div>
                    </div>
                    <div class="pure-u-1 pure-u-md-12-24">
                        <div
                            class="pure-menu pure-menu-horizontal custom-can-transform centered"
                        >
                            <ul class="pure-menu-list">
                                <li class="pure-menu-item">
                                    <a href=route("/blog") class="pure-menu-link">{"Blog"}</a>
                                </li>
                                <li class="pure-menu-item">
                                    <a href=route("/projects") class="pure-menu-link">{"Projects"}</a>
                                </li>
                                <li class="pure-menu-item">
                                    <a href=route("/about")class="pure-menu-link">{"About"}</a>
                                </li>
                            </ul>
                        </div>
                    </div>
                    <div class="pure-u-1 pure-u-md-6-24">
                        <div class="pure-menu pure-menu-horizontal custom-menu-3">
                            <ul class="pure-menu-list">
                                <li class="pure-menu-item">
                                    <a href="https://www.github.com/pyxxil" aria-label="Github">
                                        <img
                                            class="menu-image"
                                            src=route("/static/images/GitHub.svg")
                                            alt="Github"
                                        />
                                    </a>
                                </li>
                                <li class="pure-menu-item">
                                    <a href="https://www.gitlab.com/pyxxil" aria-label="Gitlab">
                                        <img
                                            class="menu-image"
                                            src=route("/static/images/GitLab.svg")
                                            alt="Gitlab"
                                        />
                                    </a>
                                </li>
                                <li class="pure-menu-item">
                                    <a href="https://www.linkedin.com/in/josh-hill-b655131a1/" aria-label="LinkedIn">
                                        <img
                                            class="menu-image"
                                            src=route("/static/images/LinkedIn.svg")
                                            alt="Gitlab"
                                        />
                                    </a>
                                </li>
                            </ul>
                        </div>
                    </div>
                </div>

                <div class="pure-g" style="height: calc(100% - 7em)">
                    <span class="pure-u-1" style="height: 2em" />

                    <Editor onchange=self.link.callback(|code| Msg::Assemble(code)) />

                    <span class="pure-u-1 split" style="height: 2em" />

                    <div class="pure-u-lg-1-2 pure-u-1" style="height: 100%">
                        <label for="assembler-output-pane" style="display: none;">{"Assembler Output Pane"}</label>
                        <span class="pure-u-1-24" />
                        <textarea id="assembler-output-pane" class="pure-u-22-24 shadow bordered" aria-label="output pane" spellcheck="false" readonly=true value=self.assembled />
                        <span class="pure-u-1-24" />
                    </div>
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
