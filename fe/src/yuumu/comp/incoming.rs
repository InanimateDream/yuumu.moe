use super::*;
pub struct Incoming;
impl Component for Incoming {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self { Self }
    fn update(&mut self, _: Self::Message) -> ShouldRender { true }
    fn view(&self) -> Html {
        html! {
            <div id="incoming-page">
                <div id="incoming-img">
                    <img id="incoming" src="/img/yuyuko-incoming.jpg" />
                </div>
                <span id="incoming-text"> { "Incoming..." } </span>
            </div>
        }
    }
}