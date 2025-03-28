

use iced::alignment::Horizontal;
use iced::widget::{center, container, row, stack, text};
use iced::Length::Fill;
use iced::{Element, Size};

use std::ops::RangeInclusive;
use iced_divider::divider::divider;

pub fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .theme(App::theme)
        .antialiasing(true)
        .centered()
        .window_size(Size::new(600.0, 400.0))
        .run()
}

struct App {
    column_widths: [f32; 4],
    divider_values: Vec<f32>,
    range: RangeInclusive<f32>,
    divider_width: f32,
    handle_width: f32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    DividerChange((usize, f32)),
}

impl App {
    fn new() -> Self {
        let column_widths = [100.0; 4];
        App {
            column_widths,
            // Since the default width is 4, adjust the value to line up with the item border
            divider_values: vec![98.0, 198.0, 298.0],
            range: 0.0..=400.0,
            divider_width: column_widths.iter().sum::<f32>(),
            handle_width: 4.0,
        }
    }

    fn title(&self) -> String {
        String::from("Custom Widget - Iced")
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::DividerChange((index, value)) => {
                // Adjust the left side
                if index == 0 {
                    self.column_widths[index] = value;
                } else {
                    self.column_widths[index] = value - self.divider_values[index-1];
                }
                // Adjust the right side
                if index == self.divider_values.len()-1 {
                    self.column_widths[index+1] = self.divider_width - value;
                } else {
                    self.column_widths[index+1] = self.divider_values[index+1] - value;
                }
                
                self.divider_values[index] = value;
            },
        }
    }

    fn view(&self) -> Element<Message> {

        let mut dividers: Vec<Element<Message>> = vec![];
        let mut item_row: Vec<Element<Message>> = vec![];

        for (i, width) in self.column_widths.iter().enumerate() {
            // Add whatever container you want.
            item_row.push(container(
                            text(self.column_widths[i].to_string())
                                    .width(Fill)
                                    .align_x(Horizontal::Center))
                            .width(*width)
                            .style(move|theme| container::bordered_box(theme))
                            .into());

            // In this case, I don't want one at the end.
            if i < self.column_widths.len()-1 {
                            dividers.push(divider(
                                i,
                                self.divider_values[i],
                                self.range.clone(),
                                Message::DividerChange,
                            )
                            .height(21.0)
                            .handle_width(self.handle_width)
                            .into());
            }
        };

        // Put the items into  a row
        let rw: Element<Message> = 
            row(item_row)
                .width(self.divider_width)
                .into();
        // Insert the row at the beginning so that the dividers are on top
        // You could add a space in the row and let the dividers be on the
        // bottom but then you'll have to play around with the values
        // if the dividers so that they can be seen, not difficult just much
        // easier to let them stay on top.
        dividers.insert(0, rw);
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


