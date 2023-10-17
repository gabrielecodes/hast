use rustree::speculative_parse;

#[proc_macro]
pub fn tree_parse(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = speculative_parse(tokens).expect("Error during parsing");

    // print the tree
    ast.print_tree();

    // add a print statement in the function "do_the_squaring"
    // ast.find_node_mut("do_the_squaring");

    proc_macro::TokenStream::new()
}
