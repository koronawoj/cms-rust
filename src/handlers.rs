use serde::{Deserialize, Serialize};
use crate::models::{CreateOrUpdateCustomerDTO};
use crate::AppError;
use crate::data_access::DBAccessManager;
use warp::{self, Reply, Rejection, reply, reject, http::StatusCode};


#[derive(Debug, Deserialize, Clone)]
pub struct CreateOrUpdateCustomer {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub address: String,
}

impl CreateOrUpdateCustomer {
    pub fn to_dto(&self) -> CreateOrUpdateCustomerDTO {
        CreateOrUpdateCustomerDTO{
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
            address: self.address.clone(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct IdResponse {
    pub id: i64,
}

impl IdResponse {
    pub fn new(id: i64) -> IdResponse {
        IdResponse{id}
    }
}

pub async fn list_customers(db_manager: DBAccessManager) -> Result<impl Reply, Rejection> {
    log::info!("handling list of customers");
    let result = db_manager.list_customers();
    respond(result, StatusCode::OK)
}

pub async fn create_customer(db_manager: DBAccessManager, new_customer: CreateOrUpdateCustomer) -> Result<impl Reply, Rejection> {

    log::info!("handling add customer");

    let create_customer_dto = new_customer.to_dto();

    let id_response = db_manager.create_customer(create_customer_dto).map(|customer|
        { IdResponse::new(customer.guid) }
    );

    respond(id_response, StatusCode::CREATED)

}

// pub async fn get_customer(guid: String, db_manager: DBAccessManager) -> Result<Box<dyn Reply>, Infallible> {
//     let customers = db.lock().await;
//     for customer in customers.iter() {
//         if customer.guid == guid {
//             return Ok(Box::new(reply::json(&customer)));
//         }
//     }

//     Ok(Box::new(StatusCode::NOT_FOUND))
// }

pub async fn update_customer(customer_id: i64, db_manager: DBAccessManager, updated_customer: CreateOrUpdateCustomer) -> Result<impl Reply, Rejection> {
    log::info!("handling update customer");
    let response = db_manager.update_customer(customer_id, updated_customer);
    respond(response, StatusCode::OK)
}

pub async fn delete_customer(customer_id: i64, db_manager: DBAccessManager) -> Result<impl Reply, Rejection> {
    log::info!("handling delete customer");
    let result = db_manager.delete_customer(customer_id).map(|_| -> () {()});
    respond(result, StatusCode::NO_CONTENT)
}


fn respond<T: Serialize>(result: Result<T, AppError>, status: StatusCode) -> Result<impl Reply, Rejection> {
    match result {
        Ok(response) => {
            Ok(reply::with_status(reply::json(&response), status))
        }
        Err(err) => {
            log::error!("Error while trying to respond: {}", err.to_string());
            Err(reject::custom(err))
        }
    }
}