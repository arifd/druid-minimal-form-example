mod form;

use druid::{
    widget::{Button, Flex},
    AppDelegate, Command, DelegateCtx, Env, Handled, Selector, Target,
};
use druid::{AppLauncher, Data, Lens, WidgetExt, WidgetId, WindowDesc};
use form::{form_field_ui, formatters::*};
use std::collections::HashMap;
use std::sync::Arc;

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
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppData,
        _env: &Env,
    ) -> Handled {
        if let Some((id, validity)) = cmd.get(NAME_OF_FORM) {
            self.form_1.insert(*id, *validity);
            data.valid = self.form_1.iter().all(|(_, valid)| *valid);
            return Handled::Yes;
        }

        Handled::No
    }
}
