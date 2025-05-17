use std::collections::HashMap;

use wicked_waifus_protocol::NormalItem;

use wicked_waifus_protocol_internal::PlayerInventoryData;

pub struct PlayerInventory {
    items: HashMap<i32, i32>,
}

pub struct ItemUsage {
    pub id: i32,
    pub quantity: i32,
}

#[derive(thiserror::Error, Debug)]
pub enum InventoryError {
    #[error("Item with id: {0} doesn't exist in inventory")]
    ItemNotFound(i32),
    #[error("There isn't enough quantity of item with id: {0}, current: {1}, requested: {2}")]
    ItemNotEnough(i32, i32, i32),
    #[error("There isn't enough quantity of some of the items required")]
    ItemsNotEnough(),
}

impl PlayerInventory {
    const UNION_EXP_ID: i32 = 1;
    const SHELL_CREDIT_ID: i32 = 2;
    const ASTRITE_ID: i32 = 3;
    const LUNITE_ID: i32 = 4;
    const WAVEPLATE_ID: i32 = 5;
    const WAVEPLATE_CRYSTAL_ID: i32 = 6;

    pub fn load_from_save(data: PlayerInventoryData) -> Self {
        Self {
            items: data.items.clone(),
        }
    }

    pub fn build_save_data(&self) -> PlayerInventoryData {
        PlayerInventoryData {
            items: self.items.clone(),
        }
    }

    pub fn add_item(&mut self, id: i32, quantity: i32) -> i32 {
        self.add_internal(id, quantity)
    }

    pub fn add_items(&mut self, usages: &[ItemUsage]) -> HashMap<i32, i32> {
        self.add_many_internal(usages)
    }

    pub fn consume_item(&mut self, id: i32, quantity: i32) -> Result<i32, InventoryError> {
        Ok(*self.consume_items(&[ItemUsage { id, quantity: -quantity }])?.get(&id).unwrap())
    }

    pub fn consume_items(&mut self, usages: &[ItemUsage]) -> Result<HashMap<i32, i32>, InventoryError> {
        if !self.has_enough_items(usages) {
            return Err(InventoryError::ItemsNotEnough());
        }
        Ok(self.add_many_internal(usages))
    }

    // TODO: Check if this is item or not
    #[inline(always)]
    pub fn get_union_exp(&self) -> i32 {
        self.items.get(&Self::UNION_EXP_ID).copied().unwrap_or(0)
    }

    #[inline(always)]
    pub fn get_shell_credits(&self) -> i32 {
        self.items.get(&Self::SHELL_CREDIT_ID).copied().unwrap_or(0)
    }

    #[inline(always)]
    pub fn add_astrite(&mut self, count: i32) -> i32 {
        self.add_internal(Self::ASTRITE_ID, count)
    }

    #[inline(always)]
    pub fn get_astrite(&self) -> i32 {
        self.items.get(&Self::ASTRITE_ID).copied().unwrap_or(0)
    }

    #[inline(always)]
    pub fn get_lunite(&self) -> i32 {
        self.items.get(&Self::LUNITE_ID).copied().unwrap_or(0)
    }

    // TODO: Check if this is item or not
    #[inline(always)]
    pub fn get_waveplate(&self) -> i32 {
        self.items.get(&Self::WAVEPLATE_ID).copied().unwrap_or(0)
    }

    // TODO: Check if this is item or not
    #[inline(always)]
    pub fn get_waveplate_crystal(&self) -> i32 {
        self.items.get(&Self::WAVEPLATE_CRYSTAL_ID).copied().unwrap_or(0)
    }

    pub fn to_normal_item_list(&self) -> Vec<NormalItem> {
        self.items.iter()
            .filter(|(&id, _)| Self::WAVEPLATE_ID != id && Self::WAVEPLATE_CRYSTAL_ID != id)
            // TODO: Implement expiration
            .map(|(&id, &count)| NormalItem { id, count, expire_time: 0 })
            .collect::<Vec<_>>()
    }

    pub fn to_normal_item_list_filtered(&self, ids: Vec<i32>) -> Vec<NormalItem> {
        self.items.iter()
            .filter(|(&id, _)| ids.contains(&id))
            // TODO: Implement expiration
            .map(|(&id, &count)| NormalItem { id, count, expire_time: 0 })
            .collect::<Vec<_>>()
    }

    #[inline(always)]
    fn add_internal(&mut self, id: i32, quantity: i32) -> i32 {
        *self.items.entry(id)
            .and_modify(|count| *count += quantity)
            .or_insert(quantity)
    }

    #[inline(always)]
    fn add_many_internal(&mut self, usages: &[ItemUsage]) -> HashMap<i32, i32> {
        usages.iter()
            .filter(|usage| usage.quantity != 0)
            .map(|delta| (delta.id, self.add_internal(delta.id, delta.quantity)))
            .collect::<HashMap<_, _>>()
    }

    #[inline(always)]
    fn has_enough_item(&self, id: i32, quantity: i32) -> bool {
        self.items.get(&id).copied().unwrap_or(0) >= quantity
    }

    #[inline(always)]
    fn has_enough_items(&self, items_delta: &[ItemUsage]) -> bool {
        items_delta.iter()
            .fold(true, |is_enough, delta| {
                is_enough && self.has_enough_item(delta.id, -delta.quantity)
            })
    }
}

impl Default for PlayerInventory {
    fn default() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
}