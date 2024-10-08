# Codify.rs

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.70%2B-blue)](https://rust-lang.org)
[![Package](https://img.shields.io/crates/v/codify)](https://crates.io/crates/codify)

**Codify.rs translates between types from different programming languages.**

🚧 _This is presently under heavy construction._

## 🛠️ Prerequisites

- [Rust](https://rust-lang.org) 1.70+

## ⬇️ Installation

### Installation via Cargo

```bash
cargo add codify
```

## 👉 Examples

### Importing the library

```rust
use codify::*;
```

### Converting a C++ type to Rust

```rust
println!("{}", cpp::Type::Bool.to_rust());
```

## 📚 Reference

## 👨‍💻 Development

```bash
git clone https://github.com/artob/codify.rs.git
```

- - -

[![Share on Twitter](https://img.shields.io/badge/share%20on-twitter-03A9F4?logo=twitter)](https://twitter.com/share?url=https://github.com/artob/codify.rs&text=Codify.rs)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/artob/codify.rs&title=Codify.rs)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hacker%20news-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/artob/codify.rs&t=Codify.rs)
[![Share on Facebook](https://img.shields.io/badge/share%20on-facebook-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/artob/codify.rs)
