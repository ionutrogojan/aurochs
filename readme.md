# Aurochs

Pronounced O-rocks

### History

Is an extinct cattle species, considered to be the wild ancestor of modern domestic cattle. With a shoulder height of up to 180 cm (71 in) in bulls and 155 cm (61 in) in cows, it was one of the largest herbivores in the Holocene; it had massive elongated and broad horns that reached 80 cm (31 in) in length. [Wiki](https://en.wikipedia.org/wiki/Aurochs)

### Some use cases may include:
1. A server side application where you use Rust to generate the HTML and you send the generated string as the request result
2. A static site generator where you create a blog template which takes a set of arguments and returns a html representation
3. A frontend generator for applications making use of html, such as [Tauri](https://tauri.app/) and [Electron](https://www.electronjs.org/)

Brings some of the JavaScript functionality of creating HTML elements to Rust

### Usage

```rs
use aurochs::{ Document, Element };

fn main() {
    let mut html = Document::create_element(Element::HTML);
    html.set_attribute("lang", "en");

    let mut title = Document::create_element(Element::TITLE);
    title.inner_text("Aurochs");

    let mut head = Document::create_element(Element::HEAD);
    head.append_child(title);

    let mut paragraph = Document::create_element(Element::P);
    paragraph.inner_text("Hello World!");

    let mut body = Document::create_element(Element::BODY);
    body.append_child(paragraph);

    html.append_child_list(vec![ head, body ]);

    println!("{}", html.render());

    /*
    <html lang="en">
        <head>
            <title>Aurochs</title>
        </head>
        <body>
            <p>Hello World!</p>
        </body>
    </html>
    */
}
```