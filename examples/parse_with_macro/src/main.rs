//! This unnecessarily complicated example show how we can extend a nested
//! function and the surrounding implementation

use parse_with_macro::tree_parse;

fn main() {
    tree_parse! {
        struct Container<'a, T> {
            value: &'a i32,
            phantom: core::marker::PhantomData<T>,
        }

        trait Operations {
            fn square(&self) -> Result<i32, Box<dyn std::error::Error>>;
        }

        impl<'a, T> Operations for Container<'a, T> {
            fn square(&self) -> Result<i32, Box<dyn std::error::Error>> {
                fn do_the_squaring(x: i32) -> i32 {
                    x * x
                }

                let result = do_the_squaring(*self.value);
                Ok(result)
            }
        }
    }
}
