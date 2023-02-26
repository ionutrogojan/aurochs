//! # Aurochs
//! 
//! Pronounced O-rocks
//! 
//! ### History
//! 
//! Is an extinct cattle species, considered to be the wild ancestor of modern domestic cattle. With a shoulder height of up to 180 cm (71 in) in bulls and 155 cm (61 in) in cows, it was one of the largest herbivores in the Holocene; it had massive elongated and broad horns that reached 80 cm (31 in) in length. [Wiki](https://en.wikipedia.org/wiki/Aurochs)
//! 
//! ### Some use cases may include:
//! 1. A server side application where you use Rust to generate the HTML and you send the generated string as the request result
//! 2. A static site generator where you create a blog template which takes a set of arguments and returns a html representation
//! 3. A frontend generator for applications making use of html, such as [Tauri](https://tauri.app/) and [Electron](https://www.electronjs.org/)
//! 
//! Brings some of the JavaScript functionality of creating HTML elements to Rust
use std::rc::Rc;
/// A list of all available HTML elements
/// 
/// # Example
/// ```
/// Element::HTML
/// ```
#[derive(Debug, Clone)]
pub enum Element {
    HTML, HEAD, LINK, META, STYLE, TITLE, BODY, HEADER, MAIN, FOOTER, // MAIN ELEMENTS
    ARTICLE, ASIDE, NAV, SECTION, DIV, UL, OL, LI, SPAN, BR, // SECTIONING ELEMENTS
    H1, H2, H3, H4, H5, H6, P, A, // TEXT ELEMENTS
    IMG, AUDIO, VIDEO, TRACK, SOURCE, SVG, CANVAS, // MEDIA ELEMENTS
    SCRIPT, 
    BUTTON, INPUT, DATALIST, SELECT, OPTION, FORM, LABEL, TEXTAREA, DETAILS, DIALOG, SUMMARY, // INPUT ELEMENTS
    TEMPLATE
}
#[derive(Debug, Clone)]
enum Closing { TAG, SELF, NONE }
#[derive(Debug, Clone)]
enum Children {
    Text(String),
    Node(Rc<Node>)
}
#[derive(Debug, Clone)]
/// A mutable struct composed of a tag, attribute list, child list and closing tag.
/// 
/// Do not attempt to modify the Node element directly. Use the available methods.
/// ```
/// Node {
///     tag: Element,
///     attributes: Option<Vec<String>>,
///     children: Option<Vec<Children>>,
///     closing_tag: Closing
/// }
/// ```
pub struct Node {
    tag: Element,
    attributes: Option<Vec<String>>,
    children: Option<Vec<Children>>,
    closing_tag: Closing
}

impl Node {
    /// Sets the value of the Node's attribute. This does not return an error if the wrong (key, value) was set.
    /// 
    /// # Example
    /// ```
    /// use aurochs::{ Document, Element };
    /// 
    /// let mut html = Document::create_element(Element::HTML);
    /// html.set_attribute("lang", "en");
    /// ```
    /// 
    /// # Javascript Equivalent 
    /// [MDN web docs](https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttribute)
    /// ```
    /// let html = document.createElement("html");
    /// html.setAttribute("lang", "en");
    /// ```
    /// 
    /// # HTML
    /// ```
    /// <html lang="en"></html>
    /// ```
    pub fn set_attribute(&mut self, name: &str, value: &str) {
        match &mut self.attributes {
            Some(attributes) => attributes.push(format!("{}=\"{}\"", name, value)),
            None => {
                let mut attributes: Vec<String> = Vec::new();
                attributes.push(format!("{}=\"{}\"", name, value));
                self.attributes = Some(attributes);
            },
        }
    }
    /// Sets the values of the Node's attributes. This does not return an error if the wrong (key, value) was set.
    /// 
    /// # Example
    /// 
    /// ```
    /// use aurochs::{ Document, Element };
    /// 
    /// let mut script = Document::create_element(Element::SCRIPT);
    /// script.set_attribute_list(vec![("src", "./main.js"), ("defer", ""), ("type", "module")]);
    /// ```
    /// # Javascript Equivalent 
    /// ```
    /// let script = document.createElement("script");
    /// const attributes = { "src": "./main.js", "defer": "", "type": "module" };
    /// for (let key in attributes ) {
    ///     script.setAttribute(key, attributes[key]);
    /// }
    /// ```
    /// # HTML
    /// ```
    /// <script src="./main.js" defer type="module"></script>
    /// ```
    pub fn set_attribute_list(&mut self, attributes: Vec<(&str, &str)>) {
        for ( name, value ) in attributes {
            self.set_attribute(name, value);
        }
    }
    /// Sets the rendered text content of a Node.
    /// 
    /// # Example
    /// ```
    /// use aurochs::{ Document, Element };
    /// 
    /// let mut paragraph = Document::create_element(Element::P);
    /// paragraph.inner_text("Hello World!");
    /// ```
    /// 
    /// # Javascript Equivalent 
    /// [MDN web docs](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/innerText)
    /// ```
    /// let paragraph = document.createElement("p");
    /// paragraph.inner_text("Hello World!");
    /// ```
    /// 
    /// # HTML
    /// ```
    /// <p>Hello World!</p>
    /// ```
    pub fn inner_text(&mut self, value: &str) {
        match &mut self.children {
            Some(child) => child.push(Children::Text(value.to_string())),
            None => {
                let mut children: Vec<Children> = Vec::new();
                children.push(Children::Text(value.to_string()));
                self.children = Some(children);
            }
        }
    }
    /// Adds a Node to the end of the list of children of a specified parent Node.
    /// 
    /// # Example
    /// ```
    /// use aurochs::{ Document, Element };
    /// 
    /// let mut body = Document::create_element(Element::BODY);
    /// let mut paragraph = Document::create_element(Element::P);
    /// body.append_child(paragraph);
    /// ```
    /// 
    /// # Javascript Equivalent 
    /// [MDN web docs](https://developer.mozilla.org/en-US/docs/Web/API/Node/appendChild)
    /// ```
    /// let body = document.createElement("body");
    /// let paragraph = document.createElement("p");
    /// body.appendChild(paragraph);
    /// ```
    /// 
    /// # HTML
    /// ```
    /// <body>
    ///     <p><p>
    /// </body>
    /// ```
    pub fn append_child(&mut self, node: Node) {
        match &mut self.children {
            Some(child) => child.push(Children::Node(Rc::new(node))),
            None => {
                let mut children: Vec<Children> = Vec::new();
                children.push(Children::Node(Rc::new(node)));
                self.children = Some(children);
            }
        }
    }
    /// Adds multiple Nodes to the end of the list of children of a specified parent Node.
    /// 
    /// # Example
    /// ```
    /// use aurochs::{ Document, Element };
    /// 
    /// let mut body = Document::create_element(Element::BODY);
    /// let mut h1 = Document::create_element(Element::H1);
    /// let mut h2 = Document::create_element(Element::H2);
    /// let mut h3 = Document::create_element(Element::H3);
    /// body.append_child_list(vec![ h1, h2, h3 ]);
    /// ```
    /// 
    /// # Javascript Equivalent 
    /// ```
    /// let body = document.createElement("body");
    /// let h1 = document.createElement("h1");
    /// let h2 = document.createElement("h2");
    /// let h3 = document.createElement("h3");
    /// 
    /// const elements = { h1, h2, h3 };
    ///
    /// for (let elem in elements) {
    ///     body.appendChild(elements[elem]);
    /// }
    /// ```
    /// 
    /// # HTML
    /// ```
    /// <body>
    ///     <h1><h1>
    ///     <h2><h2>
    ///     <h3><h3>
    /// </body>
    /// ```
    pub fn append_child_list(&mut self, children: Vec<Node>) {
        for node in children {
            self.append_child(node);
        }
    }
    /// Returns a duplicate of the Node on which this method was called. 
    /// Cloning a Node copies all of its attributes and their values, including intrinsic (inline) listeners.
    /// 
    /// Warning: `clone_node()` may lead to duplicate Node attributes in a document, such as IDs and CLASSes! 
    /// 
    /// # Example
    /// ```
    /// use aurochs::{ Document, Element };
    /// 
    /// let mut paragraph = Document::create_element(Element::P);
    /// let mut paragraph_clone = paragraph.clone_node();
    /// ```
    /// 
    /// # Javascript Equivalent 
    /// [MDN web docs](https://developer.mozilla.org/en-US/docs/Web/API/Node/cloneNode)
    /// ```
    /// let paragraph = document.createElement("p");
    /// let paragraph_clone = paragraph.cloneNode(true);
    /// ```
    /// 
    /// # HTML
    /// ```
    /// <p><p>
    /// <p><p>
    /// ```
    pub fn clone_node(&self) -> Node {
        let cloned_attributes = match &self.attributes {
            Some(attrs) => Some(attrs.clone()),
            None => None,
        };

        let cloned_children = match &self.children {
            Some(children) => {
                let mut cloned_children = Vec::new();
                for child in children {
                    match child {
                        Children::Text(text) => {
                            cloned_children.push(Children::Text(text.clone()));
                        },
                        Children::Node(node) => {
                            cloned_children.push(Children::Node(Rc::new(node.clone_node())));
                        },
                    }
                }
                Some(cloned_children)
            },
            None => None,
        };

        Node {
            tag: self.tag.clone(),
            attributes: cloned_attributes,
            children: cloned_children,
            closing_tag: self.closing_tag.clone(),
        }
    }
    /// Returns the parsed Node Tree as a string
    /// 
    /// # Example
    /// ```
    /// use aurochs::{ Document, Element };
    /// 
    /// let mut html = Document::create_element(Element::HTML);
    /// html.set_attribute("lang", "en");
    ///
    /// let mut head = Document::create_element(Element::HEAD);
    /// let mut body = Document::create_element(Element::BODY);
    /// 
    /// html.append_child_list(vec![ head, body]);
    /// 
    /// println!("{}", html.render());
    /// ```
    /// 
    /// # HTML
    /// ```
    /// <html lang="en">
    ///     <head></head>
    ///     <body></body>
    /// </html>
    /// ```
    pub fn render(&self) -> String {
        let tag = format!("{:?}", self.tag).to_lowercase();

        let attributes = match &self.attributes {
            Some(attrs) => attrs.join(" "),
            None => String::new(),
        };

        let children = match &self.children {
            Some(children) => {
                let mut children_html = String::new();
                for child in children {
                    match child {
                        Children::Text(text) => children_html.push_str(text),
                        Children::Node(node) => children_html.push_str(&node.render()),
                    }
                }
                children_html
            },
            None => String::new(),
        };

        match &self.closing_tag {
            Closing::TAG => format!("<{} {}>{}</{}>", tag, attributes, children, tag),
            Closing::SELF => format!("<{} {}/>", tag, attributes),
            Closing::NONE => format!("<{} {}>", tag, attributes),
        }
    }
}
/// The root Element of the HTML Tree
/// 
/// It only serves the purpose of creating new Nodes
pub struct Document;

impl Document {
    /// Returns a mutable instance of a Node with the specified tag
    /// 
    /// # Example
    /// 
    /// ```
    /// use aurochs::{ Document, Element };
    /// 
    /// let mut html = Document::create_element(Element::HTML);
    /// ```
    /// 
    /// # Javascript Equivalent 
    /// [MDN web docs](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElement)
    /// ```
    /// let html = document.createElement("html");
    /// ```
    /// 
    /// # HTML
    /// ```
    /// <html></html>
    /// ```
    pub fn create_element(element: Element) -> Node {
        let closing_tag = match element {
            Element::HTML => Closing::TAG,
            Element::HEAD => Closing::TAG,
            Element::LINK => Closing::NONE,
            Element::META => Closing::NONE,
            Element::STYLE => Closing::TAG,
            Element::TITLE => Closing::TAG,
            Element::BODY => Closing::TAG,
            Element::HEADER => Closing::TAG,
            Element::MAIN => Closing::TAG,
            Element::FOOTER => Closing::TAG,
            Element::ARTICLE => Closing::TAG,
            Element::ASIDE => Closing::TAG,
            Element::NAV => Closing::TAG,
            Element::SECTION => Closing::TAG,
            Element::DIV => Closing::TAG,
            Element::UL => Closing::TAG,
            Element::OL => Closing::TAG,
            Element::LI => Closing::TAG,
            Element::SPAN => Closing::TAG,
            Element::BR => Closing::SELF,
            Element::H1 => Closing::TAG,
            Element::H2 => Closing::TAG,
            Element::H3 => Closing::TAG,
            Element::H4 => Closing::TAG,
            Element::H5 => Closing::TAG,
            Element::H6 => Closing::TAG,
            Element::P => Closing::TAG,
            Element::A => Closing::TAG,
            Element::IMG => Closing::NONE,
            Element::AUDIO => Closing::TAG,
            Element::VIDEO => Closing::TAG,
            Element::TRACK => Closing::SELF,
            Element::SOURCE => Closing::SELF,
            Element::SVG => Closing::TAG,
            Element::CANVAS => Closing::TAG,
            Element::SCRIPT => Closing::TAG,
            Element::BUTTON => Closing::TAG,
            Element::INPUT => Closing::NONE,
            Element::DATALIST => Closing::TAG,
            Element::SELECT => Closing::TAG,
            Element::OPTION => Closing::TAG,
            Element::FORM => Closing::TAG,
            Element::LABEL => Closing::TAG,
            Element::TEXTAREA => Closing::TAG,
            Element::DETAILS => Closing::TAG,
            Element::DIALOG => Closing::TAG,
            Element::SUMMARY => Closing::TAG,
            Element::TEMPLATE => Closing::TAG,
        };
        Node { tag: element, attributes: None, children: None, closing_tag }
    }

    // pub fn create_default() -> Vec<Node> {
    // TODO: create a default template and append the content to it
    // return a vec![ HEAD, BODY ] so we can append elements to it
    // }
}