# rlox - A Rust-Based Lox Interpreter

`rlox` is an interpreter for the Lox programming language, implemented in Rust. This project is inspired by the *Crafting Interpreters* book by Robert Nystrom and follows its principles to build a tree-walking interpreter. The goal of `rlox` is to provide a simple, expressive scripting language while demonstrating core interpreter concepts, all while taking advantage of Rust’s powerful safety and performance features.

## Features

- **Lexical Analysis**: `rlox` includes a tokenizer that breaks down source code into tokens, categorizing elements like keywords, identifiers, operators, and literals.
- **Parsing**: A recursive descent parser processes tokens into an Abstract Syntax Tree (AST) to represent code structure.
- **Interpretation**: `rlox` executes code by evaluating the AST directly, supporting basic control flow, expressions, functions, and variable scope.
- **Error Handling**: Detailed error messages help developers debug their `rlox` scripts, with syntax and runtime error reporting.

## File Extension

Lox programs for `rlox` use the `.rlox` extension.

## Getting Started

### Prerequisites

To build and run `rlox`, you will need:

- [Rust](https://www.rust-lang.org) installed (recommended version: 1.50 or later)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/will-create/rlox.git
   cd rlox
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the interpreter with an example `.rlox` file:
   ```bash
   cargo run --release examples/script.rlox
   ```

## Usage

You can run `rlox` files directly from the command line:

```bash
rlox path/to/script.rlox
```

## Example Code

Here’s a simple `.rlox` script:

```rlox
// Hello World in rlox
print "Hello, world!";

// Variables and expressions
var x = 10;
print x * 2;

// Functions
fun greet(name) {
    print "Hello, " + name + "!";
}
greet("Rustaceans");
```

## Project Structure

- **src**: Contains all source code files for `rlox`, including lexer, parser, AST definitions, and interpreter logic.
- **examples**: Sample `.rlox` files showcasing various language features.
- **tests**: Unit and integration tests.

## Contributing

Contributions are welcome! If you would like to contribute to `rlox`, feel free to fork the repository, make changes, and submit a pull request. Please ensure all code passes the tests before submitting.

## License

This project is licensed under the MIT License.

