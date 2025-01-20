# Ratzilla

Build terminal-themed web applications with Rust and WebAssembly. Powered by [Ratatui].

## Quickstart

Add **Ratzilla** as a dependency in your `Cargo.toml`:

```sh
cargo add ratzilla
```

Here is a minimal example:

```rust no_run
use std::cell::RefCell;
use std::io;
use std::rc::Rc;

use ratzilla::ratatui::{
    layout::Alignment,
    style::Color,
    widgets::{Block, Paragraph},
    Terminal,
};

use ratzilla::{DomBackend, RenderOnWeb};
use ratzilla::event::KeyCode;

fn main() -> io::Result<()> {
    let counter = Rc::new(RefCell::new(0));
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    terminal.on_key_event({
        let counter_cloned = counter.clone();
        move |key_event| {
            if key_event.code == KeyCode::Char(' ') {
                let mut counter = counter_cloned.borrow_mut();
                *counter += 1;
            }
        }
    });

    terminal.render_on_web(move |f| {
        let counter = counter.borrow();
        f.render_widget(
            Paragraph::new(format!("Count: {counter}"))
                .alignment(Alignment::Center)
                .block(
                    Block::bordered()
                        .title("Ratzilla")
                        .title_alignment(Alignment::Center)
                        .border_style(Color::Yellow),
                ),
            f.area(),
        );
    });

    Ok(())
}
```

Then add your `index.html` which imports the JavaScript module:

<details>
  <summary>index.html</summary>
  
```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Ratzilla</title>
    <style>
      body {
        margin: 0;
        width: 100%;
        height: 100vh;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        align-content: center;
        font-family: "Courier New", Courier, monospace;
        font-size: 16px;
        background-color: #333;
      }
    </style>
  </head>
  <body>
    <script type="module">
      import init from "./pkg/ratzilla.js";
      init();
    </script>
  </body>
</html>
```

</details>

Install [trunk] to build and serve the web application.

```sh
cargo install --locked trunk
```

Then serve it on your browser:

```sh
trunk serve
```

Now go to `http://localhost:8080` and enjoy TUIs in your browser!

## Backends

**Ratzilla** currently supports the following backends:

1. [`DomBackend`]: Works by converting the cells to HTML elements (e.g. `<span>`, `<a>`, etc.) and rendering them in the DOM.
2. [`CanvasBackend`]: Works by rendering the cells in a HTML canvas element.

Both of these backends have their own pros and cons.

The [`DomBackend`] is more flexible and easier to style, but it can be slower for large TUIs. The [`CanvasBackend`] is faster and more efficient, but does not support all the features of the [`DomBackend`] such as hyperlinks.

## Widgets

**Ratzilla** provides web-only widgets that you can use while building TUIs for web.

### [`Hyperlink`]

A widget that can be used to render hyperlinks.

```rust no_run
use ratzilla::widgets::Hyperlink;

let link = Hyperlink::new("https://ratatui.rs");

// Then you can render it as usual:
// frame.render_widget(link, frame.area());
```

## Acknowledgements

Thanks to [Webatui] projects for the inspiration and the initial implementation of the essential parts of DOM backend.

Special thanks to [Martin Blasko] for his huge help and contributions.

Lastly, thanks to [Ratatui] for providing the core TUI components.

[trunk]: https://trunkrs.dev
[Ratatui]: https://ratatui.rs
[`DomBackend`]: https://docs.rs/ratzilla/latest/ratzilla/struct.DomBackend.html
[`CanvasBackend`]: https://docs.rs/ratzilla/latest/ratzilla/struct.CanvasBackend.html
[`Hyperlink`]: https://docs.rs/ratzilla/latest/ratzilla/widgets/struct.Hyperlink.html
[Webatui]: https://github.com/TylerBloom/webatui
[Martin Blasko]: https://github.com/MartinBspheroid
