use yew::prelude::*;

// message on update
pub enum InputMsg {
    Update(String),
}

// props on_change
#[derive(Clone, PartialEq, Properties)]
pub struct InputProps {
    pub on_change: Callback<String>,
    pub id: String,
    pub type_: String,
    pub placeholder: String,
}

// component input
pub struct Input {
    link: ComponentLink<Self>,
    on_change: Callback<String>,
    id: String,
    type_: String,
    placeholder: String,
}

impl Component for Input {
    type Message = InputMsg;
    type Properties = InputProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Input {
            link,
            on_change: props.on_change,
            id: props.id,
            type_: props.type_,
            placeholder: props.placeholder,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            InputMsg::Update(text) => {
                self.on_change.emit(text);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let on_change = self.link.callback(|e: ChangeData| match e {
            ChangeData::Value(v) => InputMsg::Update(v),
            _ => panic!("unexpected message"),
        });

        html! {
            <input
                id={self.id.clone()}
                type={self.type_.clone()}
                placeholder={self.placeholder.clone()}
                onchange={on_change}
            />
        }
    }
}
