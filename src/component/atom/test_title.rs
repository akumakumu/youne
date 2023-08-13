use yew::prelude::*;

//Properties
#[derive(Properties, PartialEq)]
pub struct Prop {
    pub title : String,
    pub classer : Classer
}

// Enum
#[derive(PartialEq)]
pub enum Classer {
    Center
}

impl Classer {
    pub fn conv(&self) -> String {
        match self {
            Classer::Center => "text-center".to_owned()
        }
    }
}

#[function_component(TestTitle)]
pub fn test_title(prop : &Prop) -> Html {
    html! {
        <h1 class={ prop.classer.conv() }>{ &prop.title }</h1>
    }
}