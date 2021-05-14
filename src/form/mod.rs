use druid::{
    text::Formatter,
    widget::{Flex, TextBox},
    Data, Selector, Widget, WidgetExt, WidgetId,
};

use self::widgets::TextBoxErrorDelegate;

pub mod formatters;
pub mod widgets;

/// Call this where you would normally place a child.
///
/// The selector argument enables this child to be part of a larger form.
/// By sending its status to the AppDelegate, categorised by a Selector,
pub fn form_field_ui<T: Data + std::fmt::Debug>(
    placeholder: &str,
    formatter: impl Formatter<T> + 'static,
    selector: Selector<(WidgetId, bool)>,
) -> impl Widget<T> {
    let widget_id = WidgetId::next();

    // Widgets
    let input = TextBox::new()
        .with_placeholder(placeholder)
        .with_formatter(formatter)
        .validate_while_editing(false)
        .delegate(TextBoxErrorDelegate::new(widget_id));
    let error = widgets::error_display_widget(widget_id, selector);

    // Layout
    Flex::column()
        .with_child(input.expand_width())
        .with_child(error)
}
