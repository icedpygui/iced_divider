

use iced::alignment::Horizontal;
use iced::widget::{center, container, row, stack, text};
use iced::Length::Fill;
use iced::{Element, Size};

use iced_divider::divider::divider_horizontal;

pub fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .theme(App::theme)
        .antialiasing(true)
        .centered()
        .window_size(Size::new(600.0, 400.0))
        .run()
}

struct App {
    column_widths: Vec<f32>,
    handle_width: f32,
    handle_height: f32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    DividerChange((usize, f32)),
}

impl App {
    fn new() -> Self {
        let column_widths = vec![100.0; 4];
        App {
            column_widths,
            handle_width: 4.0,
            handle_height: 21.0,
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
                let diff = self.column_widths[index] - value;

                // Adjust the left side
                self.column_widths[index] = value;
                
                // Adjust the right side
                if index < self.column_widths.len()-1 {
                     self.column_widths[index+1] += diff;
                }
            },
        }
    }

    fn view(&self) -> Element<Message> {

        let mut item_row: Vec<Element<Message>> = vec![];

        for (i, width) in self.column_widths.iter().enumerate() {
            // Add whatever container you want.
            item_row.push(
                container(
                    text(self.column_widths[i].to_string())
                            .width(Fill)
                            .align_x(Horizontal::Center)
                    )
                    .width(*width)
                    .style(move|theme| container::bordered_box(theme))
                    .into());
        }
        
        let div = divider_horizontal(
            self.column_widths.clone(),
            self.handle_width,
            self.handle_height,
            Message::DividerChange
        )
        // using include last handle will prevent the user 
        // from changing the total width
        // since no handle at the end
        // .include_last_handle(false)
        .into();


        // Put the items into  a row
        let rw: Element<Message> = 
            row(item_row).into();
        
        // Put them in a stack with div on top
        let stk = stack([rw, div]);

        // Center everything in the window
        center(stk).into()

    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}


