## Summary

This project is a second iteration from my implementation of [Hashgraph in Rust
that compiles to Web Assembly](https://github.com/jaybutera/rust-hashgraph).
The original version used pretty abstractions but quickly became very slow at
scale. This is a complete redo with emphasis on performance over
readability/portability. Accomplished using data-driven principles, this code
achieves significant speed ups by utilizing both better algorithms and more performant
data structures. The code also formalizes the Hashgraph process into
well established graph search algorithms, like the Floyd-Warshal and Kruskal
methods. Following these abstractions, this code could probably be improved
another ten-fold just by replacing the hand-coded matrix bits with a proper matrix maths library.
