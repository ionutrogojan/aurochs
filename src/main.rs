use aurochs::Document;

fn main() {
    let mut html = Document::create_element("html");
    html.set_attribute("lang", "en");

    let mut title = Document::create_element("title");
    title.inner_text("Aurochs");

    let mut head = Document::create_element("head");
    head.append_child(title);

    let mut paragraph = Document::create_element("p");
    paragraph.inner_text("Hello World!");

    let mut _break = Document::create_element("br");
    _break.set_attribute("class", "breaking");

    let mut _break2 = _break.clone_node();
    _break2.set_attribute("id", "still_breaking");

    let mut body = Document::create_element("body");
    body.append_child_list(vec![ paragraph, _break, _break2 ]);

    html.append_child_list(vec![ head, body ]);

    println!("{}", html.render());

    /*
    <html lang="en">
        <head>
            <title>Aurochs</title>
        </head>
        <body>
            <p>Hello World!</p>
            <br class="breaking">
            <br class="breaking" id="still_breaking">
        </body>
    </html>
    */
}