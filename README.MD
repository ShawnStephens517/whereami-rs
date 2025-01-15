# Whereami-Rust

![Project Icon](whereami.webp)

Whereami-Rust is a Rust-based command-line tool that provides information about your current location in the filesystem. It is designed to be simple, fast, and easy to use.

## Features

- Display the current working directory
- Show the absolute path of the current directory
- Cross-platform support (Windows, macOS, Linux)

## Installation

To install Whereami-Rust, you need to have Rust and Cargo installed on your system. You can install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org/).

Once you have Rust installed, you can clone the repository and build the project:

```sh
git clone https://github.com/shawnstephens517/whereami-rs.git
cd whereami-rs
cargo build --release
```

The compiled binary will be located in the `target/release` directory.

## Usage

To use Whereami-Rust, simply run the compiled binary:

```sh
./target/release/whereami-rust
```

This will output the absolute path of your current working directory.

## Contributing

Contributions are welcome! If you have any ideas, suggestions, or bug reports, please open an issue or submit a pull request.

1. Fork the repository
2. Create a new branch (`git checkout -b feature-branch`)
3. Make your changes
4. Commit your changes (`git commit -m 'Add some feature'`)
5. Push to the branch (`git push origin feature-branch`)
6. Open a pull request

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Acknowledgements

- [Rust Programming Language](https://www.rust-lang.org/)
- [Cargo](https://doc.rust-lang.org/cargo/)

## Contact

For any questions or inquiries, please file an issue.
