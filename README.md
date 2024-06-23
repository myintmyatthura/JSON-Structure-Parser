# Fast Struct Parser with **Serde/Serde JSON** written in **Rust**

---

## A tool to serialize pure JSON objects or text-files, with a built-in parser for custom Data Structures.

---

A highly optimized and memory-efficient Struct Parser built in Rust, enhanced with serde for streamlined JSON object serializing. Parse data-structures in critical industrial applications like backend systems and databases. This parser seamlessly integrates with Rust-built software and offers extensive customization capabilities for precise data parsing.

**Features:**

- JSON Object Serializing/Deserializing for both
  text-files and JSON files.
- Context-based String Parser to parse custom
  data-structures
- Simple usage with Command-Line Arguments
- Highly optimized using efficient sub-string
  searching algorithms
- Memory-efficient and robust
- Works out of the box for serializing JSON
  objects in textfiles and parsing retrieved Structs.

---

**Pre-requisites for execution:**

- Have Rust installed
- Have Cargo installed
- Dependencies: serde, serde_json

```toml
[dependencies]
serde = { version = "1.0.203", features = ["derive"] }
serde_json = "1.0.117"
```

---

**How to use:**

Running this command will automatically serialize your JSON textfile and perform
string-parsing as per defined dictionary.

```
cargo run -- "input_dir" "output_dir"
```
