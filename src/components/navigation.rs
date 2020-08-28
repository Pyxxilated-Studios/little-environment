use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};

#[must_use]
pub fn static_urlfor(route: &str) -> String {
    format!("https://static.pyxxilated.studio{}", route)
}

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
                <div class="pure-u-1 pure-u-md-1-5">
                    <div class="pure-menu">
                    <a href=urlfor("/") class="pure-menu-heading custom-brand">
                        {"Pyxxilated"}<br />{"Studios"}
                    </a>
                    <a
                        href="#"
                        aria-label="None"
                        class="custom-toggle"
                        id="toggle"
                        style="height: 2.5em;"
                    >
                        <s class="bar"></s><s class="bar"></s>
                    </a>
                    </div>
                </div>
                <div class="pure-u-1 pure-u-md-3-5">
                    <div
                    class="pure-menu pure-menu-horizontal custom-can-transform centered"
                    >
                    <ul class="pure-menu-list">
                        <li class="pure-menu-item">
                        <a href=urlfor("/blog") class="pure-menu-link">{"Blog"}</a>
                        </li>
                        <li class="pure-menu-item">
                        <a href=urlfor("/projects") class="pure-menu-link">{"Projects"}</a>
                        </li>
                        <li class="pure-menu-item">
                        <a href=urlfor("/about") class="pure-menu-link">{"About"}</a>
                        </li>
                    </ul>
                    </div>
                </div>
                <div class="pure-u-1 pure-u-md-1-5">
                    <div class="pure-menu pure-menu-horizontal custom-menu-3">
                    <ul class="pure-menu-list">
                        <li class="pure-menu-item">
                        <input
                            type="checkbox"
                            class="switch"
                            id="darkSwitch"
                            aria-label="Dark Mode Switch"
                        />
                        </li>
                        <li class="pure-menu-item">
                        <a href="https://www.github.com/pyxxil" aria-label="Github">
                            <svg
                            class="menu-image"
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
                            viewBox="0 0 24 24"
                            >
                            <path
                                d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
                            />
                            </svg>
                        </a>
                        </li>
                        <li class="pure-menu-item">
                        <a href="https://www.gitlab.com/pyxxil" aria-label="Gitlab">
                            <svg
                            class="menu-image"
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 586 559"
                            width="24"
                            height="24"
                            >
                            <path
                                d="m461.48 298.35l-17.78-54.63a7.72 7.72 0 0 0 -.43 -1.47l-35.67-109.8a14.18 14.18 0 0 0 -13.54 -9.67 13.94 13.94 0 0 0 -13.38 9.75l-34 104.63h-107.31l-34.05-104.63a13.94 13.94 0 0 0 -13.32 -9.75h-.08a14.22 14.22 0 0 0 -13.5 9.76l-35.7 109.93c0 .1-.08.18-.11.28l-18.1 55.61a20.29 20.29 0 0 0 7.37 22.71l156.38 113.63a8 8 0 0 0 9.45 -.05l156.41-113.58a20.28 20.28 0 0 0 7.36 -22.72m-233.75-45.13l43.59 134.16-104.64-134.16m148.05 134.19l41.8-128.62 1.8-5.57h61.1l-94.67 121.28m69.44-231.67l30.63 94.33h-61.31m-22.03 16l-30.37 93.46-18.12 55.66-48.42-149.12m-52.73-110.33l30.69 94.33h-61.27m-19.98 70.97a4.31 4.31 0 0 1 -1.56 -4.83l13.44-41.3 98.57 126.37m192.98-80.24l-110.46 80.21.37-.48 98.2-125.86 13.44 41.28a4.31 4.31 0 0 1 -1.55 4.84"
                            />
                            </svg>
                        </a>
                        </li>
                        <li class="pure-menu-item">
                        <a
                            href="https://www.linkedin.com/in/pyxxil"
                            aria-label="LinkedIn"
                        >
                            <svg
                            class="menu-image"
                            style="enable-background: new 0 0 67 67;"
                            version="1.1"
                            viewBox="0 0 67 67"
                            width="24"
                            height="24"
                            >
                            <path
                                d="M50.837,48.137V36.425c0-6.275-3.35-9.195-7.816-9.195 c-3.604,0-5.219,1.983-6.119,3.374V27.71h-6.79c0.09,1.917,0,20.427,0,20.427h6.79V36.729c0-0.609,0.044-1.219,0.224-1.655 c0.49-1.22,1.607-2.483,3.482-2.483c2.458,0,3.44,1.873,3.44,4.618v10.929H50.837z M22.959,24.922c2.367,0,3.842-1.57,3.842-3.531 c-0.044-2.003-1.475-3.528-3.797-3.528s-3.841,1.524-3.841,3.528c0,1.961,1.474,3.531,3.753,3.531H22.959z M34,64 C17.432,64,4,50.568,4,34C4,17.431,17.432,4,34,4s30,13.431,30,30C64,50.568,50.568,64,34,64z M26.354,48.137V27.71h-6.789v20.427 H26.354z"
                            />
                            </svg>
                        </a>
                        </li>
                    </ul>
                    </div>
                </div>
            </div>
        }
    }
}
