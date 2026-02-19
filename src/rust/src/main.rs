use std::fs::File;
use std::io::{prelude::*, BufReader};

use crate::parser::*;
use crate::reorder::*;
use crate::tree::*;
use crate::writenewick::*;

pub mod linecount;
pub mod parser;
pub mod reorder;
pub mod tokenizer;
pub mod tree;
pub mod utils;
pub mod writenewick;

fn main() {
    let filename = "input.tre".to_string();

    let file = File::open(filename).expect("asd");
    let f = BufReader::new(&file);

    let mut trees: Vec<Node> = Vec::new();

    for line in f.lines() {
        let line_string: String = line.unwrap();

        let mut tree = parse_tree(line_string);

        //pub fn label_internal_nodes(node: &mut Node) -> String {
        //label_internal_nodes(&mut tree);

        // todo: sort the tree
        reorder(&mut tree);

        trees.push(*tree);
    }

    let path = "output.tre".to_string();
    let mut output = File::create(path).expect("asd2");

    for tree in trees {
        // todo: print a newick string
        let newick = tree.writenewick();

        writeln!(output, "{}", newick).expect("hello");
    }
}
