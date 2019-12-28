use super::prelude::*;

pub struct Index {
    link: ComponentLink<Self>,
    curr_tab: Tab,
    curr_page: Page,
}

pub enum Msg {
    HeaderClicked(Tab),
    Routing(Page),
}

impl Component for Index {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        route(link.callback(Msg::Routing));
        Index {
            link,
            curr_tab: Tab::Home,
            curr_page: Page::Home,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::HeaderClicked(tab) => {
                self.curr_tab = tab;
                true
            },
            Msg::Routing(page) => {
                self.curr_page = page;
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div id="yuumu">
                <Header tab=self.curr_tab
                    onclick=self.link.callback(Msg::HeaderClicked) />
                <Body page=self.curr_page />
                <div id="footer"></div>
            </div>
        }
    }
}