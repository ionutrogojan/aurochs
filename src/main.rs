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