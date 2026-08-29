use crate::common::LauncherState;
use crate::common::Message;
use crate::error_collector::WaymacErrorVector;

use iced::widget::column;
use iced::widget::container;
use iced::widget::container::Style;
use iced::widget::{scrollable, text};
use iced::{Background, Border, Color, Element, Length};

use crate::data::{LAUNCHER_CONTAINER_ID, LAUNCHER_SCROLLABLE_ID, LAUNCHER_TEXT_INPUT_ID};

pub fn view<Theme, Renderer>(state: &LauncherState) -> Element<'_, Message> {
    container(scrollable(column(
        state
            .errors_detected
            .clone()
            .into_iter()
            .map(|error| text(error).into()),
    )))
    .style(|_| Style {
        background: state.config.main_window.background,
        border: (|| match state.config.main_window.border {
            Some(bor) => bor,
            None => Border {
                width: 0.,
                ..Default::default()
            },
        })(),
        ..Default::default()
    })
    .width(state.config.main_window.size.width)
    .height(state.config.main_window.size.height)
    .into()
}
