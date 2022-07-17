mod db;
mod company;
use std::rc::Rc;
use std::cell::RefCell;
use crate::db::service::Service;
use crate::company::Company;

#[tokio::main]
async fn main() {
    let new_service = Service::new().await;
    let serv = Rc::new(RefCell::new(new_service));

    Company::create_table(&serv.borrow()).await;

    let new_company = Company::new(
        "201".to_string(),
        "Mass Wire and Steels Pvt. Ltd.".to_string(),
        "".to_string(),
        "RIICO Industrial Estate".to_string(),
        "301019".to_string(),
        "Bhiwadi".to_string(),
        "IN".to_string(),
        "ENG".to_string(),
        "INR".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "admin".to_string()
    );

    Company::select_all(&serv.borrow()).await;
}
