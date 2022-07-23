use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::TreeMap;
use near_sdk::env::{panic_str, state_exists};
use near_sdk::serde_json::Value;
use near_sdk::{near_bindgen, AccountId, PanicOnDefault};
use serde::{Deserialize, Serialize};

use uuid::Uuid;

// #[near_bindgen]
// #[derive(BorshSerialize, BorshDeserialize)]
// pub struct ColumnCell {
//     uuid: String,
//     value: String, //TODO: change to ENUM possible variants
// }

// #[near_bindgen]
// #[derive( BorshSerialize, BorshDeserialize)]
// pub struct Column {
//     uuid: String,
//     cells: Vec<ColumnCell>,
// }

#[near_bindgen]
#[derive(
    BorshDeserialize, BorshSerialize, PanicOnDefault, Serialize, Deserialize, Clone, Debug,
)]
pub struct HeadCell {
    uuid: String,
    value: String,
}

#[near_bindgen]
#[derive(
    BorshDeserialize, BorshSerialize, PanicOnDefault, Serialize, Deserialize, Clone, Debug,
)]
pub struct RowCell {
    uuid: String,
    value: String,
}
#[near_bindgen]
#[derive(
    BorshDeserialize, BorshSerialize, PanicOnDefault, Serialize, Deserialize, Clone, Debug,
)]
pub struct Row {
    uuid: String,
    cells: Vec<RowCell>,
}

#[near_bindgen]
#[derive(
    BorshDeserialize, BorshSerialize, PanicOnDefault, Serialize, Deserialize, Clone, Debug,
)]
pub struct Table {
    uuid: String,
    name: String,
    columns: Vec<HeadCell>,
    rows: Vec<Row>,
}

#[near_bindgen]
impl Table {
    pub fn new(name: String, columns: Vec<HeadCell>, rows: Vec<Row>) -> Self {
        Self {
            uuid: Uuid::new_v5(&Uuid::NAMESPACE_DNS, b"some_name").to_string(),
            name,
            columns,
            rows,
        }
    }
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Contract {
    owner: AccountId,

    registries: TreeMap<AccountId, Vec<Table>>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            owner: "alice.near".parse().unwrap(),
            registries: TreeMap::new(b"t"),
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
            registries: TreeMap::new(b"t"),
        }
    }

    pub fn new_registry(
        &mut self,
        owner_id: AccountId,
        columns: Vec<Value>,
        rows: Vec<Value>,
        name: String,
    ) {
        let mut new_columns: Vec<HeadCell> = Vec::new();
        let mut new_rows: Vec<Row> = Vec::new();

        for column in columns {
            match column {
                Value::String(s) => {
                    new_columns.push(HeadCell {
                        uuid: Uuid::new_v5(&Uuid::NAMESPACE_DNS, format!("{name}_{s}_").as_bytes())
                            .to_string(),
                        value: s.to_string(),
                    });
                }
                _ => panic_str("Unsupported column structure"),
            }
        }

        for rows in &rows {
            match rows {
                Value::Array(row) => {
                    let mut new_row_cell: Vec<RowCell> = Vec::new();
                    for (i, cell) in row.iter().enumerate() {
                        match cell {
                            Value::String(s) => {
                                new_row_cell.push(RowCell {
                                    uuid: Uuid::new_v5(
                                        &Uuid::NAMESPACE_DNS,
                                        format!("{name}_{s}_{i}").as_bytes(),
                                    )
                                    .to_string(),
                                    value: s.to_string(),
                                });
                            }
                            _ => panic_str("Invalid cell"),
                        }
                    }

                    new_rows.push(Row {
                        uuid: Uuid::new_v5(&Uuid::NAMESPACE_DNS, format!("{name}_").as_bytes())
                            .to_string(),
                        cells: new_row_cell,
                    });
                }
                _ => panic_str("Unsupported rows structure"),
            }
        }

        let mut vec = Vec::new();

        let new_table = Table::new(name, new_columns, new_rows);

        vec.push(new_table.clone());

        if !self.registries.contains_key(&owner_id) {
            self.registries.insert(&owner_id, &vec);
        } else {
            self.registries.get(&owner_id).unwrap().push(new_table)
        }
    }

    pub fn get_all_registries(&self) -> Vec<Table> {
        let mut vec = Vec::new();
        for user in self.registries.iter() {
            for t in user.1 {
                vec.push(t.clone());
            }
        }
        vec
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
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
            registries: TreeMap::new(b"t"),
        };

        let columns = json!(['a', 'b', 'c']);
        let rows = json!([['1', '2', '3'], ['4', '5', '6']]);
        contract.new_registry(
            alice(),
            vec![columns.clone()],
            vec![rows.clone()],
            "testname".to_string(),
        );

        let _result = contract.get_all_registries();

        println!("{:?}", columns);
    }
}
