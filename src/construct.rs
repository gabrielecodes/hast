//! An Enum container for [`syn`] types the input code is parsed into.

use super::utils::*;
use crate::parse::parse_construct;
use quote::{quote, ToTokens};
use std::fmt::Debug;
use syn::parse::Parser;
use syn::{
    ExprAssign, ExprLet, ExprLit, ExprMacro, ExprPath, ExprReturn, ImplItemFn, ItemFn, ItemImpl,
    ItemStruct, ItemTrait, Local, Macro, Stmt, StmtMacro, TraitItemFn,
};

#[derive(Clone)]
#[non_exhaustive]
pub enum Construct {
    ItemStruct(ItemStruct),
    ItemFn(ItemFn),
    ItemImpl(ItemImpl),
    ItemTrait(ItemTrait),
    TraitItemFn(TraitItemFn),
    ImplItemFn(ImplItemFn),
    ExprPath(ExprPath),
    ExprMacro(ExprMacro),
    ExprLet(ExprLet),
    ExprLit(ExprLit),
    ExprAssign(ExprAssign),
    ExprReturn(ExprReturn),
    Local(Local),
    Stmt(Stmt),
    StmtMacro(StmtMacro),
    Macro(Macro),
    Root,
    None,
}

macro_rules! impl_construct {
    () => {};
    ($($typ:tt)*)=> {
        $(
            impl From<&syn::$typ> for Construct {
                fn from(obj: &syn::$typ) -> Self {
                    Construct::$typ(obj.to_owned())
                }
            }

            // NOTE: enable in the future when the Construct is more exaustive
            // impl From<Construct> for syn::$typ {
            //     fn from(construct: Construct) -> syn::$typ {
            //         match construct {
            //             Construct::$typ(item) => item,
            //             _ => todo!()
            //         }
            //     }
            // }

            impl TryFrom<Construct> for syn::$typ {
                type Error = syn::Error;

                fn try_from(construct: Construct) -> std::result::Result<Self, Self::Error> {
                    syn::parse2::<$typ>(construct.into())
                }
            }
        )*

        impl From<Construct> for proc_macro2::TokenStream {
            fn from(construct: Construct) -> Self {
                match construct {
                    $(
                        Construct::$typ(item) => quote! {#item},
                    )*
                    _ => quote! {},
                }
            }
        }

        impl ToTokens for Construct {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                match self {
                    $(
                        Construct::$typ(item) => item.to_tokens(tokens),
                    )*
                    _ => (),
                }
            }
        }
    };
}

impl_construct!(
    ItemStruct
    ItemFn
    ItemImpl
    ItemTrait
    TraitItemFn
    ImplItemFn
    ExprPath
    ExprLet
    ExprLit
    ExprAssign
    ExprReturn
    Stmt
    Macro
    ExprMacro
);

// StmtMacro does not implement the Parse trait
impl From<&syn::StmtMacro> for Construct {
    fn from(mac: &syn::StmtMacro) -> Self {
        Construct::StmtMacro(mac.to_owned())
    }
}

pub(crate) fn make_construct_form_tokens(
    tokens: proc_macro2::TokenStream,
) -> syn::Result<Construct> {
    parse_single.parse2(tokens)
}

macro_rules! parse_into_construct {
    () => {};
    ($($name:ident $typ:ident)+) => {
        fn parse_single<'ast>(stream: syn::parse::ParseStream) -> syn::Result<Construct> {
            while !stream.cursor().eof() {
                $(
                    if stream.peek(syn::Token![$name]) || stream.peek2(syn::Token![$name]){
                        return Ok(Construct::$typ(parse_construct(stream)?))
                    };
                )*
                if stream.peek(syn::Ident) && stream.peek2(syn::Token![!]) {
                    return Ok(Construct::Macro(parse_construct(stream)?))
                }
            }

            Ok(Construct::None)
        }
    }
}

parse_into_construct!();

parse_into_construct!(
    struct  ItemStruct
    fn      ItemFn
    impl    ItemImpl
    trait   ItemTrait
);

#[cfg(feature = "debug")]
impl Debug for Construct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Construct::ItemStruct(strct) => format!("ItemStruct: {}", strct.ident).to_string(),
            Construct::ItemFn(func) => format!("ItemFn: {}", func.sig.ident).to_string(),
            Construct::ItemImpl(item_impl) => {
                if let Some(t) = item_impl.trait_.to_owned() {
                    let pat = match_path(&t.1);
                    format!("Impl: {} for {}", pat, match_type(&item_impl.self_ty))
                } else {
                    format!("Impl: {}", match_type(&item_impl.self_ty))
                }
            }
            Construct::ItemTrait(item_trait) => {
                format!("Trait: {}", item_trait.ident.to_string())
            }
            Construct::TraitItemFn(trait_item_fn) => {
                format!("TraitItemFn: {}", trait_item_fn.sig.ident)
            }
            Construct::ImplItemFn(impl_item_fn) => {
                format!("ImplItemFn: {}", impl_item_fn.sig.ident)
            }
            Construct::ExprPath(pat) => format!("ExprPath: {}", match_path(&pat.path)),
            Construct::Local(local) => format!("Let {}", match_pat(&local.pat)),
            Construct::Stmt(stmt) => match stmt {
                syn::Stmt::Local(local) => {
                    format!("Stmt: Local: {}", match_pat(&local.pat))
                }
                syn::Stmt::Item(item) => format!("Stmt: {}", match_item(item)),
                syn::Stmt::Expr(expr, _) => format!("Stmt: {}", match_expr(expr)),
                syn::Stmt::Macro(mac) => format!("Stmt: Macro: {:?}", match_path(&mac.mac.path)),
            },
            Construct::StmtMacro(mac) => format!("Stmt: Macro: {:?}", match_path(&mac.mac.path)),
            Construct::Macro(mac) => format!("Macro: {}", match_path(&mac.path)),
            Construct::ExprMacro(expr_mac) => {
                format!("ExprMacro: {}", match_path(&expr_mac.mac.path))
            }
            Construct::ExprReturn(ret) => {
                if let Some(expr) = &ret.expr {
                    match_expr(&expr)
                } else {
                    "ExprReturn".to_string()
                }
            }
            Construct::ExprAssign(assign) => match_expr(&assign.left),
            Construct::ExprLet(let_expr) => match_expr(&let_expr.expr),
            Construct::ExprLit(lit_expr) => match_lit_expr(&lit_expr.lit),
            Construct::Root => "Root".to_string(),
            Construct::None => "None".to_string(),
        };
        write!(f, "{}", name)
    }
}
