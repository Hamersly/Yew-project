use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
    <nav class="navbar bg-black">
        <div class="container-fluid justify-content-center">
            <a class="navbar-brand text-white" href="#">{"User List"}</a>
        </div>
    </nav>
    }
}