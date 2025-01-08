use std::io::Result as IoResult;

use ratatui::backend::WindowSize;
use ratatui::buffer::Cell;
use ratatui::layout::Position;
use ratatui::layout::Size;
use ratatui::prelude::Backend;
use web_sys::wasm_bindgen::prelude::Closure;
use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::JsValue;
use web_sys::window;
use web_sys::Document;
use web_sys::Element;

use crate::utils::create_span;
use crate::utils::get_cell_color;
use crate::widgets::HYPERLINK;

#[derive(Debug)]
pub struct WasmBackend {
    buffer: Vec<Vec<Cell>>,
    prev_buffer: Vec<Vec<Cell>>,
    grid: Element,
    document: Document,
    cells: Vec<Element>,
    initialized: bool,
}

impl WasmBackend {
    pub fn new() -> Self {
        // use this time to initialize the grid and the document object for the backend to use later on
        let window = window().unwrap();
        let document = window.document().unwrap();
        let div = document.create_element("div").unwrap();
        div.set_attribute("id", "grid").unwrap();
        let body = document.body().unwrap();
        body.append_child(&div).unwrap();

        Self {
            buffer: get_sized_buffer(),
            prev_buffer: get_sized_buffer(),
            grid: div,
            document,
            cells: vec![],
            initialized: false,
        }
    }

    // here's the deal, we compare the current buffer to the previous buffer and update only the cells that have changed since the last render call
    fn update_grid(&mut self) {
        for (y, line) in self.buffer.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                if cell.modifier.contains(HYPERLINK) {
                    continue;
                }
                if cell != &self.prev_buffer[y][x] {
                    // web_sys::console::log_1(&format!("Cell different at ({}, {})", x, y).into());
                    let elem = self.cells[y * self.buffer[0].len() + x].clone();
                    // web_sys::console::log_1(&"Element retrieved".into());
                    elem.set_inner_html(&cell.symbol());
                    elem.set_attribute("style", &get_cell_color(cell)).unwrap();
                    // web_sys::console::log_1(&"Inner HTML set".into());
                }
            }
        }
    }

    /// The rendering process is split into three steps.
    fn prerender(&mut self) {
        web_sys::console::log_1(&"hello from prerender".into());

        for line in self.buffer.iter() {
            let mut line_cells: Vec<Element> = Vec::new();
            let mut hyperlink: Vec<Cell> = Vec::new();
            let mut anchor_element: Option<Element> = None;
            for (i, cell) in line.iter().enumerate() {
                if cell.modifier.contains(HYPERLINK) {
                    // Start a new hyperlink
                    if hyperlink.is_empty() {
                        anchor_element = Some(self.document.create_element("a").unwrap());
                    }
                    hyperlink.push(cell.clone());
                    // If the next cell is not part of the hyperlink, close it
                    if !line
                        .get(i + 1)
                        .map(|c| c.modifier.contains(HYPERLINK))
                        .unwrap_or(false)
                    {
                        // Close the hyperlink by appending the accumulated cells to <a> and clearing the state
                        if let Some(anchor) = anchor_element.take() {
                            anchor
                                .set_attribute(
                                    "href",
                                    &hyperlink.iter().map(|c| c.symbol()).collect::<String>(),
                                )
                                .unwrap();
                            anchor
                                .set_attribute("style", &get_cell_color(&cell))
                                .unwrap();
                            for link_cell in &hyperlink {
                                let elem = create_span(link_cell);
                                self.cells.push(elem.clone());
                                anchor.append_child(&elem).unwrap();
                            }
                            line_cells.push(anchor.clone());
                            hyperlink.clear();
                        }
                    }
                } else {
                    let elem = create_span(cell);
                    self.cells.push(elem.clone());
                    line_cells.push(elem);
                }
            }

            // Create a <pre> element for the line
            let pre = self.document.create_element("pre").unwrap();
            pre.set_attribute("style", "margin: 0px;").unwrap();

            // Append all elements (spans and anchors) to the <pre>
            for elem in line_cells {
                pre.append_child(&elem).unwrap();
            }

            // Append the <pre> to the grid
            self.grid.append_child(&pre).unwrap();
        }
    }

    pub fn on_key_event<F>(&self, mut callback: F)
    where
        F: FnMut(&str) + 'static,
    {
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
            web_sys::console::log_1(&event);
            callback(&event.key());
        });
        self.document
            .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
}

impl Backend for WasmBackend {
    fn draw<'a, I>(&mut self, content: I) -> IoResult<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        // web_sys::console::log_1(&"hello from draw".into());
        for (x, y, cell) in content {
            let y = y as usize;
            let x = x as usize;
            let line = &mut self.buffer[y];
            line.extend(std::iter::repeat_with(Cell::default).take(x.saturating_sub(line.len())));
            line[x] = cell.clone();
        }
        // web_sys::console::log_1(&"hello from draw end ".into());
        Ok(())
    }

    fn hide_cursor(&mut self) -> IoResult<()> {
        Ok(())
    }

    fn show_cursor(&mut self) -> IoResult<()> {
        Ok(())
    }

    fn get_cursor(&mut self) -> IoResult<(u16, u16)> {
        Ok((0, 0))
    }

    fn set_cursor(&mut self, _x: u16, _y: u16) -> IoResult<()> {
        Ok(())
    }

    fn clear(&mut self) -> IoResult<()> {
        self.buffer = get_sized_buffer();
        Ok(())
    }

    fn size(&self) -> IoResult<Size> {
        Ok(Size::new(
            self.buffer.first().unwrap().len().saturating_sub(1) as u16,
            self.buffer.len().saturating_sub(1) as u16,
        ))
    }

    fn window_size(&mut self) -> IoResult<WindowSize> {
        todo!()
    }

    fn flush(&mut self) -> IoResult<()> {
        if !self.initialized {
            // web_sys::console::log_1(&"hello from flush".into());
            self.prerender();
            self.prev_buffer = self.buffer.clone(); // set the previous buffer to the current buffer for the first render
            self.initialized = true;
        }
        // web_sys::console::log_1(&"flush1".into());
        // check if the buffer has changed since the last render and update the grid
        if self.buffer != self.prev_buffer {
            self.update_grid();
        }
        self.prev_buffer = self.buffer.clone();
        Ok(())
    }

    fn get_cursor_position(&mut self) -> IoResult<Position> {
        todo!()
    }

    fn set_cursor_position<P: Into<Position>>(&mut self, _: P) -> IoResult<()> {
        todo!()
    }
}

/// Calculates the number of characters that can fit in the window.
fn get_window_size() -> (u16, u16) {
    let (w, h) = get_raw_window_size();
    // These are mildly magical numbers... make them more precise
    (w / 10, h / 20)
}

fn get_raw_window_size() -> (u16, u16) {
    fn js_val_to_int<I: TryFrom<usize>>(val: JsValue) -> Option<I> {
        val.as_f64().and_then(|i| I::try_from(i as usize).ok())
    }

    web_sys::window()
        .and_then(|s| {
            s.inner_width()
                .ok()
                .and_then(js_val_to_int::<u16>)
                .zip(s.inner_height().ok().and_then(js_val_to_int::<u16>))
        })
        .unwrap_or((120, 120))
}

// TODO: Improve this...
fn is_mobile() -> bool {
    get_raw_screen_size().0 < 550
}

/// Calculates the number of pixels that can fit in the window.
fn get_raw_screen_size() -> (i32, i32) {
    let s = web_sys::window().unwrap().screen().unwrap();
    (s.width().unwrap(), s.height().unwrap())
}

/// Calculates the number of characters that can fit in the window.
fn get_screen_size() -> (u16, u16) {
    let (w, h) = get_raw_screen_size();
    // These are mildly magical numbers... make them more precise
    (w as u16 / 10, h as u16 / 19)
}

fn get_sized_buffer() -> Vec<Vec<Cell>> {
    let (width, height) = if is_mobile() {
        get_screen_size()
    } else {
        get_window_size()
    };
    vec![vec![Cell::default(); width as usize]; height as usize]
}

fn show_diff(a: &[Vec<Cell>], b: &[Vec<Cell>]) {
    let mut diff = String::new();
    for (y, line) in a.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if cell != &b[y][x] {
                diff.push_str(&format!("{{{y}}},{{{x}}}: {cell:?} != {:?}\n", b[y][x]));
            }
        }
    }
    web_sys::console::log_1(&diff.into());
}
