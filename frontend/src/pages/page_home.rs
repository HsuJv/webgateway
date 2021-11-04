use yew::prelude::*;
use yew::ShouldRender;
use yew::{html, Component, Html};

pub struct PageHome;

impl Component for PageHome {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn view(&self) -> Html {
        html! {
            "Hello world"
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}
