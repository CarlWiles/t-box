# T-Box

T-Box is a command-line tool to manage file templates. It allows you to register templates, list all available templates, and create files from these templates with ease.

## Features

- Register a template by providing its name and file path.
- List all registered templates.
- Create files from registered templates.

## Installation

### Prerequisites

- **Rust**: Make sure you have [Rust](https://www.rust-lang.org/) installed on your system. You can install Rust using the following command:

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Build and Install

To install this CLI globally:

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/t-box.git
   cd t-box
   ```

2. Install the CLI globally using Cargo:

   ```bash
   cargo install --path .
   ```

3. Verify the installation:

   ```bash
   t-box --help
   ```

## Usage

### Register a Template

To register a new template:

```bash
t-box register <template-name> <template-file-path>
```

**Example**:

```bash
t-box register solidity solidity_template.txt
```

### List Registered Templates

To view all registered templates:

```bash
t-box list
```

**Example Output**:

```
Registered templates:
- solidity
- rust
```

### Create a File from a Template

To create a file from a registered template:

```bash
t-box create <template-name> <target-file-path>
```

**Example**:

```bash
t-box create solidity example.sol
```

If the template `solidity` is registered, the file `example.sol` will be created with its content.

## Development

### Project Structure

```plaintext
src/
├── main.rs          # Entry point of the application
├── cli.rs           # Command-line argument parsing
├── template.rs      # Template management logic
├── utils.rs         # Utility functions (optional)
```

### Running Locally

To run the project locally:

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/t-box.git
   cd t-box
   ```

2. Run the project:

   ```bash
   cargo run -- <command>
   ```

   For example:

   ```bash
   cargo run -- list
   ```

### Testing

To run tests:

```bash
cargo test
```

### Linting

Ensure the code follows Rust's style guidelines using `rustfmt`:

```bash
cargo fmt
```

## Contributing

Contributions are welcome! Please follow the steps below:

1. Fork the repository.
2. Create a new branch for your feature/bugfix.
3. Make your changes and commit them with descriptive messages.
4. Submit a pull request.

### Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](https://www.contributor-covenant.org/). By participating, you are expected to uphold this code.

## License

This project is licensed under the [MIT License](LICENSE).

## Acknowledgments

Special thanks to the open-source community for providing inspiration and support.
"""

# Writing the content to README.md

with open("README.md", "w") as file:
file.write(readme_content)

"README.md has been successfully created!"
