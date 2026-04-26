# Buup - The Text Utility Belt

Buup is a versatile text transformation toolkit that provides a dependency-free core library and CLI for common text manipulations like encoding/decoding, formatting, cryptography, (coming soon compression/decompression), and more written in pure dependency-free Rust.

It is designed to be a simple, lightweight, **open**, **secure**, **provably fast** and easy to integrate.

Drop-in replacement for all of those dodgy online text transformation tools you've ever used in the past except the batteries are included (and they are all in pure Rust).

It includes a [web application](https://benletchford.github.io/buup/) which is of course written in pure Rust (WASM via [Dioxus](https://dioxuslabs.com/)) as a separate workspace member.

<div align="center">
    <a href="https://benletchford.github.io/buup/">
        <img src="buup_web/assets/web-screenshot-dark.png">
    </a>
</div>

## Key Features

- **Zero Dependencies**: The core `buup` library and its CLI implement all transformations without external dependencies
- **Multiple Interfaces**: CLI for terminal workflows and Web UI for interactive use
- **Extensible Design**: Easy to add new transformers by implementing the `Transform` trait
- **Strong Typing**: Full type safety with comprehensive error handling
- **Thread Safety**: All transformers are designed to be safely used concurrently
- **Performance**: Optimized for speed and memory usage

## Ways to Use Buup

Buup offers three distinct ways to transform your text:

### 1. Web Application

A modern, responsive web application for interactive text transformations proudly built with [Dioxus](https://dioxuslabs.com/).

|                            Dark Mode                            |                            Light Mode                            |
| :-------------------------------------------------------------: | :--------------------------------------------------------------: |
| <img src="buup_web/assets/web-screenshot-dark.png" width="400"> | <img src="buup_web/assets/web-screenshot-light.png" width="400"> |

From source:

```bash
# Serve the web UI (requires Dioxus CLI)
cd buup_web
dx serve
```

Build for production:

```bash
dx build
```

### 2. Command Line Interface

Zero-dependency CLI for quick transformations in your terminal workflows.

```bash
# Installation
cargo binstall buup # or cargo install buup

# List available transformers
buup list

# Examples
buup base64encode "Hello, world!"     # Encode text directly
buup urldecode -i encoded.txt         # Decode from file
echo "Hello" | buup hexencode         # Pipe from stdin
```

### 3. Rust Library

Integrate Buup's transformers directly into your Rust applications.

```bash
# Add to your project
cargo add buup
```

```rust
use buup::{transformer_from_id, Transform, Base64Encode};

// Option 1: Use a specific transformer struct
let encoded = Base64Encode.transform("Hello, Library!").unwrap();
println!("{}", encoded); // SGVsbG8sIExpYnJhcnkh

// Option 2: Look up a transformer by its ID
let transformer = transformer_from_id("base64decode").unwrap();
let decoded = transformer.transform(&encoded).unwrap();
println!("{}", decoded); // Hello, Library!
```

## Tiny Binary Size

Buup is incredibly lightweight, with the entire buup library of transformers and the CLI binary compiling down to just **652K** on arm64 (again with no external dependencies).

This tiny footprint makes Buup perfect for:

- **Including in resource-constrained environments** e.g. embedded systems
- **Fast startup times** for CLI operations
- **Minimal dependencies** means fewer security vulnerabilities and simpler maintenance

*Note: Binary size may vary slightly across different platforms.*

## Available Transformers

The following transformers are currently available in Buup:

```bash
Available transformers:

ENCODERS:
  ascii_to_hex    - Convert ASCII characters to their hexadecimal representation.
  base64encode    - Encode text to Base64 format
  bin_to_hex      - Convert binary numbers to hexadecimal.
  binaryencode    - Encode text into its binary representation (space-separated bytes).
  dec_to_bin      - Convert decimal numbers to binary.
  dec_to_hex      - Convert decimal numbers to hexadecimal.
  hex_to_bin      - Converts hexadecimal input to its binary representation (Base64 encoded).
  hexencode       - Encode text to hexadecimal representation
  htmlencode      - Encodes special HTML characters into their entity representation (e.g., < to &lt;).
  morseencode     - Encode text to Morse code
  rot13           - Applies the ROT13 substitution cipher to the input text.
  urlencode       - Encode text for use in URLs

DECODERS:
  base64decode    - Decode Base64 text to plain text
  bin_to_dec      - Convert binary numbers to decimal.
  binarydecode    - Decode space-separated binary representation back to text.
  hex_to_ascii    - Decodes a hexadecimal string into its ASCII representation.
  hex_to_dec      - Converts hexadecimal numbers to their decimal representation.
  hexdecode       - Decodes a hexadecimal string into its original bytes, then interprets as UTF-8.
  htmldecode      - Decodes HTML entities (e.g., &lt;) back into characters (<).
  jwtdecode       - Decodes a JSON Web Token (JWT) without verifying the signature.
  morsedecode     - Decodes Morse code into text.
  urldecode       - Decode URL-encoded text

FORMATTERS:
  htmltomarkdown  - Converts HTML to Markdown format
  jsonformatter   - Formats (pretty-prints) a JSON string.
  jsonminifier    - Minifies a JSON string, removing unnecessary whitespace.
  linenumberadder - Adds line numbers to the beginning of each line.
  linenumberremover - Removes line numbers (and optional delimiters) from the beginning of each line.
  markdowntohtml  - Converts Markdown text to HTML format
  sqlformatter    - Formats SQL queries with proper indentation and spacing
  sqlminifier     - Minifies SQL queries by removing unnecessary whitespace and formatting
  xmlformatter    - Format XML code with proper indentation
  xmlminifier     - Compress XML by removing unnecessary whitespace

CRYPTOGRAPHY:
  md5hash         - Calculates the MD5 hash of the input string.
  sha1hash        - Computes the SHA-1 hash of the input text (Warning: SHA-1 is cryptographically weak)
  sha256hash      - Computes the SHA-256 hash of the input text
  uuid5_generate  - Generates a version 5 UUID based on namespace and name using SHA-1. Input format: "namespace|name". Namespace can be a UUID or one of: dns, url, oid, x500.

COMPRESSION:
  deflatecompress - Compresses input using the DEFLATE algorithm (RFC 1951) and encodes the output as Base64.
  deflatedecompress - Decompresses DEFLATE input (RFC 1951). Expects Base64 input.
  gzipcompress    - Compresses input using Gzip (RFC 1952) and encodes the output as Base64.
  gzipdecompress  - Decompresses Gzip formatted input (RFC 1952). Expects Base64 input.

COLORS:
  hex_to_hsl      - Converts hex color code to HSL format
  hex_to_rgb      - Converts hex color code to RGB format
  hsl_to_hex      - Converts HSL color to hex format
  hsl_to_rgb      - Converts HSL color to RGB format
  rgb_to_hex      - Converts RGB color to hex format
  rgb_to_hsl      - Converts RGB color to HSL format

OTHERS:
  cameltosnake    - Converts camelCase or PascalCase to snake_case
  color_code_convert - Converts between different color formats (HEX, RGB, HSL, CMYK)
  csvtojson       - Converts CSV data to JSON format
  jsontocsv       - Converts a JSON array of objects into CSV format.
  linesorter      - Sorts lines alphabetically.
  slugify         - Converts text into a URL-friendly slug (lowercase, dashes, removes special chars)
  snaketocamel    - Converts snake_case to camelCase
  text_stats      - Calculates basic text statistics (lines, words, chars, sentences)
  textreverse     - Reverses the input text
  uniquelines     - Removes duplicate lines, preserving the order of first occurrence.
  urlparser       - Parses a URL into its components (scheme, authority, path, query, fragment)
  uuid_generate   - Generates a version 4 UUID. Input is ignored. WARNING: Uses a non-cryptographically secure PRNG.
  whitespaceremover - Removes all whitespace (spaces, tabs, newlines) from the input text.

EXAMPLES:
  buup base64encode "Hello, world!"     # Encode text directly
  buup urldecode -i encoded.txt         # Decode from file
  echo "Hello" | buup hexencode         # Pipe from stdin
```

### Update README.md with `buup list`

```bash
cargo run --bin update_artifacts
```

## Contributing

Contributions are welcome! When adding new transformers or modifying code, please ensure:

1. **Zero external dependencies** in the core `buup` library
2. **Comprehensive tests** covering functionality and edge cases
3. **Clear error handling** using `TransformError`
4. Run `cargo test --workspace` and `cargo clippy --workspace -- -D warnings`

### Creating Custom Transformers

**Basic Structure**

To create a custom transformer:

1. Create a new struct that implements the `Transform` trait
2. Add the struct to the registry in `lib.rs`

**Step 1: Example Implementation**

Here's a simple example of a custom transformer that reverses text:

```rust
use buup::{Transform, TransformError, TransformerCategory};

/// Text Reverse transformer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextReverse;

impl Transform for TextReverse {
    fn name(&self) -> &'static str {
        "Text Reverse"
    }

    fn id(&self) -> &'static str {
        "textreverse"
    }

    fn category(&self) -> TransformerCategory {
        TransformerCategory::Other
    }

    fn description(&self) -> &'static str {
        "Reverses the input text"
    }

    fn transform(&self, input: &str) -> Result<String, TransformError> {
        Ok(input.chars().rev().collect())
    }

    fn default_test_input(&self) -> &'static str {
        "Example Input"
    }
}
```

**Step 2: Add to Registry**

In `lib.rs`, add your transformer to the `register_builtin_transformers` function:

```rust,ignore
fn register_builtin_transformers() -> Registry {
    let mut registry = Registry {
        transformers: HashMap::new(),
    };
    // ... existing registrations ...

    // Import your new transformer struct
    use crate::transformers::my_transformer::MyNewTransformer;

    // Add your new transformer instance
    registry.transformers.insert(MyNewTransformer.id(), &MyNewTransformer);

    registry
}
```

**Step 3: Export Your Transformer (Optional)**

If your transformer will be used directly, add it to the exports in `transformers/mod.rs`:

```rust,ignore
// src/transformers/mod.rs
mod base64_decode;
// ... other mods ...
mod my_transformer; // Your new module

pub use base64_decode::Base64Decode;
// ... other uses ...
pub use my_transformer::MyNewTransformer; // Your new transformer
```

**Step 4: Add Inverse Support (Optional)**

If your transformer has an inverse operation, update the `inverse_transformer` function in `lib.rs`:

```rust,ignore
pub fn inverse_transformer(t: &dyn Transform) -> Option<&'static dyn Transform> {
    match t.id() {
        // ... existing matches ...
        "my_encoder" => transformer_from_id("my_decoder").ok(), // Your transformer pair
        "my_decoder" => transformer_from_id("my_encoder").ok(), // Your transformer pair
        _ => None,
    }
}
```

**Tips for Creating Good Transformers**

1. Follow the naming convention of existing transformers
2. Provide clear and concise descriptions
3. Make sure your transformer is thread-safe (impl Sync+Send)
4. Consider implementing pairs of transformers for inverse operations
5. Add comprehensive tests for your transformer
