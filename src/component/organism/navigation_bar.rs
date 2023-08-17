use yew::prelude::*;

#[function_component(NavigationBar)]
pub fn navigationbar() -> Html {
    html! {
        <>
            <nav class="fixed top-0 left-0 bg-fuchsia-100 shadow-md rounded-md m-2 p-1">
                <div class="flex items-center w-32 space-x-4">
                    <div>
                        <img src="image/saint-normal.png" class="h-20 rounded-md"/>
                    </div>
                    
                    <div>
                        <span class="text-3xl font-bold">{"yね"}</span>
                    </div>
                </div>
            </nav>
        </>
    }
}