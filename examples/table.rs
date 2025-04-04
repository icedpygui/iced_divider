

use iced::alignment::Horizontal;
use iced::widget::{button, center, column, container, row, stack, text, vertical_space};
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

struct App <'a>{
    column_names: [&'a str; 3],
    column_widths: [f32; 3],
    column_divider_values: Vec<f32>,
    column_range: RangeInclusive<f32>,
    column_divider_width: f32,
    row_heights: Vec<f32>,
    row_divider_values: Vec<f32>,
    row_range: RangeInclusive<f32>,
    row_divider_height: f32,
    column_handle_width: f32,
    column_handle_height: f32,
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
        let column_widths = [100.0; 3];
        let row_heights = vec![50.0; 3];
        App {
            column_names: ["Col 1" ,"Col 2" ,"Col 3"],
            column_widths,
            // Since the default width is 4, adjust the value to line up with the item border
            column_divider_values: vec![98.0, 200.0],
            column_range: 0.0..=300.0,
            // The divider widths span the entire width of the columns
            column_divider_width: column_widths.iter().sum::<f32>(),
            row_heights: row_heights.clone(),
            row_divider_values: vec![50.0, 100.0],
            row_range: 0.0..=150.0,
            row_divider_height: row_heights.iter().sum::<f32>(),
            column_handle_width: 4.0,
            column_handle_height: 21.0,
            row_handle_width: column_widths.iter().sum::<f32>(),
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
                // Note: this is the divider index not the column index
                // By using the divider positions only, one doesn't need
                // to keep track of the offset values from the original ones
                // unless a reset is wanted.
                
                // Adjust the left side
                if index == 0 {
                    self.column_widths[index] = value;
                } else {
                    self.column_widths[index] = value - self.column_divider_values[index-1];
                }
                // Adjust the right side
                if index == self.column_divider_values.len()-1 {
                    self.column_widths[index+1] = self.column_divider_width - value;
                } else {
                    self.column_widths[index+1] = self.column_divider_values[index+1] - value;
                }
                
                self.column_divider_values[index] = value;
            },
            Message::RowDividerChange((index, value)) => {
                // Note: this is the divider index not the column index
                // By using the divider positions only, one doesn't need
                // to keep track of the offset values from the original ones
                // unless a reset is wanted.
                
                // Adjust the above
                if index == 0 {
                    self.row_heights[index] = value;
                } else {
                    self.row_heights[index] = value - self.row_divider_values[index-1];
                }
                // Adjust the below                    
                if index == self.row_divider_values.len()-1 {
                    self.row_heights[index+1] = self.row_divider_height - value;
                } else {
                    self.row_heights[index+1] = self.row_divider_values[index+1] - value;
                }
                
                self.row_divider_values[index] = value;
            },
        }
    }

    fn view(&self) -> Element<Message> {

        let mut header_dividers: Vec<Element<Message>> = vec![];
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

            // In this case, I don't want one at the end.
            if i < self.column_widths.len()-1 {
                            header_dividers.push(divider(
                                i,
                                self.column_divider_values[i],
                                self.column_range.clone(),
                                self.column_handle_width,
                                self.column_handle_height,
                                Message::ColumnDividerChange,
                            )
                            .into());
            }
        };

        // Put the items into  a row
        let header_row: Element<Message> = 
            row(item_row)
                .width(self.column_divider_width)
                .into();
        // Insert the row at the beginning so that the dividers are on top
        // You could add a space in the row and let the dividers be on the
        // bottom but then you'll have to play around with the values
        // if the dividers so that they can be seen, not difficult just much
        // easier to let them stay on top.
        header_dividers.insert(0, header_row);

        // put them in a stack
        let header_stk = stack(header_dividers);

        // Add some rows
        let mut rows: Vec<Element<Message>> = vec![];
        let mut row_dividers: Vec<Element<Message>> = vec![];

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

            // Not putting a divider on last row
            // if you do, some changes will have to be made in the callback message
            if index != 2 {
                row_dividers.push(divider(
                        index, 
                        self.row_divider_values[index], 
                        self.row_range.clone(), 
                        self.row_handle_width, 
                        self.row_handle_height, 
                        Message::RowDividerChange)
                        .direction(Direction::Vertical)
                        .width(self.column_widths.iter().sum::<f32>())
                        .into()
                        );
            }
        }
        // insert the rows at the beginning so the dividers are on top.
        row_dividers.insert(0, column(rows).into());
        let row_stk = stack(row_dividers);

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


