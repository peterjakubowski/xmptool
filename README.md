# Rust-XMP-Metadata-Tool

A Rust-based CLI tool for reading and writing XMP photo metadata, powered by xmpkit.

```command
# to read metadata
cargo run -- read <path> <field>
cargo run -- read path/to/img.jpg persons
cargo run -- read path/to/img.jpg regions

# to write metadata
cargo run -- write <path> <field> <value>
cargo run -- write path/to/img.jpg persons "My Name"
cargo run -- write path/to/img.jpg regions "Your Name"
```
