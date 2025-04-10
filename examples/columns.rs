//! Example

use iced::widget::{button, column, container, row, stack, text};

use iced::{Color, Element, Padding, Size};

use iced_divider::divider::{self, divider_horizontal};

pub fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .theme(App::theme)
        .antialiasing(true)
        .centered()
        .window_size(Size::new(700.0, 400.0))
        .run()
}

struct App {
    column_widths1: Vec<f32>,
    column_widths2: Vec<f32>,
    handle_width: f32,
    handle_height: f32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    DividerChange1((usize, f32)),
    DividerChange2((usize, f32)),
}

impl App {
    fn new() -> Self {
        App {
            column_widths1: vec![300.0; 2],
            column_widths2: vec![300.0; 2],
            handle_width: 4.0, // defaults to 4 just using for demo info
            handle_height: 200.0, // needs to be the same height as the row
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
            Message::DividerChange1((index, value)) => {
                let diff = self.column_widths1[index] - value;

                // Adjust the left side
                self.column_widths1[index] = value;
                
                // Adjust the right side
                if index < self.column_widths1.len()-1 {
                     self.column_widths1[index+1] += diff;
                }
            },

            Message::DividerChange2((index, value)) => {
                let diff = self.column_widths2[index] - value;

                // Adjust the left side
                self.column_widths2[index] = value;
                
                // Adjust the right side
                if index < self.column_widths2.len()-1 {
                     self.column_widths2[index+1] += diff;
                }
            },

        }
    }

    fn view(&self) -> Element<Message> {
        
        let div1 = divider_horizontal(
            self.column_widths1.clone(),
            self.handle_width,
            self.handle_height,
            Message::DividerChange1
        )
        .style(|theme, status| {
            divider::transparent(theme, status)
        })
        .into();

        let div2 = divider_horizontal(
            self.column_widths2.clone(),
            self.handle_width,
            self.handle_height,
            Message::DividerChange2
        )
        .style(|theme, status| {
            divider::transparent(theme, status)
        })
        // excludes the last divider to prevent total width resize
        .include_last_handle(false)
        .into();
        
        let str1 = 
            "By including the last \ndivider(default)\nThe column can be \nexpanded/shortened";

        let str2 = 
            "The last divider is excluded\nfixing the total width";

        // Put the columns into a row
        let rw1 = 
            row(get_children(&self.column_widths1, str1))
            .height(self.handle_height).into();
        let rw2 = 
            row(get_children(&self.column_widths2, str2))
            .height(self.handle_height)
            .into();

        // put them in a stack
        let stk1 = stack([rw1, div1]).into();
        let stk2 = stack([rw2, div2]).into();

        let col = column([stk1, stk2]).spacing(20);

        // Put in a container
        container(col)
        .padding(Padding{ top: 50.0, right: 0.0, bottom: 0.0, left: 50.0 })
        .into()

    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

fn get_children<'a>(widths: &Vec<f32>, txt: &'a str) -> Vec<Element<'a, Message>> {

    let mut items: Vec<Element<Message>> = vec![];

    for width in widths.iter() {
        // Add whatever container you want.
        items.push(
            container(
            column![
                text(format!("Width = {}", width)),
                text(txt),
                button("Some Button"),
                button("Another button"),
                ]           
                .width(*width)
            ).style(|_|{
                let mut style = container::Style::default();
                style.border.color = Color::WHITE;
                style.border.width = 1.0;
                style
            }).into()
        );
    };
    items
}
