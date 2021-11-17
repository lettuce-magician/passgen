# Passgen

## Usage
passgen (validate/generate) (password/length)

## Example
passgen validate 12345 // Checks the password and print out the problems

passgen generate 8 // Create a random generated 8 character password.

## Building
```sh
git clone https://github.com/lettuce-magician/passgen

cargo build --release
```