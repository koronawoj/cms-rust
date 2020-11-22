use diesel::prelude::*;
use crate::models::{CustomerDTO, CreateOrUpdateCustomerDTO};
use crate::errors::{AppError,ErrorType};
use crate::handlers::{CreateOrUpdateCustomer};
use crate::pool::AsyncPool;

pub struct DBAccessManager {
    pool: AsyncPool,
}

impl DBAccessManager {
    pub fn new(pool: AsyncPool) -> DBAccessManager {
        DBAccessManager {
            pool
        }
    }

    pub async fn list_customers(&self) -> Result<Vec<CustomerDTO>, AppError> {
        use super::schema::customers::dsl::*;

        let connection = self.pool.get().await;

        connection.deref(move |connection| {
            customers
                .load(connection)
                .map_err(|err| {
                    AppError::from_diesel_err(err, "while listing customers")
                })
        }).await
    }

    pub async fn create_customer(&self, dto: CreateOrUpdateCustomerDTO) -> Result<CustomerDTO, AppError> {
        use super::schema::customers;

        let connection = self.pool.get().await;

        connection.deref(move |connection| {
                
            diesel::insert_into(customers::table) // insert into customers table
                .values(&dto) // use values from CreateCustomerDTO
                .get_result(connection) // execute query
                .map_err(|err| {
                    AppError::from_diesel_err(err, "while creating customer")
                }) // if error occurred map it to AppError
        }).await
    }

    pub async fn update_customer(&self, customer_id: i64, updated_customer: CreateOrUpdateCustomer) -> Result<usize, AppError> {
        use super::schema::customers::dsl::*;

        let connection = self.pool.get().await;

        connection.deref(move |connection| {
            
            let updated = diesel::update(customers)
                .filter(id.eq(customer_id))
                .set((
                    first_name.eq(updated_customer.first_name),
                    last_name.eq(updated_customer.last_name),
                    email.eq(updated_customer.email),
                    address.eq(updated_customer.address),
                ))
                .execute(connection)
                .map_err(|err| {
                    AppError::from_diesel_err(err, "while updating customer")
                })?;

            if updated == 0 {
                return Err(AppError::new("Customer not found", ErrorType::NotFound))
            }
            return Ok(updated)
        }).await
    }

    pub async fn delete_customer(&self, customer_id: i64) -> Result<usize, AppError> {
        use super::schema::customers::dsl::*;

        let connection = self.pool.get().await;

        connection.deref(move |connection| {
                
            let deleted = diesel::delete(customers.filter(id.eq(customer_id)))
                .execute(connection)
                .map_err(|err| {
                    AppError::from_diesel_err(err, "while deleting customer")
                })?;

            if deleted == 0 {
                return Err(AppError::new("Customer not found", ErrorType::NotFound))
            }
            return Ok(deleted)
        }).await
    }

    pub async fn get_customer(&self, customer_id: i64) -> Result<Vec<CustomerDTO>, AppError> {
        use super::schema::customers::dsl::*;

        let connection = self.pool.get().await;

        connection.deref(move |connection| {
            customers
                .filter(id.eq(customer_id))
                .load(connection)
                .map_err(|err| {
                    AppError::from_diesel_err(err, "while listing customers")
                })
        }).await
    }
}