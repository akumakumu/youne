use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1 class="text-3xl text-center">{ "Youne" }</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}