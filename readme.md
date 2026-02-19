# phylogeny sort

This R-package can sort phylogenies, such that taxon labels in the newick string (read from left to right) are in alphabetical order, as far as the topology allows.

For example, the tree `((Camel, Baboon), Aardvark)` when sorted is `(Aardvark, (Baboon, Camel))`. Another tree, lets say `((C,B),(A,Z))` is sorted as `((A,Z),(B,C))`, because the label `AZ` comes before `BC` in alphabetical order.

## install

```R
library(remotes)
remotes::install_git("https://github.com/kopperud/phylogenysort.git")
```

## use

```R
library(ape)
library(phylogenysort)

tr <- read.tree("~/path/to/trees.tre")

plot(tr[[1]])
plot(tr[[2]])

tr_sorted <- list()
for (i in seq_along(tr)){
  tr_sorted[[i]] <- phylogeny_sort(tr[[i]])
}

plot(tr_sorted[[1]])
plot(tr_sorted[[2]])
```
