use iced::Task;
use iced::widget::operation::AbsoluteOffset;
use iced::widget::{Id as IcedId, operation::scroll_by};

use log::info;

use crate::common::{LauncherState, Message};
use crate::data::{ENTRY_ELEMENTS_HEIGHT, LAUNCHER_SCROLLABLE_ID};

// given an offset (mostly set by the scroll), change the focus entry in the launcher
pub fn change_focus(state: &mut LauncherState, offset: i32) -> Task<Message> {
    let limit_entry_id =
        |state: &LauncherState, offset: i32| match state.focus_desktop_entry_id as i32 + offset {
            val if (val < 0
                || val
                    > (state
                        .ui_desktop_entries
                        .clone()
                        .unwrap_or(Box::new(Vec::new()))
                        .len() as i32)
                        - 1) =>
            {
                (state.focus_desktop_entry_id as i32) as usize
            }
            _ => (state.focus_desktop_entry_id as i32 + offset) as usize,
        };

    let old_focus_desktop_entry_id = state.focus_desktop_entry_id;
    state.focus_desktop_entry_id = limit_entry_id(&state, offset);

    info!("entry num: {}", state.focus_desktop_entry_id);

    return Task::batch(vec![
        Task::done(Message::ToogleFocusDesktopEntry(
            old_focus_desktop_entry_id,
            false,
        )),
        Task::done(Message::ToogleFocusDesktopEntry(
            state.focus_desktop_entry_id,
            true,
        )),
        //TODO: make this snap_to instead of just scrolling by
        scroll_by(
            IcedId::new(LAUNCHER_SCROLLABLE_ID),
            AbsoluteOffset {
                x: 0.,
                y: offset as f32 * ENTRY_ELEMENTS_HEIGHT,
            },
        ),
    ]);
}
