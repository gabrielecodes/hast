//! Parse a simple sequence of rust constructs and print the tree

use quote::quote;
use rustree::speculative_parse;

fn main() {
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
}
