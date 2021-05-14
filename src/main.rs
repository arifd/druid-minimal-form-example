mod validation;
use std::sync::{atomic::AtomicU16, Arc};
use validation::{formatters::*, widgets::TextBoxErrorDelegate};

use druid::{
    text::Formatter,
    widget::{Button, Flex, TextBox},
    Widget,
};
use druid::{AppLauncher, Data, Lens, WidgetExt, WidgetId, WindowDesc};

#[derive(Debug, Clone, Data, Lens, Default)]
pub struct AppData {
    name: Arc<String>,
    telephone: Arc<String>,
}

pub fn main() {
    let main_window = WindowDesc::new(
        Flex::column()
            .with_child(form_field_ui("Name", NameFormatter).lens(AppData::name))
            .with_child(form_field_ui("Telephone", TelephoneFormatter).lens(AppData::telephone))
            .with_default_spacer()
            .with_child(
                Button::new("Submit")
                    .disabled_if(|_data: &AppData, _| true) // TODO
                    .expand_width(),
            ),
    );
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(AppData::default())
        .expect("launch failed");
}

fn form_field_ui<T: Data + std::fmt::Debug>(
    placeholder: &str,
    formatter: impl Formatter<T> + 'static,
) -> impl Widget<T> {
    // Unique identifier
    static ID: AtomicU16 = AtomicU16::new(1);
    let widget_id = WidgetId::reserved(ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed));

    // Widgets
    let input = TextBox::new()
        .with_placeholder(placeholder)
        .with_formatter(formatter)
        .validate_while_editing(false)
        .delegate(TextBoxErrorDelegate::new(widget_id));
    let error = validation::widgets::error_display_widget(widget_id);

    // Layout
    Flex::column()
        .with_child(input.expand_width())
        .with_child(error)
}
