# We need all the compiler optimizations we can get
cargo rustc --release -- -C target-cpu=native;
if ($?) {
    ./target/release/rust-tracer;
}