use extendr_api::prelude::*;

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

#[extendr]
fn tree_sort_alphabetical(x: String) -> String {
    //let mut trees: Vec<Node> = Vec::new();

    let mut tree = parse_tree(x);

    reorder(&mut tree);

    let s: String = tree.writenewick();

    return s;
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod phylogenysort;
    fn tree_sort_alphabetical;
}
