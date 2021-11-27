use yew::prelude::*;

use crate::utils::WeakComponentLink;

pub enum ClipboardMsg {
    UpdateClipboard(String),
    SendClipboard,
}

pub struct Clipboard {
    link: ComponentLink<Self>,
    onsubmit: Callback<String>,
    text: String,
}

// Props
#[derive(Clone, PartialEq, Properties)]
pub struct ClipboardProps {
    #[prop_or_default]
    pub weak_link: WeakComponentLink<Clipboard>,
    pub onsubmit: Callback<String>,
}

impl Component for Clipboard {
    type Message = ClipboardMsg;
    type Properties = ClipboardProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        props.weak_link.borrow_mut().replace(link.clone());
        Clipboard {
            link,
            onsubmit: props.onsubmit,
            text: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ClipboardMsg::UpdateClipboard(text) => {
                self.text = text;
            }
            ClipboardMsg::SendClipboard => {
                self.onsubmit.emit(self.text.clone());
                self.text.clear();
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let update_clipboard = self.link.callback(|e: ChangeData| match e {
            ChangeData::Value(v) => ClipboardMsg::UpdateClipboard(v),
            _ => panic!("unexpected message"),
        });
        let set_clipboard = self.link.callback(|_| ClipboardMsg::SendClipboard);
        html! {
            <>
                <textarea rows="5" cols="60" id="clipboard" onchange=update_clipboard value=self.text.clone()/>
                <br/>
                <button id="clipboard-send" onclick=set_clipboard> {"Send to peer"} </button>
                <br/>
            </>
        }
    }
}
