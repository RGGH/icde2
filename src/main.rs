use iced::widget::{container, text, Column};
use iced::{alignment, Element, Length, Sandbox, Settings};

struct GroceryList {
    grocery_items: Vec<String>,
}

#[derive(Debug, Clone)]
enum Message {}

impl Sandbox for GroceryList {
    type Message = Message;

    /* Initialize your app */
    fn new() -> GroceryList {
        Self {
            grocery_items: vec!["Eggs".to_owned(), "Milk".to_owned(), "Flour".to_owned()],
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
        container(items_list_view(&self.grocery_items))
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
