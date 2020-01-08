use super::*;
#[derive(Clone, Properties)]
pub struct Props {
    #[props(required)]
    pub title: String,
    pub children: Children,
}
pub struct Theorem { prop: Props }
impl Component for Theorem {
    type Message = ();
    type Properties = Props;
    fn create(prop: Self::Properties, _: ComponentLink<Self>) -> Self { Self { prop } }
    fn update(&mut self, _: Self::Message) -> ShouldRender { true }
    fn view(&self) -> Html {
        html! {
            <blockquote class="theorem">
                <details open=true>
                    <summary class="title"> { self.prop.title.as_str() } </summary>
                    <div class="body"> { self.prop.children.render() } </div>
                </details>
            </blockquote>
        }
    }
}