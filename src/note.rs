use yew::{html, Component, ComponentLink, Html, Properties};

pub struct Note {
    props: NoteProps,
}

#[derive(Properties, Clone, PartialEq)]
pub struct NoteProps {
    pub text: String,
    #[prop_or(vec![])]
    pub items: Vec<String>,
}

impl Component for Note {
    type Message = ();
    type Properties = NoteProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h3>{ &self.props.text }</h3>
                { for self.props.items.iter().map(|item| {
                    item
                }) }
            </div>
        }
    }

    fn update(&mut self, _msg: Self::Message) -> yew::ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        true
    }
}
