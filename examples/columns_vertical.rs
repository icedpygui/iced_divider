//! Example
use iced::widget::{button, center, column, container, stack, text, toggler};

use iced::{Color, Element, Size};

use std::ops::RangeInclusive;
use iced_divider::divider::{self, Direction};

pub fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .theme(App::theme)
        .antialiasing(true)
        .centered()
        .window_size(Size::new(400.0, 500.0))
        .run()
}

struct App {
    column_heights: [f32; 2],
    divider_value: f32,
    range: RangeInclusive<f32>,
    divider_height: f32,
    handle_width: f32,
    handle_height: f32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    DividerChange((usize, f32)),
}

impl App {
    fn new() -> Self {
        let column_heights = [200.0; 2];
        App {
            column_heights,
            // adjusting for handle_height of 4
            divider_value: 200.0,
            // The range can be shorter than the entire width
            range: 0.0..=400.0,
            divider_height: column_heights.iter().sum(),
            handle_width: 200.0,
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
                // if you have more than 2 columns use the method 
                // in the texts or table example

                // Adjust the left side
                self.column_heights[index] = value;
                
                // Adjust the right side
                self.column_heights[index+1] += self.divider_value - value;
                
                self.divider_value = value;
            },
        }
    }

    fn view(&self) -> Element<Message> {

        let mut dividers: Vec<Element<Message>> = vec![];
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
        
        // Make the divider and add to a vec for later use
        // In theis case, the containers have a border so
        // we'll set the divider background to transparent.
        dividers.push(divider::divider(
            0,
            self.divider_value,
            self.range.clone(),
            self.handle_width,
            self.handle_height,
            Message::DividerChange,
        )
        .direction(Direction::Vertical)
        .style(|theme, status| {
            divider::transparent(theme, status)
        })
        .into());
   

        // Put the columns into a row
        let col: Element<Message> = 
            column(item_col)
                .height(self.divider_height)
                .into();

        // Insert the row at the beginning so that the dividers are on top.
        // You could add a space in the row and let the dividers be on the
        // bottom.  Since the stack is shrink length, the width of the
        // divider (not divider_handle) will be the with of the stack.
        dividers.insert(0, col);
        // put them in a stack
        let stk = stack(dividers);
        // Center everything in the window
        center(stk).into()

    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}


