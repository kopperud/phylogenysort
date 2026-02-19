use crate::tree::*;
use string_join::Join;

pub fn reorder(node: &mut Box<Node>) -> () {
    if !node.children.is_empty() {
        for child in node.children.iter_mut() {
            reorder(child);
        }

        let mut child_labels: Vec<String> = Vec::new();

        for child in node.children.iter_mut() {
            let label = child.label.clone();
            child_labels.push(label);
        }
        let mut this_node_label = "_".join(child_labels.clone());

        let is_sorted = child_labels.is_sorted();

        if !is_sorted {
            node.children.reverse();
            child_labels.reverse();

            this_node_label = "_".join(child_labels);
        }
        node.label = this_node_label;
    }

    //return this_node_label;
}
