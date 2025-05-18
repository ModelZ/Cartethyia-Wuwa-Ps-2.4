use crate::{config, logic::player::Player};
use rand::Rng;

pub struct Cartetyia {
    // basic info
    pub atk: i32,
    pub hp: i32,
    pub weapon_atk: i32, 
    pub enemy: Enemy, // enemy for calculation

    // multiplier
    pub ability_damage: f32, //ability damage multiplier
    pub damage_bonus: f32, //increase damage bonus (Areo Damage Bonus, Normal Attack Bonus, etc)
    pub damage_amplify: f32, //damage amplify (Damage Amplify, etc) same as final damage
    pub special_damage: f32, //special damage multiplier
    pub crit_damage: f32, 
}

pub struct Enemy {
    // basic info
    pub name: String,
    pub level: i32,
    pub hp: i32,
    pub def: i32,
    pub element_res: f32,

    // multiplier
    pub def_multiplier: f32,
    pub element_res_multiplier: f32,
    pub damage_taken_multiplier: f32,
}


impl Cartetyia {
    pub fn new() -> Self {
        // Load the configuration (if needed)
        //let config = config::get_config();

        // Create a new enemy instance
        let enemy = Enemy {
            name: "Lorelei".to_string(),
            level: 90,
            hp: 558_562,
            def: 1512,
            element_res: 0.1,

            def_multiplier: 1.0,
            element_res_multiplier: 1.0,
            damage_taken_multiplier: 1.0,
        };

        // Return a new instance of Cartetyia
        return Self {
            atk: 312,
            hp: 14800,
            weapon_atk: 412,
            enemy, // store the enemy in the struct

            ability_damage: 1.0, //ability damage multiplier
            damage_bonus: 1.0, //increase damage bonus (Areo Damage Bonus, Normal Attack Bonus, etc)
            damage_amplify: 1.0, //damage amplify (Damage Amplify, etc) same as final damage
            special_damage: 1.0, //special damage multiplier
            crit_damage: 1.5, 
        };
        
    }

    pub fn if_cartethyia_dmg(player: &mut Player, damage: &mut i32, damage_id: i64) {
        // Get the current role id
        let current_role_id = get_cerrent_role_id(player);

        // define modelz custom variable from json
        let mz_custom_var = &config::get_config().modelz_custom;

        // if attacker is carthetyia, apply custom damage
        if current_role_id == 1409 {
            //*damage = mz_custom_var.carthetyia_dmg;

            // Set the damage value based on the damage_id for debugging
            *damage = damage_id as i32;
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

