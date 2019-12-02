use yew::prelude::*;

#[derive(Properties)]
pub struct Props<TValues> where TValues: Default + 'static {
    #[props(required)]
    pub initial_values: TValues,
}

pub struct YewForm<TValues> where TValues: Default + 'static {
    pub props: Props<TValues>,
}

impl<TValues> Component for YewForm<TValues> where TValues: Default + 'static {
    type Message = ();
    type Properties = Props<TValues>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
        }
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