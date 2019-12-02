use yew::prelude::*;
use yew_form::YewForm;

#[derive(Default)]
pub struct Values {}

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        let values = Values {};

        html! {
            <YewForm<Values> initial_values=values />
        }
    }
}