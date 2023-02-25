use aurochs::{ Document, Element };

fn main() {
    let mut text = Document::create_element(Element::P);
    text.set_attribute("class", "text-element");
    // text.inner_text("Hello World!");

    let mut child_text = Document::create_element(Element::P);
    child_text.set_attribute("class", "child-text-element");
    child_text.inner_text("Goodbye!");

    text.append_child(child_text);

    println!("{}", text.render());
}