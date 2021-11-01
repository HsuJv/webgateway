use yew::prelude::*;

pub struct PageSsh {}

pub enum Msg {}

impl Component for PageSsh {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        PageSsh {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <p>{ "Hello ssh!\n\n\n\n" }</p>
        }
    }
}
