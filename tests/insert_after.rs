use quote::quote;
use rustree::ast::SyntaxTree;

#[test]
fn should_cube() {
    let cube = quote! {
        fn cube(x: i32) -> i32{
            x * x
        }
    };

    let tokens = quote! {let x = x * x};
    let stmt: syn::Stmt = syn::parse2(tokens).unwrap();

    // instantiate a tree. It has a "root" node by default
    // with id = 0
    let mut tree = SyntaxTree::new();

    // tree.insert_node_after(cube);

    tree.print_tree();
}
