use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
mod services;
mod types;

use pages::{Home, SellerStore, SignIn, Account};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/store/:id")]
    SellerStore { id: String },
    #[at("/signin")]
    SignIn,
    #[at("/account")]
    Account,
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::SellerStore { id } => html! { <SellerStore id={id} /> },
        Route::SignIn => html! { <SignIn /> },
        Route::Account => html! { <Account /> },
    }
}