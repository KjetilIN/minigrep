# Minigrep - A Rust Cli tool for grep 

Rust cli tool that copies the behavior for grep, but only for a single file. 

## How to use it?

You can run the following command to get a list of commands:

```terminal
cargo run -- cm
```

Give it a test by searching for **Oslo** in the given `test.txt` file:

```terminal
cargo run -- Oslo test.txt
```