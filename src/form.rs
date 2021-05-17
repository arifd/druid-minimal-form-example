use druid::{Data, Lens};
use std::sync::Arc;

#[derive(Clone, Debug, Data, Lens, Default)]
pub struct FormField {
    pub field: Arc<String>,
    pub valid: bool,
}

#[macro_export]
macro_rules! form_row {
    ($name:ident) => {{
        {
            // Point to your different formatters/validators here:
            macro_rules! validate {
                (Name, $data:ident) => {
                    validators::letters($data);
                };
                (Telephone, $data:ident) => {
                    validators::numbers($data);
                };
            }

            use crate::form::FormField;
            use druid::{
                widget::{Controller, Label, Painter, TextBox},
                Event, EventCtx, RenderContext, Widget,
            };

            struct Validator;
            impl<W: Widget<FormField>> Controller<FormField, W> for Validator {
                fn event(
                    &mut self,
                    child: &mut W,
                    ctx: &mut EventCtx,
                    event: &Event,
                    data: &mut FormField,
                    env: &Env,
                ) {
                    if let Event::KeyUp(_) = event {
                        validate!($name, data);
                    } else {
                        // Pass on all other events...
                        child.event(ctx, event, data, env)
                    }
                }
            }
            let name = Label::new(stringify!($name));

            let valid_painter = Painter::new(move |ctx, data: &FormField, _| {
                ctx.fill(
                    druid::kurbo::Circle::new((5.0, 5.0), 5.0),
                    if data.valid {
                        &druid::Color::GREEN
                    } else {
                        &druid::Color::RED
                    },
                );
            });

            let input = TextBox::new()
                .with_placeholder(stringify!($name))
                .lens(FormField::field)
                .controller(Validator);

            Flex::row()
                .with_flex_child(name.expand_width(), 0.25)
                .with_child(valid_painter.fix_size(10., 10.))
                .with_default_spacer()
                .with_flex_child(input.expand_width(), 1.)
        }
    }};
}
