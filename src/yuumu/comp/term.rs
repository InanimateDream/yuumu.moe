use super::*;
#[derive(Clone, Properties)]
pub struct Props {
    #[props(required)]
    pub term: String,
    #[props(required)]
    pub trans: String,
}
pub struct Term { prop: Props }
impl Component for Term {
    type Message = ();
    type Properties = Props;
    fn create(prop: Self::Properties, _: ComponentLink<Self>) -> Self { Self { prop } }
    fn update(&mut self, _: Self::Message) -> ShouldRender { true }
    fn view(&self) -> Html {
        html! {
            <span class="term-ctx">
                <span class="term"> { self.prop.term.as_str() } </span>
                <span class="trans"> { format!(" ({}) ", self.prop.trans.as_str()) } </span>
            </span>
        }
    }
}