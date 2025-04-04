

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{center, container, column, stack, text};
use iced::Length::Fill;
use iced::{Element, Size};

use std::ops::RangeInclusive;
use iced_divider::divider::{divider, Direction};

pub fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .theme(App::theme)
        .antialiasing(true)
        .centered()
        .window_size(Size::new(600.0, 400.0))
        .run()
}

struct App {
    column_heights: [f32; 4],
    divider_values: Vec<f32>,
    range: RangeInclusive<f32>,
    divider_width: f32,
    handle_width: f32,
    handle_height: f32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    DividerChange((usize, f32)),
}

impl App {
    fn new() -> Self {
        let column_heights = [50.0; 4];
        App {
            column_heights,
            // Since the default width is 4, adjust the value to line up with the item border
            divider_values: vec![50.0, 100.0, 150.0],
            range: 0.0..=200.0,
            divider_width: column_heights.iter().sum::<f32>(),
            handle_width: 100.0,
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
                // Note: this is the divider index not the column index
                // By using the divider positions only, one doesn't need
                // to keep track of the offset values from the original ones
                // unless a reset is wanted.

                // Adjust the above
                if index == 0 {
                    self.column_heights[index] = value;
                } else {
                    self.column_heights[index] = value - self.divider_values[index-1];
                }
                // Adjust the below
                if index == self.divider_values.len()-1 {
                    self.column_heights[index+1] = self.divider_width - value;
                } else {
                    self.column_heights[index+1] = self.divider_values[index+1] - value;
                }
                
                self.divider_values[index] = value;
            },
        }
    }

    fn view(&self) -> Element<Message> {

        let mut dividers: Vec<Element<Message>> = vec![];
        let mut item_col: Vec<Element<Message>> = vec![];

        for (i, height) in self.column_heights.iter().enumerate() {
            // Add whatever container you want.
            item_col.push(container(
                            text(self.column_heights[i].to_string())
                                    .width(Fill)
                                    .height(Fill)
                                    .align_x(Horizontal::Center)
                                .align_y(Vertical::Center))
                            .width(100.0)
                            .height(*height)
                            .style(move|theme| container::bordered_box(theme))
                            .into());

            // In this case, I don't want one at the end.
            if i < self.column_heights.len()-1 {
                            dividers.push(divider(
                                i,
                                self.divider_values[i],
                                self.range.clone(),
                                self.handle_width,
                                self.handle_height,
                                Message::DividerChange,
                            )
                            .direction(Direction::Vertical)
                            .into());
            }
        };

        // Put the items into  a row
        let col: Element<Message> = 
            column(item_col)
                .width(self.divider_width)
                .into();
        // Insert the row at the beginning so that the dividers are on top
        // You could add a space in the row and let the dividers be on the
        // bottom but then you'll have to play around with the values
        // if the dividers so that they can be seen, not difficult just much
        // easier to let them stay on top.
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


