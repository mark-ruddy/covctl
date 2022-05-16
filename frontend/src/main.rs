use yew::prelude::*;

/*
so I need a structure based around a "search bar" with "search results"
I think the way you usually(or must) structure a yew/react like this is with one overarching parent component called "App"
My component layout under "App" might be something like this:
- A search bar at the top
  - maybe some "option" components similar to Googles ones under the search bar
  - search results - a basic list with summary data that auto updates as the search bar gets any full valid data points
    - search details - every/most search result items should have an attached "details" component that appears/disappears when the result is clicked

Factor: there will be different types of "search results" and hence "search details", this means I'll need logic that knows what type it is making when it asks the backend for the data - then it needs to be able to pass the "type" to the component so it knows how it should display it

Pagination: its just required isn't it?
*/

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

fn main() {
    yew::start_app::<App>();
}
