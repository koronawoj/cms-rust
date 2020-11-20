use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::models::{CustomerDTO, CreateOrUpdateCustomerDTO};
use crate::errors::{AppError,ErrorType};
use crate::handlers::{CreateOrUpdateCustomer};

type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

pub struct DBAccessManager {
    connection: PooledPg,
}

impl DBAccessManager {
    pub fn new(connection: PooledPg) -> DBAccessManager {
        DBAccessManager {connection}
    }

    pub fn list_customers(&self) -> Result<Vec<CustomerDTO>, AppError> {
        use super::schema::customers::dsl::*;

        customers
            .load(&self.connection)
            .map_err(|err| {
                AppError::from_diesel_err(err, "while listing customers")
            })
    }

    pub fn create_customer(&self, dto: CreateOrUpdateCustomerDTO) -> Result<CustomerDTO, AppError> {
        use super::schema::customers;

        diesel::insert_into(customers::table) // insert into customers table
            .values(&dto) // use values from CreateCustomerDTO
            .get_result(&self.connection) // execute query
            .map_err(|err| {
                AppError::from_diesel_err(err, "while creating customer")
            }) // if error occurred map it to AppError
    }

    pub fn update_customer(&self, customer_id: i64, updated_customer: CreateOrUpdateCustomer) -> Result<usize, AppError> {
        use super::schema::customers::dsl::*;

        let updated = diesel::update(customers)
            .filter(id.eq(customer_id))
            .set((
                first_name.eq(updated_customer.first_name),
                last_name.eq(updated_customer.last_name),
                email.eq(updated_customer.email),
                address.eq(updated_customer.address),
            ))
            .execute(&self.connection)
            .map_err(|err| {
                AppError::from_diesel_err(err, "while updating book status")
            })?;

        if updated == 0 {
            return Err(AppError::new("Book not found", ErrorType::NotFound))
        }
        return Ok(updated)
    }

    pub fn delete_customer(&self, customer_id: i64) -> Result<usize, AppError> {
        use super::schema::customers::dsl::*;

        let deleted = diesel::delete(customers.filter(id.eq(customer_id)))
            .execute(&self.connection)
            .map_err(|err| {
                AppError::from_diesel_err(err, "while deleting customer")
            })?;

        if deleted == 0 {
            return Err(AppError::new("Customer not found", ErrorType::NotFound))
        }
        return Ok(deleted)
    }

    // pub fn get_customer(&self, customer_id: i64) -> Result<CustomerDTO, AppError> {
    //     use super::schema::customers::dsl::*;
    // }

}