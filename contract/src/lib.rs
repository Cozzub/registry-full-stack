mod registry;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::state_exists;
use near_sdk::serde_json::Value;
use near_sdk::{near_bindgen, AccountId};

use crate::registry::{HeadCell, Registry, RowCell};

// Define the contract structure
#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Contract {
    owner: AccountId,
    registries: Vec<Registry>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            owner: "alice.near".parse().unwrap(),
            registries: Vec::new(),
        }
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(owner_id: AccountId) -> Self {
        assert!(!state_exists(), "Already initialized");

        Self {
            owner: owner_id.clone(),
            registries: Vec::new(),
        }
    }

    pub fn new_registry(
        &mut self,
        owner_id: AccountId,
        name: String,
        columns: Vec<Value>,
        rows: Vec<Value>,
    ) {
        let new_columns = columns
            .iter()
            .map(|column| HeadCell::from_value(column.clone()))
            .collect();

        let new_rows = rows
            .iter()
            .map(|row| {
                let new_row = row
                    .as_object()
                    .expect("Unsupported row structure, row  should be an object")
                    .iter()
                    .map(|(key, value)| {
                        let new_row = RowCell::from_value(value.clone());
                        (key.to_string(), new_row)
                    })
                    .collect();
                new_row
            })
            .collect();

        let new_table = Registry::new(name, owner_id, new_columns, new_rows);

        self.registries.push(new_table);
    }

    pub fn get_all_registries(&self) -> Vec<Registry> {
        let mut registries = Vec::new();

        for registry in &self.registries {
            registries.push(registry.clone());
        }

        registries
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::serde_json::json;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    fn alice() -> AccountId {
        AccountId::try_from("alice.near".to_string()).unwrap()
    }

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn test_create_registry() {
        // set up the mock context into the testing environment
        let context = get_context(alice());
        testing_env!(context.build());
        // instantiate a contract variable with the counter at zero
        let mut contract = Contract {
            owner: alice(),
            registries: Vec::new(),
        };

        let columns = vec![
            json!( {
                "value": "column1",
                "type": "string",
            }),
            json!( {
                "value": "column2",
                "type": "number",
            }),
        ];

        let row1 = json!({"date": {"value": "Fri Jul 01 2022"}, "number": {"value": 1}, "text": {"value": "Test"}});

        let row2 = json!({"date": {"value": "Fri Jul 01 2022"}, "number": {"value": -1}, "text": {"value": "Test"}});

        let row3 = json!({"date": {"value": "Fri Jul 01 2022"}, "number": {"value": 0.1}, "text": {"value": "Test"}});

        contract.new_registry(
            alice(),
            "testname".to_string(),
            columns.clone(),
            vec![row1.clone(), row2.clone()],
        );

        contract.new_registry(
            alice(),
            "testname".to_string(),
            columns.clone(),
            vec![row2.clone(), row1.clone(), row3.clone()],
        );

        let _result = contract.get_all_registries();

        println!("{:#?}", _result);
    }
}
