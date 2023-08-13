use yew::prelude::*;
use gloo::console::log;

// Components
mod component;

use component::atom::test_title::{TestTitle, Classer};

#[function_component(App)]
pub fn app() -> Html {

    // Logging 
    let name = "yy";
    log!(name);

    html! {
        <>
            // <h1 class="text-3xl text-center font-bold">{ "Youne" }</h1>
            < TestTitle title="Youne on test" classer={Classer::Center} />
            <p>{"fragment"}</p>
        </>
    }
}