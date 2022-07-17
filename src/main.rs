mod db;
mod ppie;
use std::rc::Rc;
use std::cell::RefCell;
use crate::db::service::Service;
use crate::ppie::instance_img::enterprise_structure::definition::financial_accounting::ox15_define_company::company::Company;
use crate::ppie::instance_img::enterprise_structure::definition::financial_accounting::ox03_define_business_area::business_area::BusinessArea;

#[tokio::main]
async fn main() {
    let new_service = Service::new().await;
    let serv = Rc::new(RefCell::new(new_service));

    BusinessArea::create_table(&serv.borrow()).await;

    let new_business_area = BusinessArea::new(
        "110".to_string(),
        "Unit-1".to_string(),
        "Plant-1".to_string(),
        "".to_string(),
        "admin".to_string()
    );

    // new_business_area.insert(&serv.borrow()).await;

    BusinessArea::select_all(&serv.borrow()).await;
}
