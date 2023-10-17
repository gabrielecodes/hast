//! [![github]](https://github.com/gabrielecodes/hast)&ensp;
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! # Rustree
//!
//! This crate provides an easy way to build and extend a lossless syntax tree. The nodes are based
//! on [`syn`] types and are stored contiguously in memory. Parsing is performed speculatively,
//! that is, it is not necessary to explicitly program the types that the input code should be parsed
//! into, the syntax tree is built automatically. Moreover the tree makes parent-child relationships
//! available to the user to easily navigate and extend the tree.
//!
//! The main item of this crate is the [`speculative_parse`] function which should be used in a
//! macro (e.g. a [`proc_macro`] or a [`proc_macro_derive`]).
//!
//! ## Motivation
//!
//! This crate is motivated by the need of having a syntax tree with several properties:
//! 1. it should allow parsing an arbitrary sequence of rust constructs into a data strucutre made of
//! [`Node`]s that are related through parent-child relationships. We can ask for the parent or
//! the children of a given node and modify them, or insert nodes in the tree.
//! 2. The parsing should be performed without making reference to the types that are being
//! parsed, that is, the parsing should be as automatic as possible. There should be no need parse
//! the input code "into" as pecific struct, a [`TokenStream`] should be sufficient.
//! 3. The tree should be easily extendable, printable, modifiable and serializable.
//!
//! The [`syn`] crate offers many types (see e.g. [`syn::Item`] or [`syn::Expr`]) to parse code
//! into, but parsing within the `Parse` trait can be laborious. Moreover it is convenient to
//! have a tree structure with methods to extend the tree.
//!
//! On the other hand, [`TokenStream`]s are based on generic types (e.g. `Group`) that do not
//! correspond to an entire rust construct (such as a whole `struct` or `fn`) and are thus not
//! convenient to use when adding complex extensions to the tree.
//!
//! _Note: as of version 0.1.0 the list of parsable constructs is not exaustive_.
//!
//! [`proc_macro`]: https://doc.rust-lang.org/proc_macro/index.html
//! [`proc_macro_derive`]: https://doc.rust-lang.org/proc_macro/index.html
//! [`ParseStream`]: https://docs.rs/syn/latest/syn/parse/type.ParseStream.html
//! [`TokenStream`]: https://doc.rust-lang.org/proc_macro/struct.TokenStream.html
//! [`syn`]: https://docs.rs/syn/latest/syn/index.html
//! [`syn::Item`]: https://docs.rs/syn/latest/syn/enum.Item.html
//! [`syn::Expr`]: https://docs.rs/syn/latest/syn/enum.Expr.htmlca
//!
//!
//! ## Examples
//! ### Visualizing the tree
//! ```
//! # fn main() {
//! let tokens = quote! {
//!     fn cube(x: i32) -> i32 {
//!         fn square(y: i32) -> i32 {
//!             println!("taking the square");
//!             y * y
//!         }
//!         let result = x * square(x);
//!         print!("the cube of {x} is {result}");
//!         result
//!     }
//! };
//!
//! let ast = speculative_parse(tokens).unwrap();
//! ast.print_tree();
//! # }
//! ```
//! output:
//! ```text
//!level  id
//!  0     0  node: Root, parent: 0, children: [1]
//!  1     1 ── node: ItemFn: cube, parent: 0, children: [2, 4, 5, 6]
//!  2     2 ──── node: ItemFn: square, parent: 1, children: [3]
//!  3     3 ────── node: Stmt: Macro: "println", parent: 2, children: []
//!  2     4 ──── node: Stmt: Local: result, parent: 1, children: []
//!  2     5 ──── node: Stmt: Macro: "print", parent: 1, children: []
//!  2     6 ──── node: ExprPath: result, parent: 1, children: []
//! ```
//! where `parent` is the `id` of the parent and `children` are the `id`'s of the children
//!
//! ### Macro usage
//! Parse an arbitrary rust code into the syntax tree, apply some modifications, and return the
//! modified code.

pub mod ast;
mod construct;
mod node;
pub mod parse;
mod utils;

use ast::SyntaxTree;
use parse::speculative_parse_inner;
use syn::parse::Parser;

/// Parsing function to be used in a macro. It parses a valid rust code and returns a [`SyntaxTree`] without
/// the need to specify the types of the constructs that are being parsed. The parsing errors if the input
/// code is not a valid rust code.
///
/// # Examples
pub fn speculative_parse(tokens: impl Into<proc_macro2::TokenStream>) -> syn::Result<SyntaxTree> {
    speculative_parse_inner.parse2(tokens.into())
}
