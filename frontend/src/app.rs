use std::borrow::Cow;

use crate::components::auth;
use crate::pages::{page_home::PageHome, page_not_found::PageNotFound, page_ssh::PageSsh};
use yew::html::IntoPropValue;
use yew::prelude::*;
use yew::services::ConsoleService;
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

impl From<AppRoute> for &str {
    fn from(route: AppRoute) -> Self {
        match route {
            AppRoute::Ssh => "/ssh",
            _ => "/",
        }
    }
}

impl IntoPropValue<Option<Cow<'_, str>>> for AppRoute {
    fn into_prop_value(self: AppRoute) -> Option<Cow<'static, str>> {
        Some(Cow::Borrowed(self.into()))
    }
}

pub struct App {
    authdone: bool,
    link: ComponentLink<Self>,
}

pub enum AppMsg {
    AuthDone,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            authdone: false,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppMsg::AuthDone => self.authdone = true,
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                {

                    if self.authdone {
                        html! {
                            <>
                                {self.view_nav()}

                                <main class="content">
                                <Router<AppRoute>
                                    render = Router::render(Self::switch)
                                    redirect=Router::redirect(|route: Route| {
                                        ConsoleService::log(&format!("{:?}", route));
                                        AppRoute::NotFound
                                    })
                                />
                                </main>
                            </>
                        }
                    }
                    else {
                        let onauthdone = &self.link.callback(|_| AppMsg::AuthDone);
                        html!{
                            <auth::AuthComponents onauthdone=onauthdone/>
                        }
                    }
                }
                <footer class="footer">
                    { "Powered by " }
                    <a href="https://yew.rs">{ "Yew" }</a>
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
