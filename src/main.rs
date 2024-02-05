use bitcoin::blockdata::transaction::Transaction;
use bitcoin::consensus::encode::{deserialize};

fn main() {
    let hex_str = "020000000001010ccc140e766b5dbc884ea2d780c5e91e4eb77597ae64288a42575228b79e234900000000000000000002bd37060000000000225120245091249f4f29d30820e5f36e1e5d477dc3386144220bd6f35839e94de4b9cae81c00000000000016001416d31d7632aa17b3b316b813c0a3177f5b6150200140838a1f0f1ee607b54abf0a3f55792f6f8d09c3eb7a9fa46cd4976f2137ca2e3f4a901e314e1b827c3332d7e1865ffe1d7ff5f5d7576a9000f354487a09de44cd00000000";
    let tx_bytes = hex::decode(hex_str).unwrap();

    let tx: Transaction = deserialize(&tx_bytes).unwrap();

    println!("Version: {}", tx.version);
    println!("Inputs:");
    for input in tx.input.iter() {
        println!("  - Outpoint: {}", input.previous_output);
        println!("  - ScriptSig: {}", hex::encode(input.script_sig.to_bytes()));
        println!("  - Sequence: {}", input.sequence);
    }
    println!("Outputs:");
    for output in tx.output.iter() {
        println!("  - Value: {} satoshis", output.value);
        println!("  - ScriptPubKey: {}", hex::encode(output.script_pubkey.to_bytes()));
    }
    println!("Locktime: {}", tx.lock_time);
}

#[cfg(test)]
mod tests {
    use bitcoin::blockdata::transaction::Transaction;
    use bitcoin::consensus::encode::deserialize;
    
    #[test]
    fn test_decode_transaction() {
        let transaction_hex = "020000000001010ccc140e766b5dbc884ea2d780c5e91e4eb77597ae64288a42575228b79e234900000000000000000002bd37060000000000225120245091249f4f29d30820e5f36e1e5d477dc3386144220bd6f35839e94de4b9cae81c00000000000016001416d31d7632aa17b3b316b813c0a3177f5b6150200140838a1f0f1ee607b54abf0a3f55792f6f8d09c3eb7a9fa46cd4976f2137ca2e3f4a901e314e1b827c3332d7e1865ffe1d7ff5f5d7576a9000f354487a09de44cd00000000";
        let transaction_bytes = hex::decode(transaction_hex).unwrap();
        let transaction: Transaction = deserialize(&transaction_bytes).unwrap();

        assert_eq!(transaction.version, 2);
        assert_eq!(transaction.input.len(), 1);
        assert_eq!(transaction.output.len(), 2);
        assert_eq!(transaction.lock_time, 0);
    }
}
