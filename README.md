# JSON PARSER

## Description

> [!IMPORTANT]  
> **WIP**
>
> This is a work in progress and is not yet ready for use (and may never be).

(Educational and recreational) Implementation of a simple JSON parser in Rust.

## Usage

```bash
cargo run -- [FILE]
```

or compile with

```bash
cargo build
```

and then

```bash
./target/debug/json_parser [FILE]
```

or put the binary in your path and use it as a command line tool.

```bash
cp ./target/debug/json_parser /usr/local/bin
json_parser [FILE]
```

## Example

For the following JSON file : 

```json
{
    "key": "value",
    "key-n": 101,
    "key-b": true,
    "key-o": {
        "inner key": "inner value"
    },
    "key-l": ["list value"]
}
```

the output will be :

```
JsonObject of size 5 {
    Key: key, Value:     String: "value"
    Key: key-n, Value:     Number: 101
    Key: key-b, Value:     Boolean: true
    Key: key-o, Value:     JsonObject of size 1 {
        Key: inner key, Value:         String: "inner value"
        }
    Key: key-l, Value:     JsonArray of size 1 [
                String: "list value"
        ]
}
```

## Contribution

Any remarks or suggestions are very welcome, feel free to open an issue or a pull request.
