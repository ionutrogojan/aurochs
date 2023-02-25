use std::rc::Rc;

#[derive(Debug)]
pub enum Element {
    SECTION,
    IMG,
    P,
}
#[derive(Debug)]
pub enum NodeBody {
    Text(String),
    Node(Rc<Node>)
}

#[derive(Debug)]
pub struct Node {
    element: Element,
    attributes: Option<Vec<String>>,
    body: Option<NodeBody>
}

impl Node {
    pub fn set_attribute(&mut self, name: &str, value: &str) {
        match &mut self.attributes {
            Some(attributes) => attributes.push(format!("{}=\"{}\"", name, value)),
            None => {
                let mut attributes: Vec<String> = Vec::new();
                attributes.push(format!("{}=\"{}\"", name, value));
                self.attributes = Some(attributes)
            },
        }
    }

    pub fn inner_text(&mut self, value: &str) {
        self.body = Some(NodeBody::Text(value.to_string()))
    }

    pub fn append_child(&mut self, node: Node) {
        self.body = Some(NodeBody::Node(Rc::new(node)))
    }

    pub fn render(&self) -> String {
        let attributes = match &self.attributes {
            Some(attrs) => attrs.join(" "),
            None => String::new(),
        };

        match &self.body {
            Some(body) => match body {
                NodeBody::Text(text) => format!(
                    "<{} {}>{}</{}>",
                    format!("{:?}", self.element).to_lowercase(),
                    attributes,
                    text,
                    format!("{:?}", self.element).to_lowercase(),
                ),
                NodeBody::Node(node) => format!(
                    "<{} {}>\n\x20\x20\x20\x20{}\n</{}>",
                    format!("{:?}", self.element).to_lowercase(),
                    attributes,
                    node.render(),
                    format!("{:?}", self.element).to_lowercase(),
                ),
            },
            None => format!(
                "<{} {}></{}>",
                format!("{:?}", self.element).to_lowercase(),
                attributes,
                format!("{:?}", self.element).to_lowercase(),
            ),
        }
    }
}

pub struct Document { }

impl Document {
    pub fn create_element(element: Element) -> Node {
        Node { element, attributes: None, body: None }
    }
}