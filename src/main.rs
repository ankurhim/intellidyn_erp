mod db;
mod ppie;
mod enums;

use crate::db::service::Service;
use crate::ppie::instance_img::enterprise_structure::definition::financial_accounting::ox15_define_company::company::Company;
use crate::ppie::instance_img::enterprise_structure::definition::financial_accounting::ox03_define_business_area::business_area::BusinessArea;

#[tokio::main]
async fn main() {
    let new_service = Service::new().await;

    Company::create_table(&new_service).await;

    let new_company = Company::new(
        "MWS".to_string(),
        "Mass Wire and Steels Pvt. Ltd.".to_string(),
        "MWSPL".to_string(),
        "RIICO Industrial Area".to_string(),
        "301019".to_string(),
        "Bhiwadi".to_string(),
        "IN",
        "ENG",
        "INR",
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "admin".to_string()
    );

    println!("{:?}", &new_company);

    // new_company.insert(&new_service).await;

    Company::select_all(&new_service).await;
}