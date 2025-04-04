//! Display an interactive selector of a single value from a range of values to resize containers.
use iced::border::{Border, Radius};
use iced::event::{self, Event};
use iced::advanced::layout;
use iced::{mouse, Background};
use iced::advanced::renderer;
use iced::touch;
use iced::advanced::widget::tree::{self, Tree};
use iced::{
    self, Color, Element, Length, Point,
    Rectangle, Size, Theme,
};
use iced::advanced::{Clipboard, Layout, Shell, Widget};
use std::ops::RangeInclusive;

/// Dividers let users resize an by moving the divider handle..
///
/// # Example
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// #
/// use iced::widget::divider;
///
/// struct State {
///     column_widths: [f32; 2],
///     divider_values: Vec<f32>,
///     range: RangeInclusive<f32>,
/// }
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     DividerChanged((usize, f32)),
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     let mut dividers: Vec<Element<Message>> = vec![];
///     let mut item_row: Vec<Element<Message>> = vec![];
///
///     for (i, width) in self.column_widths.iter().enumerate() {
///         // Add whatever container you want.
///         item_row.push(container(
///                         text(self.column_widths[i].to_string())
///                             .width(Fill)
///                             .align_x(Horizontal::Center))
///                         .width(*width)
///                         .style(move|theme| container::bordered_box(theme))
///                         .into());
///
///         // In this case, we don't want one at the end.
///         if i < self.column_widths.len()-1 {
///                         dividers.push(divider(
///                             i,
///                             self.divider_values[i],
///                             self.range.clone(),
///                             Message::DividerChange,
///                         )
///                         .into());
///         }
///     };
///
///     // Put the items into a row
///     let rw: Element<Message> = 
///         row(item_row)
///             .width(self.divider_width)
///             .into();
///     // Insert the row at the beginning so that the dividers are on top
///     // You could add a space in the row and let the dividers be on the
///     // bottom but then you'll have to play around with the stating values
///     // of the dividers so that they can be seen, not difficult just much
///     // easier to let them stay on top.
///     dividers.insert(0, rw);
///     // put them in a stack
///     let stk = stack(dividers);
///     // Center everything in the window
///     center(stk).into()
/// }
///
/// fn update(state: &mut State, message: Message) {
///     match message {
///         Message::DividerChange((index, value)) => {
///            // Adjust the left side
///            if index == 0 {
///                self.column_widths[index] = value;
///            } else {
///                self.column_widths[index] = value - self.divider_values[index-1];
///            }
///            // Adjust the right side
///            if index == self.divider_values.len()-1 {
///                self.column_widths[index+1] = self.divider_width - value;
///            } else {
///                self.column_widths[index+1] = self.divider_values[index+1] - value;
///            }
///        }
///            self.divider_values[index] = value;
///     }
/// }
/// ```

pub fn divider<'a, Message, Theme>(
    index: usize,
    value: f32,
    range: std::ops::RangeInclusive<f32>,
    handle_width: f32,
    handle_height: f32,
    on_change: impl Fn((usize, f32)) -> Message + 'a,
) -> Divider<'a, Message, Theme>
where
    Message: Clone,
    Theme: Catalog + 'a,
{
    Divider::new(
            index, 
            value,
            range, 
            handle_width, 
            handle_height, 
            on_change)
}


#[allow(missing_debug_implementations)]
pub struct Divider<'a, Message, Theme = iced::Theme>
where
    Theme: Catalog,
{
    index: usize,
    value: f32,
    range: RangeInclusive<f32>,
    handle_width: f32,
    handle_height: f32,
    step: f32,
    on_change: Box<dyn Fn((usize, f32)) -> Message + 'a>,
    on_release: Option<Message>,
    width: Length,
    height: Length,
    direction: Direction,
    class: Theme::Class<'a>,
}

impl<'a, Message, Theme> Divider<'a, Message, Theme>
where
    Message: Clone,
    Theme: Catalog,
{
    /// The default height of a [`Divider`].
    pub const DEFAULT_HEIGHT: f32 = 21.0;

    /// Creates a new [`Divider`].
    ///
    /// It expects:
    ///   * the index of the divider, used as an id
    ///   * the current value of the [`Divider`]
    ///   * an inclusive range of possible values
    ///   * a function that will be called when the [`Divider`] is dragged.
    ///     It receives the new value of the [`Divider`] and must produce a
    ///     `Message`.
    pub fn new<F>(
        index: usize,
        value: f32,
        range: RangeInclusive<f32>,
        handle_width: f32,
        handle_height: f32, 
        on_change: F) 
        -> Self
    where
        F: 'a + Fn((usize, f32)) -> Message,
    {
        let value = if value >= *range.start() {
            value
        } else {
            *range.start()
        };

        let value = if value <= *range.end() {
            value
        } else {
            *range.end()
        };

        Divider {
            index,
            value,
            range,
            handle_width,
            handle_height,
            step: 1.0,
            on_change: Box::new(on_change),
            on_release: None,
            width: Length::Fill,
            height: Length::Fill,
            direction: Direction::Horizontal,
            class: Theme::default(),
        }
    }

    /// Sets the release message of the [`Divider`].
    /// This is called when the mouse is released from the Divider.
    ///
    /// Typically, the user's interaction with the Divider is finished when this message is produced.
    /// This is useful if you need to spawn a long-running task from the Divider's result, where
    /// the default on_change message could create too many events.
    pub fn on_release(mut self, on_release: Message) -> Self {
        self.on_release = Some(on_release);
        self
    }
    /// Sets the width of the [`Divider`] which usually spans the entire width of the items.
    /// If shorter that the entire width, could act as a max width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Divider`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the step size of the [`Divider`].
    pub fn step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    /// Sets the direction of the [`Divided`].
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the style of the [`Divider`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the style class of the [`Divider`].
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Divider<'a, Message, Theme>
where
    Message: Clone,
    Theme: Catalog,
    Renderer: iced::advanced::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, self.width, self.height)
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        let state = tree.state.downcast_mut::<State>();
        let value: f64 = self.value.clone().into();
        let is_dragging = state.is_dragging;

        let locate = |cursor_position: Point| -> Option<f32> {
            let bounds = layout.bounds();
            match self.direction {
                Direction::Horizontal => {
                    let new_value = if cursor_position.x <= bounds.x {
                        Some(*self.range.start())
                    } else if cursor_position.x >= bounds.x + bounds.width {
                        Some(*self.range.end())
                    } else {
                        let step = self.step;

                        let start = *self.range.start();
                        let end = *self.range.end();

                        let percent = f32::from(cursor_position.x - bounds.x)
                            / f32::from(bounds.width);

                        let steps = (percent * (end - start) / step ).round();
                        let value = steps * step + start;

                        Some(value.min(end))
                    };

                    new_value
                },
                Direction::Vertical => {
                    let new_value = if cursor_position.y <= bounds.y {
                        Some(*self.range.start())
                    } else if cursor_position.y >= bounds.y + bounds.height {
                        Some(*self.range.end())
                    } else {
                        let step = self.step;

                        let start = *self.range.start();
                        let end = *self.range.end();

                        let percent = f32::from(cursor_position.y - bounds.y)
                            / f32::from(bounds.height);

                        let steps = (percent * (end - start) / step ).round();
                        let value = steps * step + start;

                        Some(value.min(end))
                    };

                    new_value
                },
            }
        };
        
        let change = |new_value: f32| {
            if (self.value - new_value).abs() > f32::EPSILON {
                shell.publish((self.on_change)((self.index, new_value)));

                self.value = new_value;
            }
        };
        
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let bounds = layout.bounds();
                let mut handle_bounds = layout.bounds();
                
                match self.direction {
                    Direction::Horizontal => {
                        handle_bounds.x = bounds.x + value as f32;
                        handle_bounds.width = self.handle_width;
                        handle_bounds.height = self.handle_height;
                    },
                    Direction::Vertical => {
                        handle_bounds.y = bounds.y + value as f32;
                        handle_bounds.width = self.handle_width;
                        handle_bounds.height = self.handle_height;
                    },
                }
                
                if let Some(cursor_position) =
                    cursor.position_over(handle_bounds)
                {
                    let _ = locate(cursor_position).map(change);
                    state.is_dragging = true;
                
                    return event::Status::Captured;
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                if is_dragging {
                    if let Some(on_release) = self.on_release.clone() {
                        shell.publish(on_release);
                    }
                    state.is_dragging = false;

                    return event::Status::Captured;
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(touch::Event::FingerMoved { .. }) => {
                if is_dragging {
                    let _ = cursor.position().and_then(locate).map(change);

                    return event::Status::Captured;
                }
            }
            _ => {}
        }

        event::Status::Ignored
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();
        let is_mouse_over = cursor.is_over(bounds);
        
        let status = if state.is_dragging {
            Status::Dragged
        } else if is_mouse_over {
            Status::Hovered
        } else {
            Status::Active
        };

        let style = theme.style(&self.class, status);
        
        let value = self.value;
        let (range_start, range_end) = {
            let (start, end) = self.range.clone().into_inner();

            (start, end)
        };

        let offset = if range_start >= range_end {
            0.0
        } else {
            match self.direction {
                Direction::Horizontal => {
                    bounds.width * (value - range_start)
                    / (range_end - range_start)
                },
                Direction::Vertical => {
                    (bounds.height - self.handle_height) * (value - range_start)
                        / (range_end - range_start)
                },
            }
            
        };

        match self.direction {
            Direction::Horizontal => {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle {
                            x: bounds.x + offset,
                            y: bounds.y,
                            width: self.handle_width,
                            height: self.handle_height,
                        },
                        border: Border {
                            radius: style.border_radius,
                            width: style.border_width,
                            color: style.border_color,
                        },
                        ..renderer::Quad::default()
                    },
                    style.background,
                );
            },
            Direction::Vertical => {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle {
                            x: bounds.x,
                            y: bounds.y + offset,
                            width: self.handle_width,
                            height: self.handle_height,
                        },
                        border: Border {
                            radius: style.border_radius,
                            width: style.border_width,
                            color: style.border_color,
                        },
                        ..renderer::Quad::default()
                    },
                    style.background,
                );
            },
        }

        
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();
        let handle_bounds = match self.direction {
            Direction::Horizontal => {
                let value: f64 = self.value.clone().into();
                let mut handle_bounds = layout.bounds();
                handle_bounds.x = bounds.x + value as f32;
                handle_bounds.width = self.handle_width;
                handle_bounds.height = self.handle_height;
                handle_bounds
            },
            Direction::Vertical => {
                let value: f64 = self.value.clone().into();
                let mut handle_bounds = layout.bounds();
                handle_bounds.y = bounds.y + value as f32;
                handle_bounds.width = self.handle_width;
                handle_bounds.height = self.handle_height;
                handle_bounds
            },
        };
        
        let is_mouse_over = cursor.is_over(handle_bounds);

        if state.is_dragging || is_mouse_over{
            match self.direction {
                Direction::Horizontal => mouse::Interaction::ResizingHorizontally,
                Direction::Vertical => mouse::Interaction::ResizingVertically,
            }
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<'a, Message, Theme, Renderer> From<Divider<'a, Message, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    fn from(
        divider: Divider<'a, Message, Theme>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(divider)
    }
}

/// The direction of [`Scrollable`].
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Direction {
    /// Horizontal resizing
    #[default]
    Horizontal,
    /// Vertical resizing
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct State {
    is_dragging: bool,
}

/// The possible status of a [`Divider`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The [`Divider`] can be interacted with.
    Active,
    /// The [`Divider`] is being hovered.
    Hovered,
    /// The [`Divider`] is being dragged.
    Dragged,
}

/// The appearance of a Divider.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The [`Background`] of the handle.
    pub background: Background,
    /// The border width of the handle.
    pub border_width: f32,
    /// The border [`Color`] of the handle.
    pub border_color: Color,
    /// The border [`Radius`] of the handle.
    pub border_radius: Radius,
}

/// The theme catalog of a [`Divider`].
pub trait Catalog: Sized {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

/// A styling function for a [`Divider`].
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// The default style of a [`Divider`].
pub fn primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    let color = match status {
        Status::Active => palette.primary.strong.color,
        Status::Hovered => palette.primary.base.color,
        Status::Dragged => palette.primary.strong.color,
    };

    Style {
        background: color.into(),
        border_color: Color::TRANSPARENT,
        border_width: 0.0,
        border_radius: 0.0.into()
    }
}

pub fn transparent(theme: &Theme, status: Status) -> Style {
    let mut style = primary(theme, status);
    style.background = Color::TRANSPARENT.into();
    style
}