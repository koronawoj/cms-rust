use serde::{Serialize, Deserialize};
use crate::schema::customers;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable)]
pub struct CustomerDTO {
    pub guid: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub address: String,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "customers"]
pub struct CreateOrUpdateCustomerDTO {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub address: String,
}
