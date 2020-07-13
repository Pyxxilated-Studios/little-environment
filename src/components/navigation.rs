use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};

#[must_use]
pub fn urlfor(route: &str) -> String {
    format!("https://www.pyxxilated.studio{}", route)
}

pub struct NavBar {}

impl Component for NavBar {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
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
            <div class="custom-wrapper pure-g shadow" id="menu">
                <div class="pure-u-1 pure-u-md-6-24">
                    <div class="pure-menu">
                        <a href=urlfor("/") class="pure-menu-heading custom-brand">
                            {"Pyxxilated Studios"}
                        </a>
                        <a href="#" aria-label="None" class="custom-toggle" id="toggle" style="height: 2.5em;">
                            <s class="bar" />
                            <s class="bar"/>
                        </a>
                    </div>
                </div>
                <div class="pure-u-1 pure-u-md-12-24">
                    <div class="pure-menu pure-menu-horizontal custom-can-transform centered" >
                        <ul class="pure-menu-list">
                            <li class="pure-menu-item">
                                <a href=urlfor("/blog") class="pure-menu-link">{"Blog"}</a>
                            </li>
                            <li class="pure-menu-item">
                                <a href=urlfor("/projects") class="pure-menu-link">{"Projects"}</a>
                            </li>
                            <li class="pure-menu-item">
                                <a href=urlfor("/about")class="pure-menu-link">{"About"}</a>
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
                                        src=urlfor("/static/images/GitHub.svg")
                                        alt="Github"
                                    />
                                </a>
                            </li>
                            <li class="pure-menu-item">
                                <a href="https://www.gitlab.com/pyxxil" aria-label="Gitlab">
                                    <img
                                        class="menu-image"
                                        src=urlfor("/static/images/GitLab.svg")
                                        alt="Gitlab"
                                    />
                                </a>
                            </li>
                            <li class="pure-menu-item">
                                <a href="https://www.linkedin.com/in/josh-hill-b655131a1/" aria-label="LinkedIn">
                                    <img
                                        class="menu-image"
                                        src=urlfor("/static/images/LinkedIn.svg")
                                        alt="Gitlab"
                                    />
                                </a>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        }
    }
}
