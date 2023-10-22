//! A Node representing a parsed rust [`Construct`]. The Node has no public constructor.
//! Adding a new node to the tree should be handled with the api provided by
//! `SyntaxTree`.

use super::construct::Construct;
use core::fmt::Debug;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Ident;

#[cfg_attr(feature = "serialize", derive(Deserialize, Serialize))]
#[derive(Clone)]
/// A node representing a single rust [`Construct`]. It stores its parent-child
/// relationships as well as what type of construct it represents.
pub struct Node {
    /// The id of this node.
    pub(crate) id: usize,

    /// The name of the construct.
    pub(crate) ident: Ident, // TODO

    /// A [`syn`] type (e.g. a [`syn::ItemStruct`])
    pub(crate) data: Construct,

    /// The id of the parent of this node.
    pub(crate) parent: usize,

    /// The id's of the children of this node.
    pub(crate) children: Vec<usize>,

    /// The level of this node in the tree.
    pub(crate) level: usize,
}

impl Node {
    pub(crate) fn new(data: Construct, id: usize, ident: Ident) -> Node {
        Node {
            id: id,
            ident: ident,
            data: data,
            parent: 0,
            children: vec![],
            level: 0,
        }
    }

    /// Returns a reference to the id of the Node.
    pub fn get_id(&self) -> &usize {
        &self.id
    }

    /// Returns a reference to which level in the tree this node is at.
    pub fn get_level(&self) -> &usize {
        &self.level
    }
}

impl ToTokens for Node {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.data.to_tokens(tokens)
    }
}

#[cfg(feature = "debug")]
impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let under = "──".repeat(self.level);

        write!(
            f,
            "{:^5} {:>3} {} node: {:?}, parent: {}, children: {:?}",
            self.level, self.id, under, self.data, self.parent, self.children
        )
    }
}
