
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/users/profile")]
    Profile,
    #[at("/users/friends")]
    Friends,
    #[at("/users/theme")]
    Theme,
    #[not_found]
    #[at("/users/404")]
    NotFound,
}

pub fn routing(route: &Route) -> Html {
    match route {
        Route::Profile => html! {
            <h1>{"Profile"}</h1>
        },
        Route::Friends => html! {
            <h1>{"Friends"}</h1>
        },
        Route::Theme => html! {
            <h1>{"Theme"}</h1>
        },
        Route::NotFound => html! {
            <h1>{"Not Found"}</h1>
        }
    }
}