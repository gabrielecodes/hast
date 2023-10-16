use quote::quote;
use rustree::ast::SyntaxTree;

// Add a node with a print statement
fn main() {
    let square = quote! {
        fn square(x: i32) -> i32{
            x * x
        }
    };

    let tokens = quote! {print!("The square of 2 is {}", square(2))};

    let mut tree = SyntaxTree::new();
    tree.append_tokenstream(square);
    tree.append_tokenstream(tokens);

    tree.print_tree();

    let stream = tree.get_tokenstream();

    println!("stream : {stream}");
}
