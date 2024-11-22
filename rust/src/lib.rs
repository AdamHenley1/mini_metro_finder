/* We're using adjacency matrices
 * because they're really cool and I love
 * them, but only if they're square matrices,
 * because then the maths with them is way
 * easier, and I have had some really bad
 * experiences trying to do ring theory stuff
 * with non-square matrices and vectors and stuff.
 */ 
pub mod api;
mod frb_generated;

/* We hope that there are less than 100 edges
 * connected to a single vertex.
 */
type Vertex = Vec<u8>;
type Matrix = Vec<Vertex>;


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
