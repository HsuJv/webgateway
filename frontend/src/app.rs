use std::borrow::Cow;

use crate::pages::{page_home::PageHome, page_not_found::PageNotFound, page_ssh::PageSsh};
use yew::html::IntoPropValue;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::virtual_dom::VNode;
use yew::Component;
use yew_router::prelude::*;
use yew_router::{router::Router, Switch};

#[derive(Switch, Clone, Debug)]
enum AppRoute {
    // #[at("/ssh/:id")]
    // Ssh(i32),
    #[to = "/ssh"]
    Ssh,
    #[to = "/!"]
    Home,
    #[to = ""]
    NotFound,
}

impl Into<&str> for AppRoute {
    fn into(self) -> &'static str {
        match self {
            AppRoute::Ssh => &"/ssh",
            _ => &"/",
        }
    }
}

impl IntoPropValue<Option<Cow<'_, str>>> for AppRoute {
    fn into_prop_value(self: AppRoute) -> Option<Cow<'static, str>> {
        Some(Cow::Borrowed(self.into()))
    }
}

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> VNode {
        html! {
            <>
                { self.view_nav() }
                <main  class="content">
                    <Router<AppRoute>
                        render = Router::render(Self::switch)
                        redirect=Router::redirect(|route: Route| {
                            ConsoleService::log(&format!("{:?}", route));
                            AppRoute::NotFound
                        })
                    />
                </main>
                <footer class="footer">
                    <div class="content has-text-centered">
                        { "Powered by " }
                        <a href="https://yew.rs">{ "Yew" }</a>
                    </div>
                </footer>
            </>
        }
    }
}

impl App {
    fn view_nav(&self) -> Html {
        html! {
            <nav class="navbar" role="navigation" aria-label="main navigation">
                <div class=classes!("navbar-menu")>
                    <RouterAnchor<AppRoute> classes="navbar-item" route=AppRoute::Home>
                        { "Home" }
                    </RouterAnchor<AppRoute>>
                    <RouterAnchor<AppRoute> classes="navbar-item" route=AppRoute::Ssh>
                        { "Ssh" }
                    </RouterAnchor<AppRoute>>
                </div>
            </nav>
        }
    }

    fn switch(switch: AppRoute) -> Html {
        ConsoleService::log(&format!("{:?}", switch));
        match switch {
            // Route::Ssh(ip) => {
            //     html! { <Ssh /> }
            // }
            AppRoute::Ssh => {
                html! {<PageSsh />}
            }
            AppRoute::Home => {
                html! {<PageHome />}
            }
            AppRoute::NotFound => {
                html! { <PageNotFound /> }
            }
        }
    }
}
