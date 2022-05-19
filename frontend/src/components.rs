use covalent_class_a::{resources, CovalentClient};
use log::{info, warn};
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

pub enum Msg {
    InputValue(String),
}

pub struct SearchBar;

fn get_address(value: &str) -> Result<Address, Box<dyn Error>> {
    let addr: Address = value.parse()?;
    Ok(addr)
}

fn get_tx_hash(value: &str) -> Result<H256, Box<dyn Error>> {
    let tx_hash: H256 = value.parse()?;
    Ok(tx_hash)
}

#[derive(PartialEq)]
pub enum SearchResultTypes {
    Balances(resources::Balances),
    Transactions(resources::Transactions),
}

impl Component for SearchBar {
    type Message = Msg;
    type Properties = ();

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

            let mut results: Vec<SearchResultTypes> = vec![];

            let client = CovalentClient::new("8127").expect("Could not create client");
            match get_address(&full_value) {
                Ok(_) => {
                    info!("Found valid address");
                    let balances = client
                        .get_token_balances(&full_value)
                        .expect("Failed to get token balances");
                    results.push(SearchResultTypes::Balances(balances));

                    let transactions = client
                        .get_transactions_for_address(&full_value)
                        .expect("Failed to get transactions for address");
                    results.push(SearchResultTypes::Transactions(transactions))
                }
                Err(e) => info!("Not a valid address: {}", e),
            }

            match get_tx_hash(&full_value) {
                Ok(_) => {
                    info!("Found valid tx_hash");
                }
                Err(e) => info!("Not a valid tx_hash: {}", e),
            }

            Msg::InputValue(full_value)
        });
        html! {
            <label for="search-bar">
            { "My Search Bar: " }
                <input oninput={on_change} id="search-bar" type="text"/>
            </label>
        }
    }

    fn create(_: &Context<Self>) -> Self {
        SearchBar {}
    }
}

pub struct SearchResult;

#[derive(Properties, PartialEq)]
pub struct SearchResultProps {
    results: Vec<SearchResultTypes>,
    // on_click: Callback<SearchResultDetail>,
}

impl Component for SearchResult {
    type Message = ();
    type Properties = ();

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <h1>{ "Sample Search Result" }</h1>
        }
    }

    fn create(_: &Context<Self>) -> Self {
        SearchResult {}
    }
}

pub struct SearchResultDetail;

impl Component for SearchResultDetail {
    type Message = ();
    type Properties = ();

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <h1>{ "Sample Search Result Detail" }</h1>
        }
    }

    fn create(_: &Context<Self>) -> Self {
        SearchResultDetail {}
    }
}
