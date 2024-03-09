### CliNop

This libarary is too small to use it as library, it was created just for testing purposes. If you think it can be usefull to you better just grab source.


Usage:

cargo.toml
```
[dependencies]
clinop = { git = "https://github.com/NopFault/clinop", branch = "master" }
```

main.rs

```rust
use clinop::CliNop;

fn main() {
    let arguments: CliNop = CliNop::new(env::args().collect());

    let template_base = arguments.get::<String>("test");

    println!("{:?}", template_base);
}
```

Run:

```bash
cargo run -- --test testas
```

It should return `Some("testas")`
