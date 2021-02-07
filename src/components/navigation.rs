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
            <header id="menu">
            <nav>
              <h2><a href=urlfor("/") class="brand">{"Pyxxilated Studios"}</a></h2>

              <div class="brand-links">
                <ul>
                  <li>
                    <a href=urlfor("/blog") class="brand-link">{"Blog"}</a>
                  </li>
                  <li>
                    <a href=urlfor("/projects") class="brand-link">{"Projects"}</a>
                  </li>
                  <li>
                    <a href=urlfor("/about") class="brand-link">{"About"}</a>
                  </li>
                </ul>
              </div>
            </nav>
          </header>
        }
    }
}
