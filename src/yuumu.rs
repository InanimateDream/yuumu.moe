mod header;
use yew::prelude::*;

use header::Header;

#[derive(Copy, Clone, PartialEq)]
pub enum Window {
    Home,
    Post,
    Project,
    About,
}

pub struct Index {
    link: ComponentLink<Self>,
    curr_wnd: Window,
}

pub enum Msg {
    HeaderClicked(Window)
}

impl Component for Index {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Index { link, curr_wnd: Window::Home }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::HeaderClicked(wnd) => {
                self.curr_wnd = wnd;
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div id="yuumu">
                <Header curr_wnd=self.curr_wnd
                    onclick=self.link.callback(Msg::HeaderClicked) />
            </div>
        }
    }
}

