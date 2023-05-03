/*
Example of a handler function.
*/

// 'a is a lifetime annotation
// syntax: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
pub async fn hello_world<'a>() -> &'a str {
    "Hello, World!"
}
