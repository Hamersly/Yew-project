use yew::prelude::*;

#[function_component(Loader)]
pub fn loader() -> Html {
    html! {
        <div class="mt-4 d-flex justify-content-center">
            <div class="spinner-border" role="status">
                <span class="visually-hidden">{"Loading..."}</span>
            </div>
        </div>
    }
}