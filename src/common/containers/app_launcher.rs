use std::process::exit;

use iced::Length::Fill;
use iced::widget::container::Style;
use iced::widget::image;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::text_input::Style as TextInputStyle;
use iced::widget::text_input::default as text_input_default;
use iced::widget::{Stack, Text, scrollable};
use iced::widget::{column, container, text, text_input};
use iced::{Background, Border, Color, Element, Length};

use log::error;

use crate::common::LauncherState;
use crate::common::Message;
use crate::config::app::ContainerType;
use crate::data::{LAUNCHER_CONTAINER_ID, LAUNCHER_SCROLLABLE_ID, LAUNCHER_TEXT_INPUT_ID};

pub fn view<Theme, Renderer>(state: &LauncherState) -> Element<'_, Message> {
    container(Stack::from_vec(vec![
        // TODO: implement a stack for having image/color as base-layer and the rest above
        container(column![
            //TODO: Separate launcher  widgets in different functions
            text_input("", &state.user_input)
                .on_input(Message::UserInputChanged)
                .id(LAUNCHER_TEXT_INPUT_ID)
                .line_height(state.config.input_bar.size.height)
                .width(state.config.input_bar.size.width)
                .font(state.config.input_bar.font)
                .style(|theme, status| TextInputStyle {
                    background: (|| match state.config.input_bar.background {
                        Some(bg) => bg,
                        None => Background::Color(Color::default()),
                    })(),
                    border: (|| match state.config.input_bar.border {
                        Some(bor) => bor,
                        None => Border {
                            width: 0.,
                            ..Default::default()
                        },
                    })(),
                    value: state.config.input_bar.text_color,
                    ..text_input_default(theme, status)
                }),
            scrollable(
                column(
                    state
                        .ui_desktop_entries
                        .as_ref()
                        .unwrap_or(&mut Box::new(Vec::new()))
                        .iter()
                        .filter_map(|entry| {
                            let desktop_entry_text: Text = text(entry.name.clone())
                                .font(state.config.entry.font)
                                .height(Length::Fixed(state.config.entry.size.height))
                                .into();

                            if entry.is_focus {
                                return Some(
                                    desktop_entry_text
                                        .color(match state.config.entry.specific {
                                            ContainerType::Entry {
                                                focus_text_color, ..
                                            } => focus_text_color,
                                            _ => {
                                                error!("Error while doing specific container type");
                                                exit(1);
                                            }
                                        })
                                        .width(Length::Fill)
                                        .font(state.config.entry.font)
                                        .into(),
                                );
                            }

                            Some(
                                desktop_entry_text
                                    .color(state.config.entry.text_color)
                                    .width(Length::Fill)
                                    .into(),
                            )
                        }),
                )
                .spacing(match state.config.main_window.specific {
                    ContainerType::MainWindow { spacing, .. } => spacing,
                    _ => {
                        error!("Error while doing specific container type");
                        exit(1);
                    }
                })
            )
            .direction(Direction::Vertical(Scrollbar::hidden()))
            .id(LAUNCHER_SCROLLABLE_ID)
            .width(state.config.entry.size.width)
        ])
        .id(LAUNCHER_CONTAINER_ID)
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
        .padding(match state.config.main_window.specific {
            ContainerType::MainWindow { padding, .. } => padding,
            _ => {
                error!("Error while doing specific container type");
                exit(1);
            }
        })
        .into(),
        (|| match &state.bg_image_path {
            Some(path) => image(shellexpand::tilde(path).to_string()).expand(true),
            None => image(""),
        })()
        .into(),
    ]))
    .width(Fill)
    .height(Fill)
    .into()
}
