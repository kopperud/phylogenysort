#' @export
phylogeny_sort <- function(phy) {
    newick_string <- ape::write.tree(phy)

    newick_string_sorted <- tree_sort_alphabetical(newick_string)

    txt <- textConnection(newick_string_sorted)
    tree <- ape::read.tree(txt)

    return(tree)
}
