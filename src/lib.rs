use yew::prelude::*;

pub struct YewForm {}

impl Component for YewForm {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div>{"Hello, world!"}</div>
        }
    }
}