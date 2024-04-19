use iced::widget::{button, column, container, row, scrollable, text, text_input, Column};
use iced::{alignment, Element, Length, Padding, Sandbox, Settings};

struct GroceryList {
    grocery_items: Vec<String>,
    input_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputValue(String),
    Submitted,
}

impl Sandbox for GroceryList {
    type Message = Message;

    /* Initialize your app */
    fn new() -> GroceryList {
        Self {
            grocery_items: vec!["Eggs".to_owned(), "Milk".to_owned(), "Flour".to_owned()],
            input_value: String::default(),
        }
    }

    fn title(&self) -> String {
        String::from("Grocery List App")
    }
    fn theme(&self) -> iced::Theme {
        iced::Theme::Dracula
    }

    fn update(&mut self, _message: Self::Message) {
        /*
        Update the state of your app
        */
    }

    fn view(&self) -> Element<Self::Message> {
        container(
            column!(
                items_list_view(&self.grocery_items),
                row!(text_input("Input grocery item", ""), button("Submit"))
                    .spacing(30)
                    .padding(Padding::from(30))
            )
            .align_items(iced::Alignment::Center),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .into() 
    }
}

fn items_list_view(items: &Vec<String>) -> Element<'static, Message> {
    let mut column = Column::new()
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .width(Length::Fill);
    for value in items {
        column = column.push(text(value));
    }
    container(column).height(250.0).width(300).into()
}

fn main() -> iced::Result {
    GroceryList::run(Settings::default())
}
