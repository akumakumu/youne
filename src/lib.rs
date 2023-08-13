use yew::prelude::*;
use gloo::console::log;

// Components
mod component;

use component::atom::test_title::TestTitle;

#[function_component(App)]
pub fn app() -> Html {

    // Logging 
    let name = "yy";
    log!(name);

    html! {
        <>
            // <h1 class="text-3xl text-center font-bold">{ "Youne" }</h1>
            < TestTitle />
            <p>{"fragment"}</p>
        </>
    }
}