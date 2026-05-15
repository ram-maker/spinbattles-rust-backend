use sha2::{Digest, Sha256};

/// Mock Data
///
/// Provides deterministic mock data for wallet balance checks.
/// Balance data is seeded from the wallet address so each candidate gets
/// consistent but unique-looking responses.
///
/// Battle data is served by the game server (port 8081), not from here.

/// Deterministic pseudo-random u64 from a string seed + index.
fn seeded_rand(seed: &str, index: u64) -> u64 {
    let input = format!("{}-{}", seed, index);
    let hash = Sha256::digest(input.as_bytes());
    u64::from_be_bytes(hash[24..32].try_into().unwrap())
}

/// Returns a realistic mock SBR token balance for an address.
/// Deterministic per address, in the range 0–9999 SBR (as lamports string).
pub fn get_mock_balance(address: &str) -> (String, String) {
    let addr = address.to_lowercase();
    let whole = seeded_rand(&addr, 99) % 10000;
    let fraction = seeded_rand(&addr, 100) % 100;
    let lamports = whole * 1_000_000_000 + fraction * 10_000_000;
    let ui = format!("{}.{:02} SBR", whole, fraction);
    (lamports.to_string(), ui)
}
