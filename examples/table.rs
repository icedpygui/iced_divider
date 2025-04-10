//Table Demo
use iced::alignment::Horizontal;
use iced::widget::{button, center, column, container, row, stack, text, vertical_space};
use iced::Length::Fill;
use iced::{Element, Size};

use iced_divider::divider::{divider_horizontal, divider_vertical};

pub fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .theme(App::theme)
        .antialiasing(true)
        .centered()
        .window_size(Size::new(600.0, 400.0))
        .run()
}

struct App <'a>{
    column_names: Vec<&'a str>,
    column_widths: Vec<f32>,
    row_heights: Vec<f32>,
    header_handle_width: f32,
    header_handle_height: f32,
    row_handle_width: f32,
    row_handle_height: f32,
}

#[derive(Debug, Clone)]
enum Message {
    ColumnDividerChange((usize, f32)),
    RowDividerChange((usize, f32)),
}

impl <'a> App <'a>{
    fn new() -> Self {
        App {
            column_names: vec!["Col 1" ,"Col 2" ,"Col 3"],
            column_widths: vec![100.0; 3],
            row_heights: vec![50.0; 3],
            header_handle_width: 4.0,
            header_handle_height: 21.0,
            row_handle_width: 300.0,
            row_handle_height: 4.0,
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
            Message::ColumnDividerChange((index, value)) => {
            
                let diff = self.column_widths[index] - value;

                // Adjust the left side
                self.column_widths[index] = value;
                
                // Adjust the right side
                if index < self.column_widths.len()-1 {
                     self.column_widths[index+1] += diff;
                }

                // adjust divider width if table changes width
                // using the last divider
                if index == self.column_widths.len()-1 {
                    self.row_handle_width = self.column_widths.iter().sum();
                }
            },
            Message::RowDividerChange((index, value)) => {

                let diff = self.row_heights[index] - value;

                // Adjust the left side
                self.row_heights[index] = value;
                
                // Adjust the right side
                if index < self.row_heights.len()-1 {
                     self.row_heights[index+1] += diff;
                }
            },
        }
    }

    fn view(&self) -> Element<Message> {

        let mut item_row: Vec<Element<Message>> = vec![];

        for (i, width) in self.column_widths.iter().enumerate() {
            // Add whatever container you want.
            item_row.push(container(
                            text(self.column_names[i])
                                    .width(Fill)
                                    .align_x(Horizontal::Center)
                            )
                            .width(*width)
                            .style(move|theme| container::bordered_box(theme))
                            .into());

        };

        let header_divider = 
            divider_horizontal(
                self.column_widths.clone(), 
                self.header_handle_width, 
                self.header_handle_height, 
                Message::ColumnDividerChange)
                .into();

        // Put the items into  a row
        let header: Element<Message> = 
            row(item_row).into();
        
        // put them in a stack
        let header_stk = stack([header, header_divider]);

        // Add some rows
        let mut rows: Vec<Element<Message>> = vec![];

        for index in 0..3 {
            rows.push(row![
                button("Button").width(self.column_widths[0]).height(Fill),
                container(text("0").width(Fill).height(31.0).center())
                    .width(self.column_widths[1])
                    .height(Fill)
                    .style(move|theme| container::bordered_box(theme)),
                container(text(format!("Row {index}")).width(Fill).height(31.0).center())
                    .width(self.column_widths[2])
                    .height(Fill)
                    .style(move|theme| container::bordered_box(theme)),
            ].height(self.row_heights[index]).into());
        }

        // add the divider
        let row_div = 
            divider_vertical(
                self.row_heights.clone(), 
                self.row_handle_width, 
                self.row_handle_height, 
                Message::RowDividerChange)
                .into();


        // make the stack
        let col = column(rows).into();
        let row_stk = stack([col, row_div]);

        let sp = vertical_space().height(5.0).into();

        // rows.insert(0, vertical_space().height(5.0).into());
        let col = column(vec![header_stk.into(), sp, row_stk.into()]);

        // Center everything in the window
        center(col).into()

    }
}

impl <'a>Default for App <'a>{
    fn default() -> Self {
        Self::new()
    }
}


