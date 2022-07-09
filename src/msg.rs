use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// InstantiateMsg = Firsts off the start of the contract. Gets contract instance started (like a constructor in OOP).
// ExecuteMsg = POST, PUT data -> a contract. Set some values, 
// QueryMsg = If a value is stored, and you want to read it, you can query value from the contract & get the binary data.
// CustomResponse = returning data from a query route, just auto generated
// MigrateMsg = Complex, we will not be touching

// These are macros for how it saves the data to/from JSON & Binary (how messages are stored under the hood)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg { // schema/instantiate_msg.json when you run `cargo schema`
    pub val: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CustomMsg { val: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    CustomMsg { val: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CustomResponse {
    val: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}
