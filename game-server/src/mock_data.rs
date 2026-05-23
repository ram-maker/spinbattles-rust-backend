use sha2::{Digest, Sha256};

use crate::models::BattleResult;

const BATTLE_TEMPLATES: &[(&str, u64)] = &[
    ("Arena Duel", 50),
    ("Tournament Finals", 250),
    ("Guild War", 150),
    ("Ranked Match", 75),
    ("Championship", 500),
    ("Quick Battle", 25),
];

fn seeded_rand(seed: &str, index: u64) -> u64 {
    let input = format!("{}-{}", seed, index);
    let hash = Sha256::digest(input.as_bytes());
    u64::from_be_bytes(hash[24..32].try_into().unwrap())
}

/// Generates deterministic battle results for a given player address.
pub fn generate_battles(address: &str) -> Vec<BattleResult> {
    let addr = address.to_lowercase();
    let count = (seeded_rand(&addr, 0) % 3) + 2;
    let mut battles = Vec::new();

    for i in 0..count {
        let template_index = (seeded_rand(&addr, i + 1) % BATTLE_TEMPLATES.len() as u64) as usize;
        let (name, base_reward) = BATTLE_TEMPLATES[template_index];

        let variance = seeded_rand(&addr, i + 10) % 50;
        let reward_sbr = base_reward + variance;
        let reward_lamports = reward_sbr * 1_000_000_000;

        let battle_num = seeded_rand(&addr, i + 20) % 9000 + 1000;
        let battle_id = format!("battle_{}", battle_num);

        let hours_ago = (seeded_rand(&addr, i + 30) % 48) + 1;
        let played_at = chrono::Utc::now().timestamp() - (hours_ago as i64 * 3600);

        battles.push(BattleResult {
            battle_id,
            battle_name: name.to_string(),
            player: address.to_string(),
            reward_sbr: reward_sbr.to_string(),
            reward_lamports: reward_lamports.to_string(),
            outcome: "win",
            played_at,
        });
    }

    battles
}

/// Derives a reward amount from a battle_id for the verify endpoint.
pub fn derive_reward_from_battle_id(battle_id: &str) -> String {
    let hash = Sha256::digest(battle_id.as_bytes());
    let base = u64::from_be_bytes(hash[24..32].try_into().unwrap()) % 500 + 25;
    (base * 1_000_000_000).to_string()
}
