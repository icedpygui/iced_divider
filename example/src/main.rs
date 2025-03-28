

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
    column_names: Vec<String>,
    column_widths: [f32; 4],
    divider_values: Vec<f32>,
    range: RangeInclusive<f32>,
    divider_width: f32,
    handle_widths: Vec<f32>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    DividerChange((usize, f32)),
}

impl App {
    fn new() -> Self {
        let column_widths = [100.0; 4];
        App {
            column_names: vec!["Text1".to_string(), 
                                "Text2".to_string(), 
                                "Text3".to_string(), 
                                "Text4".to_string()],
            column_widths,
            divider_values: vec![98.0, 198.0, 298.0],
            range: 0.0..=406.0,
            divider_width: column_widths.iter().sum::<f32>() + 4.0 * (column_widths.len() - 1) as f32,
            handle_widths: vec![4.0; 4],
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
                if index == 0 {
                    self.column_widths[index] = value;
                } else {
                    self.column_widths[index] = value - self.divider_values[index-1];
                }
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
        let mut top_row: Vec<Element<Message>> = vec![];

        for (i, width) in self.column_widths.iter().enumerate() {
        
            top_row.push(container(text(self.column_widths[i].to_string())
                                            .width(Fill)
                                            .align_x(Horizontal::Center))
                            .width(*width)
                            .style(move|theme| container::bordered_box(theme))
                            .into());
            if i < self.column_widths.len()-1 {
                            dividers.push(divider(
                                4,
                                i,
                                self.divider_values[i],
                                self.range.clone(),
                                Message::DividerChange,
                            )
                            .width(self.divider_width)
                            .height(21.0)
                            .handle_width(self.handle_widths.clone())
                            .into());
            }
        };

        let rw: Element<Message> = 
            row(top_row)
                // .spacing(3.0)
                .width(self.divider_width)
                .into();
        dividers.insert(0, rw);
        let stk = stack(dividers);

        center(stk).into()
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}


