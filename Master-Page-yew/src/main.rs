
mod routes;
mod components;

use yew::prelude::*;
use yew_router::prelude::*;
use routes::users;
use components::main::*;

// mainのルート
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Index,
    #[at("/about")]
    Show,
    #[at("/contact")]
    Contact,
    #[at("/users/:s")]
    Users,
    #[not_found]
    #[at("/404")]
    NotFound,
}

// 主となるルーティング
fn routing(route: &Route) -> Html {
    match route {
        Route::Index =>  html! { 
            <main_index::Index /> 
        },
        Route::Show => html! {  
            <main_show::Show prop1="lorem" prop2="ipsum" /> 
        },
        Route::Contact => html! {
            <h1>{"Contact"}</h1>
        },
        Route::Users => html! {
            <Switch <users::Route> render={Switch::render(users::routing)} />
        },
        Route::NotFound => html! {
            <h1>{"Not Found"}</h1>
        },
    }
}

// Appコンポーネント
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch <Route> render={Switch::render(routing)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}