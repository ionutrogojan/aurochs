//! # Aurochs
//! 
//! Pronounced O-rocks
//! 
//! ### Usage:
//! Aurochs is a html generator. Using a similar syntax to JavaScript, easily create templates, generate html and create amazing applications.
//! 
//! Brings some of the JavaScript functionality of creating HTML elements to Rust
//! 
//! ### Some use cases may include:
//! 1. A server side application where you use Rust to generate the HTML and you send the generated string as the request result
//! 2. A static site generator where you create a blog template which takes a set of arguments and returns a html representation
//! 3. A frontend generator for applications making use of html, such as [Tauri](https://tauri.app/) and [Electron](https://www.electronjs.org/)
enum Closing { TAG, NONE }
impl Closing {
    fn clone(&self) -> Closing {
        match *self {
            Closing::NONE => Closing::NONE,
            Closing::TAG => Closing::TAG
        }
    }
}
enum Children {
    Text(String),
    Node(Node)
}
/// A mutable struct composed of a tag, attribute list, child list and closing type.
/// 
/// Do not attempt to modify the Node element directly. Use the available methods.
/// ```
/// Node {
///     tag: String,
///     attributes: Option<Vec<String>>,
///     children: Option<Vec<Children>>,
///     closing_type: Closing
/// }
/// ```
pub struct Node {
    tag: String,
    attributes: Option<Vec<String>>,
    children: Option<Vec<Children>>,
    closing_type: Closing
}

// FIXME:: You should be using traits rather than methods for a bunch of things - Clone, ToString, maybe even Extend
impl Node {
    /// Sets the value of the Node's attribute. This does not return an error if the wrong (key, value) was set.
    /// 
    /// # Example
    /// ```
    /// use aurochs::Document;
    /// 
    /// let mut html = Document::create_element("html");
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
    /// use aurochs::Document;
    /// 
    /// let mut script = Document::create_element("script");
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
    /// use aurochs::Document;
    /// 
    /// let mut paragraph = Document::create_element("p");
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
    /// use aurochs::Document;
    /// 
    /// let mut body = Document::create_element("body");
    /// let mut paragraph = Document::create_element("p");
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
    pub fn append_child(&mut self, node: Node) { // TODO: return a result of Error if trying to append child to Closing::NONE element
        match &mut self.children {
            Some(child) => child.push(Children::Node(node)),
            None => {
                let mut children: Vec<Children> = Vec::new();
                children.push(Children::Node(node));
                self.children = Some(children);
            }
        }
    }
    /// Adds multiple Nodes to the end of the list of children of a specified parent Node.
    /// 
    /// # Example
    /// ```
    /// use aurochs::Document;
    /// 
    /// let mut body = Document::create_element("body");
    /// let mut h1 = Document::create_element("h1");
    /// let mut h2 = Document::create_element("h2");
    /// let mut h3 = Document::create_element("h3");
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
    /// use aurochs::Document;
    /// 
    /// let mut paragraph = Document::create_element("p");
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
    // pub fn clone_node(&self) -> Node {
    //     let cloned_attributes = match &self.attributes {
    //         Some(attrs) => Some(attrs.clone()),
    //         None => None,
    //     };

    //     let cloned_children = match &self.children {
    //         Some(children) => {
    //             let mut cloned_children = Vec::new();
    //             for child in children {
    //                 match child {
    //                     Children::Text(text) => {
    //                         cloned_children.push(Children::Text(text.clone()));
    //                     },
    //                     Children::Node(node) => {
    //                         cloned_children.push(Children::Node(node.clone_node()));
    //                     },
    //                 }
    //             }
    //             Some(cloned_children)
    //         },
    //         None => None,
    //     };

    pub fn clone_node(&self) -> Node {
        let cloned_attributes = match &self.attributes {
            Some(attrs) => Some(attrs.iter().cloned().collect()),
            None => None,
        };

        let cloned_children = match &self.children {
            Some(children) => {
                let mut cloned_children = Vec::new();
                for child in children {
                    cloned_children.push(match child {
                        Children::Text(text) => Children::Text(text.clone()),
                        Children::Node(node) => Children::Node(node.clone_node()),
                    });
                }
                Some(cloned_children)
            }
            None => None,
        };

        Node {
            tag: self.tag.clone(),
            attributes: cloned_attributes,
            children: cloned_children,
            closing_type: self.closing_type.clone(),
        }
    }
    //     Node {
    //         tag: self.tag,
    //         attributes: cloned_attributes,
    //         children: cloned_children,
    //         closing_type: self.closing_type,
    //     }
    // }
    /// Returns the parsed Node Tree as a string
    /// 
    /// # Example
    /// ```
    /// use aurochs::Document;
    /// 
    /// let mut html = Document::create_element("html");
    /// html.set_attribute("lang", "en");
    ///
    /// let mut head = Document::create_element("head");
    /// let mut body = Document::create_element("body");
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
        let attributes = match &self.attributes {
            Some(attrs) => format!(" {}", attrs.join(" ")),
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

        match &self.closing_type {
            Closing::TAG => format!("<{}{}>{}</{}>", self.tag, attributes, children, self.tag),
            Closing::NONE => format!("<{}{}>", self.tag, attributes),
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
    /// use aurochs::Document;
    /// 
    /// let mut html = Document::create_element("html");
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
    pub fn create_element(element_tag: &str) -> Node {
        let closing_type:Closing = match element_tag {
            "area" | "base" | "br" | "command" | "col" | "embed" | "hr" | "img" | "input" | "link" | "meta" | "param" | "source" => Closing::NONE,
            _ => Closing::TAG
        };
        Node { tag: element_tag.to_string(), attributes: None, children: None, closing_type }
    }
    // pub fn create_default() -> Vec<Node> {
    // TODO: create a default template and append the content to it
    // return a vec![ HEAD, BODY ] so we can append elements to it
    // }

    // TODO: create custom element -> <x-custom></x-custom>
}