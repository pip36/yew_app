use yew::{html, Component, ComponentLink, Html, InputData, KeyboardEvent};

pub struct TodoList {
    pub link: ComponentLink<Self>,
    pub todos: Vec<Todo>,
    pub input_value: String,
}

pub enum TodoMsg {
    Add,
    UpdateInput(String),
}

pub struct Todo {
    text: String,
}

impl Component for TodoList {
    type Message = TodoMsg;

    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TodoList {
            link,
            input_value: "".to_string(),
            todos: vec![
                Todo {
                    text: "First todo".to_string(),
                },
                Todo {
                    text: "Second todo".to_string(),
                },
            ],
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            TodoMsg::Add => {
                self.todos.push(Todo {
                    text: self.input_value.clone(),
                });
                self.input_value = "".to_string();
            }
            TodoMsg::UpdateInput(v) => self.input_value = v,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        let onkeypress = self.link.batch_callback(|event: KeyboardEvent| {
            if event.key() == "Enter" {
                Some(TodoMsg::Add)
            } else {
                None
            }
        });

        html!(
            <>
              <input
                value=self.input_value.clone()
                oninput=self.link.callback(|e: InputData| TodoMsg::UpdateInput(e.value))
                onkeypress=onkeypress
              />

              <ul>
                {for self.todos.iter().map(|todo| {
                    html!(<li>{&todo.text}</li>)
                })}
              </ul>
            </>
        )
    }
}
