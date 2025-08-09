# `leaky`

This crate provides `Leak<T>` that encapsulates a `'static` reference to heap-allocated data. It offers a convenient way to convert common heap-owning types like `String`, `PathBuf`, and `Vec<T>` into their static reference counterparts.

## Semantics of `&'static T`

In Rust, `&'static T` has a broad semantic range. It can refer to:

1. **Compile-time constants**: Stored in the binary's read-only data section.
2. **Leaked data**: Memory originally on the heap that has been intentionally leaked.

The `Leak<T>` type provides a clear semantic distinction for the second case, explicitly indicating that the contained `'static` reference points to leaked heap memory. This improves code clarity and helps developers understand the origin and lifecycle of the data.

## The `&'static T` Deserialization Problem

A significant limitation of `&'static T` is its incompatibility with deserialization. While `&'static` references can be serialized, they cannot be deserialized as `serde` is not designed to introduce implicit memory leaks. It cannot fulfill the `'static` lifetime requirement for newly created data without an explicit instruction to leak that memory. This makes `&'static T` difficult to use directly in configurations or (trusted) data payloads.

The `Leak<T>` type solves this by implementing `serde::Deserialize`. It deserializes into an owned type (e.g., `Box<str>`), and then immediately **leaks** that data to produce a `'static` reference. This allows `Leak<T>` to be used seamlessly in deserialization.

## Examples

```Rust
use serde::{Serialize, Deserialize};
use leaky::Leak;
use std::path::Path;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct Config {
    log_file: Leak<Path>,
    server_name: Leak<str>,
    #[serde(default)]
    api_keys: Leak<[Leak<str>]>,
}

fn main() {
    let json_data = r#"{
        "log_file": "/var/log/app.log",
        "server_name": "production-web-01",
        "api_keys": ["key1", "key2"]
    }"#;

    let config: Config = serde_json::from_str(json_data).unwrap();

    assert_eq!(config.log_file.as_ref(), Path::new("/var/log/app.log"));
    assert_eq!(config.server_name.as_ref(), "production-web-01");
    assert_eq!(config.api_keys.as_ref(), &["key1".to_string(), "key2".to_string()]);
}
```