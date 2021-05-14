use druid::{
    text::ValidationError,
    widget::{Either, Label, SizedBox, TextBoxEvent, ValidationDelegate},
    BoxConstraints, Color, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, Point, Selector, Size, UpdateCtx, Widget, WidgetExt, WidgetId, WidgetPod,
};

///////////////////////////////////////////////////////////////////////////////
// ERROR DISPLAY WIDGET                                                      //
///////////////////////////////////////////////////////////////////////////////

/// Create a widget that will display errors.
///
/// The `id` param is the `WidgetId` that this widget should use; that id
/// will be sent messages when it should display or clear an error.
pub fn error_display_widget<T: Data>(id: WidgetId) -> impl Widget<T> {
    ErrorController::new(
        Either::new(
            |d: &Option<ValidationError>, _| d.is_some(),
            Label::dynamic(|d: &Option<ValidationError>, _| {
                d.as_ref().map(|d| d.to_string()).unwrap_or_default()
            })
            .with_text_color(ERROR_TEXT_COLOR)
            .with_text_size(12.0),
            SizedBox::empty(),
        )
        .with_id(id),
    )
}

///////////////////////////////////////////////////////////////////////////////
// ERROR CONTROLLER                                                          //
///////////////////////////////////////////////////////////////////////////////

/// A widget that manages a child which can display an error.
///
/// This is not a blessed pattern, but works around certain limitations of Druid,
/// using Commands.
///
/// The basic idea is that this widget owns an `Option<Error>`, and it either
/// clears or sets this error based on `Command`s sent to it from some other
/// widget.
///
/// It's child's data is this `Option<Error>`; the incoming data is ignored
/// completely.
struct ErrorController<W> {
    child: WidgetPod<Option<ValidationError>, W>,
    error: Option<ValidationError>,
}

impl<W: Widget<Option<ValidationError>>> ErrorController<W> {
    fn new(child: W) -> ErrorController<W> {
        ErrorController {
            child: WidgetPod::new(child),
            error: None,
        }
    }
}

impl<T, W: Widget<Option<ValidationError>>> Widget<T> for ErrorController<W> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(SHOW_ERROR) => {
                self.error = Some(cmd.get_unchecked(SHOW_ERROR).to_owned());
                ctx.request_update();
            }
            Event::Command(cmd) if cmd.is(CLEAR_ERROR) => {
                self.error = None;
                ctx.request_update();
            }
            _ => self.child.event(ctx, event, &mut self.error, env),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _: &T, env: &Env) {
        self.child.lifecycle(ctx, event, &self.error, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _: &T, _: &T, env: &Env) {
        self.child.update(ctx, &self.error, env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _: &T, env: &Env) -> Size {
        let size = self.child.layout(ctx, bc, &self.error, env);
        self.child.set_origin(ctx, &self.error, env, Point::ZERO);
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _: &T, env: &Env) {
        self.child.paint(ctx, &self.error, env);
    }

    fn id(&self) -> Option<WidgetId> {
        Some(self.child.id())
    }
}

///////////////////////////////////////////////////////////////////////////////
// TEXTBOX ERROR DELEGATE                                                    //
///////////////////////////////////////////////////////////////////////////////
const ERROR_TEXT_COLOR: Color = Color::rgb8(0xB6, 0x00, 0x04);

/// Sent by the [`TextBoxErrorDelegate`] when an error should be displayed.
const SHOW_ERROR: Selector<ValidationError> =
    Selector::new("druid-minimal-form-validation.show-error");

/// Sent by the [`TextBoxErrorDelegate`] when an error should be cleared.
const CLEAR_ERROR: Selector = Selector::new("druid-minimal-form-validation.clear-error");

pub struct TextBoxErrorDelegate {
    target: WidgetId,
}

impl TextBoxErrorDelegate {
    pub fn new(target: WidgetId) -> TextBoxErrorDelegate {
        TextBoxErrorDelegate { target }
    }
}

impl ValidationDelegate for TextBoxErrorDelegate {
    fn event(&mut self, ctx: &mut EventCtx, event: TextBoxEvent, _current_text: &str) {
        match event {
            TextBoxEvent::Changed => {
                ctx.submit_command(CLEAR_ERROR.to(self.target));
            }
            TextBoxEvent::PartiallyInvalid(err) => {
                ctx.submit_command(SHOW_ERROR.with(err.to_owned()).to(self.target));
            }
            _ => (),
        }
    }
}
