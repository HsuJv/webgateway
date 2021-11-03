use yew::prelude::*;
use yew::ShouldRender;
use yew::{html, Component, Html};

use crate::components;

pub enum HomeMsg {}

pub struct PageHome {
    link: ComponentLink<Self>,
}

impl Component for PageHome {
    type Message = HomeMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn view(&self) -> Html {
        html! {
            <components::auth::AuthComponents/>
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}
