use iced_widget::core::theme::Palette;
use iced_widget::core::{Color, color};

pub mod button;
pub mod utils;

#[derive(Default, Debug)]
pub enum Theme {
    #[default]
    Dark,
    Light,
}

impl Theme {
    pub const ALL: &'static [Self] = &[Self::Dark, Self::Light];

    pub fn palette(&self) -> Palette {
        match self {
            Self::Dark => Palette {
                background: todo!(),
                text: todo!(),
                primary: todo!(),
                success: todo!(),
                danger: todo!(),
            },
            Self::Light => Palette {
                background: todo!(),
                text: todo!(),
                primary: todo!(),
                success: todo!(),
                danger: todo!(),
            },
        };
    }
}
