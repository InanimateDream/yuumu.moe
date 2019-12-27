use super::*;
use Window::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    #[props(required)]
    pub curr_wnd: Window,
    #[props(required)]
    pub onclick: Callback<Window>,
}

pub struct Header {
    prop: Props,
    link: ComponentLink<Self>,
}

impl Component for Header {
    type Message = Window;
    type Properties = Props;

    fn create(prop: Self::Properties, link: ComponentLink<Self>) -> Self {
        Header { link, prop }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            wnd@_ => {
                self.prop.onclick.emit(wnd);
                true
            },
        }
    }

    fn change(&mut self, prop: Self::Properties) -> ShouldRender {
       self.prop.curr_wnd = prop.curr_wnd;
       true
    }

    fn view(&self) -> Html {
        let is_current = |w|
            if w == self.prop.curr_wnd { "active-page" } else { "inactive-page" };

        html! {
            <header>
                <div id="nav-title">
                    <span class="title">{ "全て桜の下に" }</span>
                    <span class="placeholder"></span>
                    <span class="subtitle">{ "ほとけには　桜の花を　たてまつれ　我が後の世を　人とぶらはば" }</span>
                </div>
                <div class="placeholder"></div>
                <nav>
                    <a href="#/" class=("button", is_current(Home))
                        onclick=self.link.callback(|_| Home)>{ "Home" }</a>
                    <a href="#/post" class=("button", is_current(Post))
                        onclick=self.link.callback(|_| Post)>{ "Post" }</a>
                    <a href="#/project" class=("button", is_current(Project))
                        onclick=self.link.callback(|_| Project)>{ "Project" }</a>
                    <a href="#/about" class=("button", is_current(About))
                        onclick=self.link.callback(|_| About)>{ "About" }</a>
                </nav>
            </header>
        }
    }
}