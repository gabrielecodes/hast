# Rustree

This crate provides an easy way to build and extend a lossless syntax tree. The nodes are based
on `syn` types and are stored contiguously in memory. Parsing is performed speculatively,
that is, it is not necessary to explicitly program the types that the input code should be parsed
into, the syntax tree is built automatically. Moreover the tree makes parent-child relationships
available to the user to easily navigate and extend the tree.
The main item of this crate is the [`speculative_parse`](https://github.com/gabrielecodes/hast/blob/master/src/lib.rs) function which should be used in a
macro (e.g. a `proc_macro` or a `proc_macro_derive`).

## Motivation

This crate is motivated by the need of having a syntax tree with several properties:

This crate is motivated by the need of having a parser with several properties:

1. Tree output. The parser should make available be a tree structure with parent-child
relationships between the nodes,
1. No need for special handling of input code. The parser should ingest an arbitrary sequence
of constructs and parse it automatically.
3. Easy traversal and extension. The tree should be easily extendable, printable, traversable
and serializable.

_Note: as of version 0.1.0 the list of parsable constructs is not exaustive_.

## Examples

### Visualizing the tree

```rust
# fn main() {
let tokens = quote! {
    fn cube(x: i32) -> i32 {
        fn square(y: i32) -> i32 {
            println!("taking the square");
            y * y
        }
        let result = x * square(x);
        print!("the cube of {x} is {result}");
        result
    }
};
let ast = speculative_parse(tokens).unwrap();
ast.print_tree();
# }
```

output:

```rust
level  id
  0     0  node: Root, parent: 0, children: [1]
  1     1 ── node: ItemFn: cube, parent: 0, children: [2, 4, 5, 6]
  2     2 ──── node: ItemFn: square, parent: 1, children: [3]
  3     3 ────── node: Stmt: Macro: "println", parent: 2, children: []
  2     4 ──── node: Stmt: Local: result, parent: 1, children: []
  2     5 ──── node: Stmt: Macro: "print", parent: 1, children: []
  2     6 ──── node: ExprPath: result, parent: 1, children: []
```

where `parent` is the `id` of the parent and `children` are the `id`'s of the children

### Macro usage

Parse an arbitrary rust code into the syntax tree, apply some modifications, and return the
modified code.
