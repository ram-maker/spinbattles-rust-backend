use crate::mock_data::{derive_reward_from_battle_id, generate_battles};
use crate::models::{BattlesResponse, VerifyResponse};

/// GET /battles/:address — won battles for a player.
pub fn get_battles(address: &str) -> BattlesResponse {
    let battles = generate_battles(address);
    BattlesResponse {
        success: true,
        address: address.to_string(),
        battles,
    }
}

/// GET /battles/:battle_id/verify — eligibility check for the reward backend.
pub fn verify_battle(battle_id: &str) -> VerifyResponse {
    let (eligible, reward_lamports, reason) = if battle_id.starts_with("battle_") {
        let reward = derive_reward_from_battle_id(battle_id);
        (true, reward, "Battle result verified")
    } else {
        (false, "0".to_string(), "Battle ID not found or invalid format")
    };

    VerifyResponse {
        success: true,
        battle_id: battle_id.to_string(),
        eligible,
        reward_lamports,
        reason,
    }
}
