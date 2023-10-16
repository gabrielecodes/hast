use super::ast::SyntaxTree;
use super::construct::Construct;
use syn::parse::{Parse, ParseStream};
use syn::visit::Visit;

pub(crate) fn speculative_parse_inner(stream: ParseStream) -> syn::Result<SyntaxTree> {
    let ast = SyntaxTree::new();
    parse_inner(ast, &stream)
}

pub(crate) fn parse_construct<T: Parse>(stream: ParseStream) -> syn::Result<T> {
    T::parse(stream)
}

macro_rules! parse_inner_macro {
    () => {};
    ($($name:ident $typ:ident $fn_name:ident)+) => {
        fn parse_inner(mut tree: SyntaxTree, stream: &ParseStream) -> syn::Result<SyntaxTree> {
            while !stream.cursor().eof() {
                $(
                    if stream.peek(syn::Token![$name]) || stream.peek2(syn::Token![$name]){
                        let child = Construct::$typ(parse_construct(stream)?);
                        let c: syn::$typ = child.try_into()?;
                        tree.set_current_level(1);
                        tree.$fn_name(&c);
                        tree.set_current_level(0);
                    };
                )*
                if stream.peek(syn::Ident) && stream.peek2(syn::Token![!]) {
                    let child = Construct::Macro(parse_construct(stream)?);
                    let c: syn::Macro = child.try_into()?;
                    tree.set_current_level(1);
                    tree.visit_macro(&c);
                    tree.set_current_level(0);
                }
            }
            Ok(tree)
        }
    }
}

parse_inner_macro!();

parse_inner_macro!(
    struct  ItemStruct  visit_item_struct
    fn      ItemFn      visit_item_fn
    impl    ItemImpl    visit_item_impl
    trait   ItemTrait   visit_item_trait
);
