mod form;
mod validators;

use druid::{
    widget::{Button, Flex},
    Env,
};
use druid::{AppLauncher, Data, Lens, WidgetExt, WindowDesc};
use form::FormField;

#[derive(Debug, Clone, Data, Lens, Default)]
pub struct AppData {
    name: FormField,
    telephone: FormField,
}

pub fn main() {
    let main_window = WindowDesc::new(
        Flex::column()
            .with_child(form_row!(Name).lens(AppData::name))
            .with_child(form_row!(Telephone).lens(AppData::telephone))
            .with_default_spacer()
            .with_child(
                Button::new("Submit")
                    .on_click(|_, _, _| println!("Success"))
                    .disabled_if(|data: &AppData, _| !data.name.valid || !data.telephone.valid)
                    .expand_width(),
            ),
    );
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(AppData::default())
        .expect("launch failed");
}
