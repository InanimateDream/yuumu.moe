use super::prelude::*;

pub struct Index {
    link: ComponentLink<Self>,
    curr_page: Page,
}

pub enum Msg {
    Routing(Page),
}

impl Component for Index {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        route(link.callback(Msg::Routing));
        Index {
            link,
            curr_page: Page::Home,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Routing(page) => {
                self.curr_page = page;
                true
            }
        }
    }

    fn view(&self) -> Html {
        let tab: Tab = self.curr_page.into();
        html! {
            <div id="yuumu">
                <Header tab=tab />
                <Body page=self.curr_page />
                <div id="footer"></div>
            </div>
        }
    }
}