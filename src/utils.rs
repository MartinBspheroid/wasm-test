use ratatui::{buffer::Cell, style::Color};
use web_sys::Element;

pub(crate) fn create_span(cell: &Cell, document_mode: &DocumentMode) -> Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let span = document.create_element("span").unwrap();
    span.set_inner_html(cell.symbol());

    let style = get_cell_color(cell, document_mode);
    span.set_attribute("style", &style).unwrap();
    span
}

pub(crate) fn get_cell_color(cell: &Cell, document_mode: &DocumentMode) -> String {
    let fg = ansi_to_rgb(cell.fg);
    let bg = ansi_to_rgb(cell.bg);

    let fg_style = match fg {
        Some(color) => format!("color: rgb({}, {}, {});", color.0, color.1, color.2),
        None => {
            if document_mode.dark {
                "color: rgb(255, 255, 255);".to_string()
            } else {
                "color: rgb(0, 0, 0);".to_string()
            }
        }
    };

    let bg_style = match bg {
        Some(color) => format!(
            "background-color: rgb({}, {}, {});",
            color.0, color.1, color.2
        ),
        None => "background-color: transparent;".to_string(),
    };

    format!("{} {}", fg_style, bg_style)
}

pub fn ansi_to_rgb(color: Color) -> Option<(u8, u8, u8)> {
    match color {
        Color::Black => Some((0, 0, 0)),
        Color::Red => Some((128, 0, 0)),
        Color::Green => Some((0, 128, 0)),
        Color::Yellow => Some((128, 128, 0)),
        Color::Blue => Some((0, 0, 128)),
        Color::Magenta => Some((128, 0, 128)),
        Color::Cyan => Some((0, 128, 128)),
        Color::Gray => Some((192, 192, 192)),
        Color::DarkGray => Some((128, 128, 128)),
        Color::LightRed => Some((255, 0, 0)),
        Color::LightGreen => Some((0, 255, 0)),
        Color::LightYellow => Some((255, 255, 0)),
        Color::LightBlue => Some((0, 0, 255)),
        Color::LightMagenta => Some((255, 0, 255)),
        Color::LightCyan => Some((0, 255, 255)),
        Color::White => Some((255, 255, 255)),
        _ => None, // Handle invalid color names
    }
}
pub fn set_document_title(title: &str) {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .set_title(title);
}

#[derive(Debug)]
pub struct DocumentMode {
    pub dark: bool,
    pub light: bool,
}

/// Returns the current document mode (e.g. "dark" or "light").
pub fn get_document_mode() -> DocumentMode {
    let mode = web_sys::window()
        .unwrap()
        .match_media("(prefers-color-scheme: dark)")
        .unwrap()
        .unwrap()
        .matches();

    DocumentMode {
        dark: mode,
        light: !mode,
    }
}
