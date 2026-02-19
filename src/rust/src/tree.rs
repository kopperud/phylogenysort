#[derive(Debug, Default)]
pub struct Node {
    pub label: String,
    pub length: f64,
    pub children: Vec<Box<Node>>,
}

impl Node {
    pub fn new(label: String, length: f64) -> Self {
        let children: Vec<Box<Node>> = Vec::new();
        Self {
            label,
            length,
            children,
        }
    }
}
