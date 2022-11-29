use gloo::console::log;
use stylist::{ast::Sheet, yew::styled_component, Style};
use yew::{html, Html};

#[styled_component(App)]
pub fn app() -> Html {
    log!("hello yew");
    let get_app_style_sheet = || -> Sheet { include_str!("dist/app.css").parse().expect("") };
    html! {
      <div class={Style::new(get_app_style_sheet()).unwrap()}>
        <h1>{"Hello Yew"}</h1>
      </div>
    }
}
