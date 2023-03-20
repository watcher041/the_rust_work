
use yew::{Component, Html, html, Context,Properties};

// Showコンポーネント
#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub prop1: String,
    pub prop2: String,
}

pub struct Show;
impl Component for Show {

    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            {
                format!(
                    "prop1: {} and prop2: {}",
                    ctx.props().prop1,
                    ctx.props().prop2
                )
            }
        }
    }
}
