use super::prelude::*;

#[derive(Clone, Properties)]
pub struct Props {
    #[props(required)]
    pub page: Page,
}

pub struct Body {
    prop: Props,
    link: ComponentLink<Self>
}

impl Component for Body {
    type Message = ();
    type Properties = Props;

    fn create(prop: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { prop, link }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, prop: Self::Properties) -> ShouldRender {
       self.prop.page = prop.page;
       true
    }

    fn view(&self) -> Html {
        html! {
            dispatch(self.prop.page)
        }
    }
}

fn dispatch(pg: Page) -> Html {
    match pg {
        Page::Home => html! {
            <Incoming />
        },
        Page::Post => html! {
            <Incoming />
        },
        Page::Project => html! {
            <Incoming />
        },
        Page::About => html! {
            <main id="about-main">
                <About />
            </main>
        },
    }
}
