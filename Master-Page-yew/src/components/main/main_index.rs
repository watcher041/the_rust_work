
use yew::{Component, Html, html, Context};
use material_yew::MatButton;

// Indexコンポーネント
pub struct Index;
impl Component for Index {

    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        let document = gloo_utils::document();
        let link = document.create_element("link").unwrap();
        link.set_attribute("rel","stylesheet").unwrap();
        link.set_attribute("href","stylesheets/main.css").unwrap();
        _ = document.head().unwrap().append_child(link.as_ref());
        Self
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <div id="main_index">
                <div id="main_index_left">
                    <img id="main_index_left_img" src="images/logo.png" />
                    <ul>
                        <li><MatButton label="Click me!" raised=true/></li>
                        <li><MatButton label="Click me!" raised=true/></li>
                        <li><MatButton label="Click me!" raised=true/></li>
                    </ul>
                </div>
                <video id="main_index_right" src="videos/top.mp4"></video>
            </div>
        }
    }
}
