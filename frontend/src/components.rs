use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "My App" }</h1>
                <SearchBar/>
            </div>
        }
    }

    fn create(_: &Context<Self>) -> Self {
        App {}
    }
}

pub enum Msg {
    InputValue(String),
}

pub struct SearchBar;

impl Component for SearchBar {
    type Message = Msg;
    type Properties = ();

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change = ctx.link().callback(|e: Event| {
            let target: EventTarget = e.target().expect("Event should have a target");
            Msg::InputValue(target.unchecked_into::<HtmlInputElement>().value())
        });
        html! {
            <label for="search-bar">
            { "My Search Bar: " }
                <input onchange={on_change} id="search-bar" type="text"/>
            </label>
        }
    }

    fn create(_: &Context<Self>) -> Self {
        SearchBar {}
    }
}
