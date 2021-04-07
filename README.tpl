# {{crate}}

{{readme}}

## Testing

- Test it normally using `cargo test`
- Test it with Miri using `cargo +nightly miri test`
- Test it with ThreadSanitizer using `RUSTFLAGS="-Zsanitizer=thread --cfg tsan" cargo +nightly test --tests -Zbuild-std --target={your target triple}`
- Test it with Loom using `RUSTFLAGS="--cfg loom" cargo test --tests --release`

## License

{{license}}
