Rust benchmarks pretending (on my system) that you can significantly boost performance by un-inlining an expression (wrapping it in a private function called only in one place)

For fellow [Rust](https://github.com/rust-lang/rust)-noobs, to demonstrate you need to:
  `rustup default nightly`
  `cargo bench`
