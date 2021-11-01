use yew::prelude::*;
use yew::Component;
use yew::ShouldRender;

pub struct PageHome;

impl Component for PageHome {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn view(&self) -> Html {
        html! {
            <section class="hero is-danger is-bold is-large">
                <div class="hero-body">
                    <div class="container">
                        <h1 class="title">
                            { "Hello World" }
                        </h1>
                        <h2 class="subtitle">
                            { "Hello again" }
                        </h2>
                    </div>
                </div>
            </section>
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}
