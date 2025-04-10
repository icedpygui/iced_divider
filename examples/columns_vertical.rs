//! Example
use iced::widget::{button, center, column, container, stack, text, toggler};

use iced::{Color, Element, Size};

use iced_divider::divider::{self, divider_vertical};

pub fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .theme(App::theme)
        .antialiasing(true)
        .centered()
        .window_size(Size::new(400.0, 500.0))
        .run()
}

struct App {
    column_heights: Vec<f32>,
    handle_width: f32,
    handle_height: f32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    DividerChange((usize, f32)),
}

impl App {
    fn new() -> Self {
        App {
            column_heights: vec![200.0; 2],
            handle_width: 175.0,
            handle_height: 4.0,
        }
    }

    fn title(&self) -> String {
        String::from("Divider Widget - Iced")
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::DividerChange((index, value)) => {
                let diff = self.column_heights[index] - value;

                // Adjust the left side
                self.column_heights[index] = value;
                
                // Adjust the right side
                if index < self.column_heights.len()-1 {
                     self.column_heights[index+1] += diff;
                }
            },
        }
    }

    fn view(&self) -> Element<Message> {

        let mut item_col: Vec<Element<Message>> = vec![];

        for height in self.column_heights.iter() {
            // Add whatever container you want.
            item_col.push(
                container(
                column![
                    text(format!("Column = {}", height)),
                    button("Some Button"),
                    button("Another button"),
                    toggler(false).label("Toggler"),
                    ]           
                    .height(*height)
                    .width(self.handle_width)
                )
                .style(|_|{
                    let mut style = container::Style::default();
                    style.border.color = Color::WHITE;
                    style.border.width = 1.0;
                    style
                }
            ).into()
            );
        };
        
        // Make the divider
        // In theis case, the containers have a border so
        // we'll set the divider background to transparent.
        let div = divider_vertical(
            self.column_heights.clone(),
            self.handle_width,
            self.handle_height,
            Message::DividerChange,
        )
        .style(|theme, status| {
            divider::transparent(theme, status)
        })
        .into();
   

        // Put the columns into a row
        let col: Element<Message> = 
            column(item_col).into();

        // put them in a stack
        let stk = 
            stack([col, div]);

        // Center everything in the window
        center(stk).into()

    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}


