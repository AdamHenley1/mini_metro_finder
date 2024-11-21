pub mod api;
mod frb_generated;

/* Ignore this function, it's just here
 * for testing purposes.
 */
#[allow(dead_code)]
fn rust_main(name: String) -> String {
    let mut message = String::from("hello, ");
    message.push_str(&name);
    message.push_str("!");
    message
}
