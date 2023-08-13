use yew::prelude::*;
// use gloo::console::log;

// Components
mod component;

// use component::atom::test_title::{TestTitle, Classer};
use component::organism::navigation_bar::NavigationBar;

#[function_component(App)]
pub fn app() -> Html {

    // Logging 
    // let name = "yy";
    // log!(name);

    html! {
        <>
            // < TestTitle title="Youne on test" classer={Classer::Center} />
            < NavigationBar />
        </>
    }
}