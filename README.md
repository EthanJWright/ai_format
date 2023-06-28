# AI Format

AI Format is a command-line tool that utilizes OpenAI's ChatGPT API to format text using AI-generated completions. It takes a prompt and generates formatted text based on the provided input.

## Prerequisites

To use AI Format, you need to have the following:

- An OpenAI API key: You can obtain an API key from the OpenAI website.
- Rust programming language and Cargo package manager: Make sure you have Rust and Cargo installed on your system. Visit the official Rust website for installation instructions.

## Installation

1. Clone the AI Format repository:

   ```
   git clone https://github.com/EthanJWright/ai_format.git
   ```

2. Navigate to the project directory:

   ```
   cd ai_format
   ```

3. Build the project using Cargo:

   ```
   cargo build --release
   ```

## Usage

Once you have built the project, you can use AI Format as follows:

```
cargo run --release -- <open_ai_key> <filename> <prompt>
```

- `<open_ai_key>`: Your OpenAI API key.
- `<filename>`: The name of the file you want to format.
- `<prompt>`: The prompt to provide for AI-based formatting.

Make sure to replace `<open_ai_key>`, `<filename>`, and `<prompt>` with the appropriate values.

## Examples

Here are some examples of how to use AI Format:

```
cargo run --release -- ABCDEFG12345678 example.txt "format text as HTML"
```

This command will use the OpenAI API key "ABCDEFG12345678" to format the contents of the file "example.txt" based on the provided prompt.

## License

This project is licensed under the [MIT License](LICENSE).

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request on the GitHub repository.

## Acknowledgements

AI Format is built using the [chatgpt-rs](https://docs.rs/chatgpt_rs/latest/chatgpt/) library.

## Disclaimer

AI Format is an experimental project and should be used with caution. The AI-generated completions may not always produce desired results, and it is advisable to review and verify the formatted output.

## Contact

For any inquiries or questions, please contact [ewright362@gmail.com](mailto:ewright362@gmail.com).
