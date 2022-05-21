use covalent_class_a::{resources, CovalentClient};
use lazy_static::lazy_static;
use log::info;
use std::error::Error;
use std::sync::Arc;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web3::types::{Address, H256};
use web_sys::{EventTarget, HtmlInputElement, InputEvent};
use yew::prelude::*;

lazy_static! {
    static ref BACKEND_ADDR: String = "http://127.0.0.1:3030".to_string();
}

fn print_type_of<T>(_: &T) {
    info!("{}", std::any::type_name::<T>())
}

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

#[derive(PartialEq, Clone, Debug)]
pub enum SearchResultTypes {
    BalancesData(resources::BalancesData),
    TransactionsData(resources::TransactionsData),
    TransactionData(resources::TransactionData),
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
            let target: EventTarget = e.target().expect("Event should have a target");
            let full_value = target.unchecked_into::<HtmlInputElement>().value();

            // TODO: comment out below logging when not testing
            info!("Current search query: {}", full_value);
            let results: Vec<SearchResultTypes> = vec![];

            match get_address(&full_value) {
                Ok(_) => {
                    info!("Found valid address");
                    // TODO: display "loading..." here while fetch api sends off the requests
                    // TODO: implement search results for get_token_balances and details

                    let covalent_client =
                        CovalentClient::new("8217", "ckey_44f70254f9364489b8fddce0549")
                            .expect("Could not create covalent client");

                    let client = covalent_client.clone();
                    let value = full_value.clone();
                    // TODO: row and then on back look into pulling data from these asyncs with a channel
                    spawn_local(async move {
                        let balances = match client.get_token_balances(&value).await {
                            Ok(balances) => balances,
                            Err(e) => panic!("failed to get token balances: {}", e),
                        };
                        info!("Got balance address: {}", balances.data.address);
                    });

                    let client = covalent_client.clone();
                    let value = full_value.clone();
                    spawn_local(async move {
                        let transactions = match client.get_transactions_for_address(&value).await {
                            Ok(transactions) => transactions,
                            Err(e) => panic!("failed to get transactions for address: {}", e),
                        };
                    });
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
                SearchResultTypes::BalancesData(balances) => {
                    rendered_results.push(html! {
                        <>
                            <h3>{ "Balances Result For Address" }</h3>
                            <p>{ "For address:" } { &balances.data.address }</p>
                            <p>{ "Currency:" } { &balances.data.quote_currency }</p>
                        </>
                    });
                }
                SearchResultTypes::TransactionsData(transactions) => {
                    rendered_results.push(html! {
                        <>
                            <h3>{ "Transactions Result For Address" }</h3>
                            <p>{ "For address:" } { &transactions.data.address }</p>
                            <p>{ "Currency:" } { &transactions.data.quote_currency }</p>
                            <p> { "Chain ID:" } { &transactions.data.chain_id }</p>
                        </>
                    });
                }
                _ => panic!("unknown"),
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
