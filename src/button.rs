use iced_widget::button::{Catalog, Status, Style, StyleFn};
use iced_widget::core::{Background, Border, Color};

use iced_widget::text_input::Catalog;

use crate::Theme;
use crate::utils::default;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default_class)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// The default version of the button, it's the `filled` version.
fn default_class(theme: &Theme, status: Status) -> Style {
    primary(theme, status)
}

fn button(foreground: Color, background: Color, hover: Color, status: Status) -> Style {
    match status {
        Status::Active | Status::Pressed => Style {
            background: Some(Background::Color(background)),
            text_color: foreground,
            border: Border {
                radius: 4.0.into(),
                ..default()
            },
            ..default()
        },
        _ => todo!(),
    }
}

pub fn primary(theme: &Theme, status: Status) -> Style {
    let foreground = theme.palette().text;
    let button_colors = theme.colors().buttons.primary;

    let background = button_colors.background;

    let background_hover = button_colors.background_hover;

    button(foreground, background, background_hover, status)
}
