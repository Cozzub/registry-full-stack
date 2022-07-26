use crate::*;

#[near_bindgen]
impl Contract {
    #[private]
    pub fn is_name_exist(&mut self, name: &String) -> bool {
        for registry in &self.registries {
            if registry.name == *name {
                return true;
            }
        }
        false
    }
}
