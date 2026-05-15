/// Utility binary: generates a fresh Solana keypair and prints the base58 private key.
/// Run with: cargo run --bin keygen
/// Paste the output into BACKEND_SIGNER_PRIVATE_KEY in .env
fn main() {
    use solana_sdk::signer::Signer;

    let keypair = solana_sdk::signature::Keypair::new();
    let private_key_b58 = bs58::encode(keypair.to_bytes()).into_string();
    let public_key = keypair.pubkey();

    println!("=== SpinBattles Backend Signer Keypair ===");
    println!("Public key  : {}", public_key);
    println!("Private key : {}", private_key_b58);
    println!();
    println!("Add to backend/.env:");
    println!("BACKEND_SIGNER_PRIVATE_KEY={}", private_key_b58);
    println!();
    println!("WARNING: Never use a keypair with real SOL or tokens as the backend signer.");
}
