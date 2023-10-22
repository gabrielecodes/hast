//! Syntax Tree data structure.
//!
//! The tree is based on a [`Vec`] containing [`Node`]s indexed by a monotonically
//! increasing integer index. A new tree has always a `Root` node. A [`Node`] is a
//! represntation of a rust construct obtained by parsing the input code.
//!

use super::construct::{make_construct_form_tokens, Construct};
use super::node::Node;
use proc_macro2::Span;
use quote::quote;
use std::ops::Deref;
use syn::{Ident, ImplItemFn, ItemFn, ItemImpl, ItemStruct, ItemTrait, Macro, Stmt, TraitItemFn};

#[cfg_attr(feature = "serialize", derive(Deserialize, Serialize))]
/// The Syntax Tree struct
pub struct SyntaxTree {
    nodes: Vec<Node>,
    last_id: usize,
    current_node_id: usize,
    current_level: usize,
}

impl SyntaxTree {
    /// Returns an empty SyntaxTree
    pub(crate) fn new() -> Self {
        let root = Construct::Root;
        let ident = Ident::new("root", Span::call_site());
        let node = Node::new(root, 0, ident);
        Self {
            nodes: vec![node],
            last_id: 0,
            current_node_id: 0,
            current_level: 1,
        }
    }

    // Not public api. Sets the level from the parse_inner macro.
    pub(crate) fn set_current_level(&mut self, level: usize) {
        self.current_level = level;
    }

    /// Returns a mutable reference to the current node.
    fn get_current_node_mut(&mut self) -> Option<&mut Node> {
        self.nodes.get_mut(self.current_node_id)
    }

    // increases the id of the nodes in the given slice by 1
    // needed to maintain the ordering of the indexes
    fn update_ids(&mut self, id: usize) {
        self.nodes[id..].iter_mut().for_each(|node| node.id += 1);
    }

    /// Adds a node to the SyntaxTree given its data
    /// and returns the index of the node
    fn add_node(&mut self, data: Construct, ident: Ident) -> usize {
        let id = self.nodes.len();
        let mut node = Node::new(data, id, ident.to_owned());
        if let Some(parent) = self.get_current_node_mut() {
            parent.children.push(id);
            node.parent = self.current_node_id;
            node.level = self.current_level;
        } else {
            node.parent = 0;
            node.level = 0;
        };

        self.nodes.push(node);
        self.last_id = id;
        id
    }

    /// Returns the node with the given name (identifier) or `None`
    /// if the node is not in the tree
    pub fn find_node(&self, ident: &Ident) -> Option<&Node> {
        self.nodes.iter().find(|node| node.ident.eq(ident))
    }

    /// Returns the node with the given name (identifier) or `None`
    /// if the node is not in the tree
    pub fn find_node_mut(&mut self, ident: &Ident) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|node| node.ident.eq(ident))
    }

    /// Adds a node to the tree before the node with the
    /// specified `id`. The added node has the same level of
    /// the node with the specified id and its `data` field is
    /// equal to the input data
    ///     
    /// Arguments:
    ///
    /// * `ident` - name (identifier) of the node after which the new node is added
    /// * `data`  - data of the new node that is being adde
    pub fn insert_node_before(&mut self, ident: &Ident, data: Construct) -> Option<usize> {
        let Some(sibling) = self.find_node(&ident) else {
            return None;
        };
        let node_id = sibling.id - 1;
        let mut node = Node::new(data, node_id, ident.to_owned());

        node.level = sibling.level;
        node.parent = sibling.parent;

        self.nodes.insert(node_id, node);
        self.update_ids(node_id + 1);

        Some(node_id)
    }

    /// Adds a node to the tree after the node with the
    /// specified `id`. The added node has the same level of
    /// the node with the specified id and its `data` field is
    /// equal to the input data
    ///
    /// Arguments:
    ///
    /// * `ident` - name (identifier) of the node after which the new node is added
    /// * `data`  - data of the new node that is being adde
    pub fn insert_node_after(&mut self, ident: &Ident, data: Construct) -> Option<usize> {
        let Some(sibling) = self.find_node(&ident) else {
            return None;
        };
        let node_id = sibling.id + 1;
        let mut node = Node::new(data, node_id, ident.to_owned());

        node.level = sibling.level;
        node.parent = sibling.parent;

        self.nodes.insert(node_id, node);
        self.update_ids(node_id + 1);

        Some(node_id)
    }

    /// Adds a node at the end of the tree and returns the id
    /// of the node
    pub fn append_node(&mut self, data: Construct, name: &str) -> Option<usize> {
        let id = self.last_id + 1;
        let parent_node = self.get_node_mut(self.last_id)?;
        let ident = Ident::new(name, Span::call_site());
        let mut node = Node::new(data, id, ident);

        parent_node.children.push(id);
        node.parent = parent_node.id;
        node.level = parent_node.level - 1;

        self.nodes.push(node);
        Some(id)
    }

    /// Adds a node at the end of the tree and returns its id
    /// given the TokenStream representation of the node
    pub fn append_tokenstream(
        &mut self,
        tokens: proc_macro2::TokenStream,
        name: &str,
    ) -> Option<usize> {
        let data = make_construct_form_tokens(tokens).unwrap();

        let id = self.last_id + 1;
        let parent_node = self.get_node_mut(self.last_id)?;
        let ident = Ident::new(name, Span::call_site());
        let mut node = Node::new(data, id, ident);

        parent_node.children.push(id);
        node.parent = parent_node.id;
        node.level = id;

        self.nodes.push(node);
        Some(id)
    }

    /// Returns a proc_macro2::TokenStream from the nodes of the tree
    pub fn get_tokenstream(&self) -> proc_macro2::TokenStream {
        let nodes = &self.nodes;
        quote! {#(#nodes)*}
    }

    /// Returns an SyntaxTree with the given capacity
    pub fn with_capacity(n: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(n),
            last_id: 0,
            current_node_id: 0,
            current_level: 0,
        }
    }

    /// Returns a reference to the node with the given id
    pub fn get_node(&self, id: usize) -> Option<&Node> {
        self.nodes.get(id)
    }

    /// Returns a mutable reference to the node with the given id
    pub fn get_node_mut(&mut self, id: usize) -> Option<&mut Node> {
        self.nodes.get_mut(id)
    }

    /// Returns the size of the SyntaxTree as the lenght of the vector
    /// of nodes
    pub fn length(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the capacity of the SyntaxTree as the capacity of the
    /// vector of nodes
    pub fn capacity(&self) -> usize {
        self.nodes.capacity()
    }

    /// Returns an iterator over the nodes of the SyntaxTree
    pub fn iter(&self) -> impl Iterator<Item = &Node> {
        self.nodes.iter()
    }

    /// Returns a mutable iterator over the nodes of the SyntaxTree
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Node> {
        self.nodes.iter_mut()
    }

    #[cfg(feature = "debug")]
    pub fn print_tree(&self) {
        println!("level  id");
        self.nodes.iter().for_each(|node| println!("{node:?}"));
        println!("")
    }
}

pub(crate) mod visitor {
    use crate::utils::{
        match_expr, match_impl_item_fn, match_item_impl, match_lit_expr, match_pat, match_path,
        match_trait_item_fn,
    };

    use super::*;
    use syn::visit::{self, Visit};

    impl<'ast> Visit<'ast> for SyntaxTree {
        fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
            self.add_node(i.into(), i.ident.to_owned());
        }

        fn visit_block(&mut self, node: &'ast syn::Block) {
            self.current_level += 1;
            let return_id = self.current_node_id;
            visit::visit_block(self, node);
            self.current_node_id = return_id;
            self.current_level -= 1;
        }

        fn visit_item_fn(&mut self, i: &'ast ItemFn) {
            let return_id = self.current_node_id;
            let id = self.add_node(i.into(), i.sig.ident.to_owned());
            self.current_node_id = id;
            self.visit_block(&i.block);
            self.current_node_id = return_id;
        }

        fn visit_stmt(&mut self, i: &'ast Stmt) {
            match i {
                syn::Stmt::Local(local) => {
                    let expr = local.init.as_ref();
                    if let Some(exp) = expr {
                        match exp.expr.deref() {
                            syn::Expr::Block(block) => {
                                let return_id = self.current_node_id;

                                let ident = Ident::new("Block", Span::call_site());
                                let id = self.add_node(i.into(), ident);
                                self.current_node_id = id;
                                self.visit_expr_block(block);
                                self.current_node_id = return_id;
                            }
                            _ => {
                                let ident = Ident::new("Local", Span::call_site());
                                self.add_node(i.into(), ident); // TODO
                            }
                        }
                    }
                }
                syn::Stmt::Item(item) => match item {
                    syn::Item::Fn(func) => {
                        self.visit_item_fn(func);
                    }
                    syn::Item::Impl(item_impl) => self.visit_item_impl(item_impl),
                    &_ => {
                        let ident = Ident::new("ItemImpl", Span::call_site());
                        self.add_node(i.into(), ident);
                    }
                },
                syn::Stmt::Expr(expr, _) => match expr {
                    syn::Expr::Call(call) => {
                        let ident = match_expr(&call.func);
                        self.add_node(i.into(), ident);
                    }
                    syn::Expr::Path(path) => {
                        let ident = match_path(&path.path);
                        let ident = ident.first().unwrap();
                        self.add_node(path.into(), ident.to_owned());
                    }
                    syn::Expr::Let(let_expr) => {
                        let ident = match_pat(&let_expr.pat);
                        let ident = ident.first().unwrap();
                        self.add_node(let_expr.into(), ident.to_owned());
                    }
                    syn::Expr::Lit(lit_expr) => {
                        let ident = match_lit_expr(&lit_expr.lit);
                        self.add_node(lit_expr.into(), ident);
                    }
                    syn::Expr::Assign(assign) => {
                        let ident = match_expr(&assign.left);
                        self.add_node(assign.into(), ident);
                    }
                    syn::Expr::Macro(mac) => {
                        let ident = if let Some(ident) = match_path(&mac.mac.path).first() {
                            ident.to_owned()
                        } else {
                            Ident::new("macro", Span::call_site())
                        };

                        self.add_node(mac.into(), ident);
                    }
                    syn::Expr::Return(ret) => {
                        let ident = if let Some(expr) = ret.expr.to_owned() {
                            match_expr(&expr)
                        } else {
                            Ident::new("return", Span::call_site())
                        };

                        self.add_node(ret.into(), ident);
                    }
                    &_ => (), // TODO
                },
                syn::Stmt::Macro(mac) => {
                    let ident = if let Some(ident) = match_path(&mac.mac.path).first() {
                        self.add_node(mac.into(), ident.to_owned())
                    } else {
                        let ident = Ident::new("macro", Span::call_site());
                        self.add_node(mac.into(), ident)
                    };
                }
            };
        }

        fn visit_item_impl(&mut self, i: &'ast ItemImpl) {
            let id = if let Some(ident) = match_item_impl(i).first() {
                self.add_node(i.into(), ident.to_owned())
            } else {
                let ident = &Ident::new("ItemImpl", Span::call_site());
                self.add_node(i.into(), ident.to_owned())
            };

            self.current_node_id = id;
            visit::visit_item_impl(self, i);
            self.current_node_id = 0;
        }

        fn visit_impl_item_fn(&mut self, i: &'ast ImplItemFn) {
            self.current_level += 1;
            let return_id = self.current_node_id;

            let ident = match_impl_item_fn(i);
            let id = self.add_node(i.into(), ident);

            self.current_node_id = id;
            visit::visit_impl_item_fn(self, i);
            self.current_node_id = return_id;

            self.current_level -= 1;
        }

        fn visit_item_trait(&mut self, i: &'ast ItemTrait) {
            let return_id = self.current_node_id;
            let id = self.add_node(i.into(), i.ident.to_owned());

            self.current_node_id = id;
            visit::visit_item_trait(self, i);
            self.current_node_id = return_id;
        }

        fn visit_trait_item_fn(&mut self, i: &'ast TraitItemFn) {
            self.current_level += 1;
            let return_id = self.current_node_id;

            let ident = match_trait_item_fn(i);
            let id = self.add_node(i.into(), ident);

            self.current_node_id = id;
            visit::visit_trait_item_fn(self, i);
            self.current_node_id = return_id;

            self.current_level -= 1;
        }

        // top level macro
        fn visit_macro(&mut self, i: &'ast Macro) {
            let ident = match_path(&i.path);
            let ident = ident.first().unwrap();
            self.add_node(i.into(), ident.to_owned());
        }
    }
}
