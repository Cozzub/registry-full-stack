use crate::*;

#[near_bindgen]
impl Contract {
    pub fn get_all_registries(&self) -> Vec<Registry> {
        let mut registries = Vec::new();

        for registry in &self.registries {
            registries.push(registry.clone());
        }

        registries
    }

    pub fn get_cells_count(&self) -> u64 {
        self.cells_counter
    }
}
