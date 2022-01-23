/* Compile: 
        rustc -g s9_1_1_unrecoverable_errors_with_panic.rs -o main
        RUST_BACKTRACE=1 ./main
*/
fn main() {
    panic!("crash and burn");
}