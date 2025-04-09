

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{center, container, column, stack, text};
use iced::Length::Fill;
use iced::{Element, Size};

use iced_divider::divider::divider_vertical;

pub fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .theme(App::theme)
        .antialiasing(true)
        .centered()
        .window_size(Size::new(600.0, 400.0))
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
        let column_heights = vec![50.0; 4];
        App {
            column_heights,
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
                let diff = self.column_heights[index] - value;

                // Adjust the top
                self.column_heights[index] = value;
                
                // Adjust the bottom
                if index < self.column_heights.len()-1 {
                     self.column_heights[index+1] += diff;
                }
            },
        }
    }

    fn view(&self) -> Element<Message> {

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
        };

        let div = divider_vertical(
            self.column_heights.clone(),
            self.handle_width,
            self.handle_height,
            Message::DividerChange
        )
        // using include last handle will prevent the user 
        // from changing the total height
        // since no handle at the end
        // .include_last_handle(false)
        .into();


        // Put the items into  a row
        let col: Element<Message> = 
            column(item_col)
                .into();
        
        // put them in a stack
        let stk = stack([col, div]);

        // Center everything in the window
        center(stk).into()

    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}


