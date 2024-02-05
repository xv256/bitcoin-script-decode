# Bitcoin Transaction Decoder

This Rust program decodes a Bitcoin transaction from its hexadecimal representation and prints its details.

## Usage

1. Install the required crates:
   ```bash
   cargo install bitcoin hex 
   ```
2. Run the program:
   ```bash
   cargo run
   ```
## Output
The program will print the following information about the transaction:

* Version
* Inputs (including outpoint, scriptSig, and sequence)
* Outputs (including value and scriptPubKey)
* Locktime

## Testing
To run the tests:
```bash
   cargo test
   ```

## Code Structure
* The mainfunction:
    * Decodes a hexadecimal string representing a transaction.
    * Deserializes the transaction data.
    * Prints the transaction details.
* The testsmodule:
    * Contains a test case to verify the transaction decoding functionality.

## Dependencies
* bitcoin: For interacting with Bitcoin data structures.
* hex: For decoding hexadecimal strings.