use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::panic_str;
use near_sdk::serde_json::Value;
use near_sdk::{near_bindgen, AccountId, PanicOnDefault};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[near_bindgen]
#[derive(
    BorshDeserialize, BorshSerialize, PanicOnDefault, Serialize, Deserialize, Clone, Debug,
)]
pub struct HeadCell {
    pub uuid: String,
    pub value: String,
    pub type_: String,
}

impl HeadCell {
    pub fn from_value(value: Value) -> Self {
        match value {
            Value::Object(obj) => {
                let value = obj.get("value").expect("Unsupported column cell structure, column cell should be an object with a value field").as_str().unwrap().to_string();
                let type_ = obj.get("type").expect("Unsupported column cell structure, column cell should be an object with a type field").as_str().unwrap().to_string();
                let uuid =
                    Uuid::new_v5(&Uuid::NAMESPACE_DNS, format!("{value}_").as_bytes()).to_string();
                Self { uuid, value, type_ }
            }
            _ => panic_str("Unsupported row structure, row should be an object with a value field"),
        }
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
enum CellValue {
    String(String),
    PosInt(u64),
    /// Always less than zero.
    NegInt(i64),
    /// Always finite.
    Float(f64),
    Boolean(bool),
    Null,
}

impl CellValue {
    pub fn new() -> Self {
        CellValue::Null
    }

    pub fn from_value(value: Value) -> Self {
        match value {
            Value::String(s) => CellValue::String(s),
            Value::Number(n) => {
                if n.is_u64() {
                    CellValue::PosInt(n.as_u64().unwrap())
                } else if n.is_i64() {
                    CellValue::NegInt(n.as_i64().unwrap())
                } else {
                    CellValue::Float(n.as_f64().unwrap())
                }
            }
            Value::Bool(b) => CellValue::Boolean(b),
            Value::Null => CellValue::Null,
            _ => panic_str(
                "Unsupported row cell structure, row cell should be an object with a value field",
            ),
        }
    }
}

#[near_bindgen]
#[derive(
    BorshDeserialize, BorshSerialize, PanicOnDefault, Serialize, Deserialize, Clone, Debug,
)]
pub struct RowCell {
    pub uuid: String,
    value: CellValue,
}

impl RowCell {
    pub fn from_value(value: Value) -> Self {
        match value {
            Value::Object(obj) => {
                let mut cell_value = CellValue::new();
                let uuid =
                    Uuid::new_v5(&Uuid::NAMESPACE_DNS, format!("aaa_").as_bytes()).to_string();
                for (_, value) in obj {
                    cell_value = CellValue::from_value(value);
                }

                Self {
                    uuid,
                    value: cell_value,
                }
            }
            _ => panic_str("Unsupported row structure, row should be an object with a value field"),
        }
    }
}

#[near_bindgen]
#[derive(
    BorshDeserialize, BorshSerialize, PanicOnDefault, Serialize, Deserialize, Clone, Debug,
)]
pub struct Registry {
    uuid: String,
    owner: AccountId,
    pub name: String,
    columns: Vec<HeadCell>,
    rows: Vec<HashMap<String, RowCell>>,
}

#[near_bindgen]
impl Registry {
    pub fn new(
        name: String,
        owner: AccountId,
        columns: Vec<HeadCell>,
        rows: Vec<HashMap<String, RowCell>>,
    ) -> Self {
        Self {
            uuid: Uuid::new_v5(&Uuid::NAMESPACE_DNS, &name.as_bytes()).to_string(),
            name,
            owner,
            columns,
            rows,
        }
    }
}
