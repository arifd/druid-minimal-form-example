mod validation;
use std::collections::HashMap;
use std::sync::Arc;
use validation::{formatters::*, widgets::TextBoxErrorDelegate};

use druid::{
    text::Formatter,
    widget::{Button, Flex, TextBox},
    AppDelegate, Command, DelegateCtx, Env, Handled, Key, Selector, Target, Widget,
};
use druid::{AppLauncher, Data, Lens, WidgetExt, WidgetId, WindowDesc};

// A Selector to track the validity of your forms
const NAME_OF_FORM: Selector<(WidgetId, bool)> = Selector::new("NAME_OF_FORM");

#[derive(Debug, Clone, Data, Lens, Default)]
pub struct AppData {
    name: Arc<String>,
    telephone: Arc<String>,
    valid: bool,
}

pub fn main() {
    let main_window = WindowDesc::new(
        Flex::column()
            .with_child(form_field_ui("Name", NameFormatter, NAME_OF_FORM).lens(AppData::name))
            .with_child(
                form_field_ui("Telephone", TelephoneFormatter, NAME_OF_FORM)
                    .lens(AppData::telephone),
            )
            .with_default_spacer()
            .with_child(
                Button::new("Submit")
                    .disabled_if(|data: &AppData, _| !data.valid)
                    .expand_width(),
            ),
    );
    AppLauncher::with_window(main_window)
        .delegate(Delegate::new())
        .log_to_console()
        .launch(AppData::default())
        .expect("launch failed");
}

fn form_field_ui<T: Data + std::fmt::Debug>(
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
    let error = validation::widgets::error_display_widget(widget_id, selector);

    // Layout
    Flex::column()
        .with_child(input.expand_width())
        .with_child(error)
}

pub struct Delegate {
    form_1: HashMap<WidgetId, bool>,
}

impl Delegate {
    pub fn new() -> Self {
        Self {
            form_1: HashMap::new(),
        }
    }
}

impl AppDelegate<AppData> for Delegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppData,
        _env: &Env,
    ) -> Handled {
        if let Some((id, validity)) = cmd.get(NAME_OF_FORM) {
            self.form_1.insert(*id, *validity);
            data.valid = self.form_1.iter().all(|(_, valid)| *valid);
            dbg!(&data.valid);
        }

        Handled::No
    }
}
