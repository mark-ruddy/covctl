use covalent_class_a::{resources, CovalentClient};
use log::info;
use std::error::Error;
use wasm_bindgen::JsCast;
use web3::types::{Address, H256};
use web_sys::{EventTarget, HtmlInputElement, InputEvent};
use yew::prelude::*;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn view(&self, _: &Context<Self>) -> Html {
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

fn get_address(value: &str) -> Result<Address, Box<dyn Error>> {
    let addr: Address = value.parse()?;
    Ok(addr)
}

fn get_tx_hash(value: &str) -> Result<H256, Box<dyn Error>> {
    let tx_hash: H256 = value.parse()?;
    Ok(tx_hash)
}

#[derive(PartialEq, Clone)]
pub enum SearchResultTypes {
    Balances(resources::Balances),
    Transactions(resources::Transactions),
}

pub struct SearchBar {
    results: Vec<SearchResultTypes>,
}

impl Component for SearchBar {
    type Message = Vec<SearchResultTypes>;
    type Properties = ();

    // When a new message is received from the input being updated we should re-render
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.results = msg;
        true
    }

    fn create(_: &Context<Self>) -> Self {
        SearchBar { results: vec![] }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change = ctx.link().callback(|e: InputEvent| {
            // DESIGN:
            // - user enters "one full data point" - like a tx_hash or an address etc.(I'll find more as I go through more API)
            // - once a full data point is found - put some sort of loading indicator and send off the API calls through the backend library
            // - (maybe) the api calls come back basically as a set of different rust structs. Maybe I have an enum of all possible structs that my SearchResult component can display and it will display each one slightly differently. This could be good cause it would allow me to have just one component for all results even if they vary a bit.

            let target: EventTarget = e.target().expect("Event should have a target");

            // this form of on_change can only be used when the target is a HtmlInputElement
            // the full_value is all of the text currently in the target HtmlInputElement
            let full_value = target.unchecked_into::<HtmlInputElement>().value();

            // TODO: comment out below logging when not testing
            info!("Current search query: {}", full_value);

            let results: Vec<SearchResultTypes> = vec![];
            // The Klaytn mainnet chain_id on the unified Covalent API is 8127
            // let _client = CovalentClient::new("8127")
            // .expect("Could not create client");

            match get_address(&full_value) {
                Ok(_) => {
                    info!("Found valid address");
                    // TODO: re-setup axum backend and then use the yew fetch api here to call it
                    // yew fetches from backend route(no parameters by default, let the backend handle configuration like chain_id) -> backend route uses library method to make api call, and serializes it to JSON -> frontend gets the JSON and deserializes it using the library resource structs
                }
                Err(e) => info!("Not a valid address: {}", e),
            }

            match get_tx_hash(&full_value) {
                Ok(_) => {
                    info!("Found valid tx_hash");
                }
                Err(e) => info!("Not a valid tx_hash: {}", e),
            }
            results
        });

        html! {
            <>
                <label for="search-bar">
                { "My Search Bar: " }
                    <input oninput={on_change} id="search-bar" type="text"/>
                </label>
                <SearchResultList results={self.results.clone()}/>
            </>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct SearchResultListProps {
    results: Vec<SearchResultTypes>,
}

pub struct SearchResultList {
    props: SearchResultListProps,
}

impl Component for SearchResultList {
    type Message = ();
    type Properties = SearchResultListProps;

    fn create(_: &Context<Self>) -> Self {
        SearchResultList {
            props: SearchResultListProps { results: vec![] },
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let mut rendered_results: Vec<Html> = vec![];
        for result in &self.props.results {
            match result {
                SearchResultTypes::Balances(balances) => {
                    rendered_results.push(html! {
                        <>
                            <h3>{ "Balances Result For Address" }</h3>
                            <p>{ "For address:" } { &balances.address }</p>
                            <p>{ "Currency:" } { &balances.quote_currency }</p>
                        </>
                    });
                }
                SearchResultTypes::Transactions(transactions) => {
                    rendered_results.push(html! {
                        <>
                            <h3>{ "Transactions Result For Address" }</h3>
                            <p>{ "For address:" } { &transactions.address }</p>
                            <p>{ "Currency:" } { &transactions.quote_currency }</p>
                            <p> { "Chain ID:" } { &transactions.chain_id }</p>
                        </>
                    });
                }
            }
        }
        html! {
            <>
                <h2>{ "Search Results should be below here" }</h2>
                { for rendered_results }
            </>
        }
    }
}

pub struct SearchResultDetail;

impl Component for SearchResultDetail {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        SearchResultDetail {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <h1>{ "Sample Search Result Detail" }</h1>
        }
    }
}
