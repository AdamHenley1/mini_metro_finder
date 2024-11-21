pub mod api;
mod frb_generated;


fn rust_main(name: String) -> String {
    let mut message = String::from("hello, ");
    message.push_str(&name);
    message.push_str("!");
    message
}
