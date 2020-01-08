use super::prelude::*;

#[derive(Clone, Properties)]
pub struct Props {
    #[props(required)]
    pub tab: Tab,
}

pub struct Header {
    prop: Props,
}

impl Component for Header {
    type Message = ();
    type Properties = Props;

    fn create(prop: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { prop }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, prop: Self::Properties) -> ShouldRender {
       self.prop.tab = prop.tab;
       true
    }

    fn view(&self) -> Html {
        let is_current = |tab|
            if tab == self.prop.tab { "active-page" } else { "inactive-page" };

        html! {
            <header>
                <div id="nav-title">
                    <span class="title">{ "全て桜の下に" }</span>
                    <span class="placeholder"></span>
                    <span class="subtitle">{ "ほとけには　桜の花を　たてまつれ　我が後の世を　人とぶらはば" }</span>
                </div>
                <div class="placeholder"></div>
                <nav>
                    <a href=HOME_URL class=("button", is_current(Tab::Home))>{ "Home" }</a>
                    <a href=POST_URL class=("button", is_current(Tab::Post))>{ "Post" }</a>
                    <a href=PROJECT_URL class=("button", is_current(Tab::Project))>{ "Project" }</a>
                    <a href=ABOUT_URL class=("button", is_current(Tab::About))>{ "About" }</a>
                </nav>
            </header>
        }
    }
}