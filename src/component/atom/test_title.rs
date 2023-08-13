use yew::prelude::*;

//Properties
#[derive(Properties, PartialEq)]
pub struct Prop {
    pub title : String
}

#[function_component(TestTitle)]
pub fn test_title(prop : &Prop) -> Html {
    html! {
        <h1 class="text-3xl text-center font-bold">{ &prop.title }</h1>
    }
}