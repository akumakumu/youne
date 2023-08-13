use yew::prelude::*;

#[function_component(TestTitle)]
pub fn test_title() -> Html {
    html! {
        <h1 class="text-3xl text-center font-bold">{ "Youne on Test" }</h1>
    }
}