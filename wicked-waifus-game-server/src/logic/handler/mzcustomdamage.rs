use crate::{config, logic::player::Player};
use rand::Rng;

// If Cartetyia do damage
pub struct Cartetyia {
    pub atk: i32,
    pub hp: i32,
    pub weapon_atk: i32,
}

impl Cartetyia {
    pub fn new() -> Self {
        // Load the configuration (if needed)
        //let config = config::get_config();

        // Define Cartetyia stats
        Self {
            atk: 312,
            hp: 14800,
            weapon_atk: 412,
        }
    }

    pub fn if_cartethyia_dmg(player: &mut Player, damage: &mut i32) {
        // Get the current role id
        let current_role_id = get_cerrent_role_id(player);

        // define modelz custom variable from json
        let mz_custom_var = &config::get_config().modelz_custom;

        // if attacker is carthetyia, apply custom damage
        if current_role_id == 1409 {
            *damage = mz_custom_var.carthetyia_dmg;
        }
    }

    
}
pub fn random_damage() -> (i32, bool) {
    // Generate a random damage value
    let mut damage = 0;
    // Add base random damage
    damage += rand::rng().random_range(2000..10000); // Random base damage between 2000 and 10000
    // Calculate critical hit with a 50% probability
    let crit_chance = rand::rng().random_bool(0.5); // 50% chance for a critical hit
    if crit_chance {
        return ((damage as f32 * 3.5) as i32, crit_chance); // Apply critical hit multiplier 3.5x
    }
    return (damage, crit_chance); // No critical hit
}

pub fn get_cerrent_role_id(player: &Player) -> i32 {
    // Get the current formation
    let current_formation = player.formation_list
        .values()
        .find(|f| f.is_current)
        .unwrap();
    // Get the current role id
    let current_role_id = current_formation.cur_role;
    return current_role_id;
}
