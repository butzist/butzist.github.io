use yew::prelude::*;

mod about;
mod languages;
mod navbar;
mod projects;

use about::About;
use languages::Languages;
use navbar::{ContentPage, Nav};
use projects::Projects;

fn page_for_nav(page: ContentPage) -> Html {
    match page {
        ContentPage::About => html!(<About />),
        ContentPage::Languages => html!(<Languages />),
        ContentPage::Projects => html!(<Projects />),
    }
}

#[function_component(App)]
fn app() -> Html {
    let content = use_state(|| ContentPage::Projects);
    let set_content = {
        let content = content.clone();
        Callback::from(move |page: ContentPage| content.set(page))
    };

    html!(
        <>
            <Nav set_content={ set_content.clone() } />

            <div class="pt-5">
                { page_for_nav(*content) }
            </div>
        </>
    )
}

fn main() {
    yew::start_app::<App>();
}
