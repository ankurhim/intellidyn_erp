use uuid::Uuid;
use serde_derive::{ Serialize, Deserialize };
use crate::db::service::Service;
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyCodeToCompany{
    pub assign_pk: Uuid,
    pub company_code: String,
    pub company_name: String,
    pub city: String,
    pub company: String,
    pub created_by: String,
    pub created_on: SystemTime,
    pub modified_by: Option<String>,
    pub modified_on: Option<SystemTime>,
}

impl Default for CompanyCodeToCompany {
    fn default() -> Self {
        CompanyCodeToCompany {
            assign_pk: Uuid::new_v4(),
            company_code: "".to_string(),
            company_name: "".to_string(),
            city: "".to_string(),
            company: "".to_string(),
            created_by: "".to_string(),
            created_on: SystemTime::now(),
            modified_by: None,
            modified_on: None,
        }
    }
}

impl CompanyCodeToCompany {
    pub fn new(
        company_code: String,
        company_name: String,
        city: String,
        company: String,
        created_by: String,
    ) -> Self {
        let mut new_assignment = CompanyCodeToCompany::default();

        new_assignment.company_code = company_code.to_string();
        new_assignment.company_name = company_name.to_string();
        new_assignment.city = city.to_string();
        new_assignment.company = company.to_string();
        new_assignment.created_by = created_by.to_string();

        new_assignment
    }

    pub async fn create_table(service: &Service) {

        service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS assign_company_code_to_company (
                id SERIAL PRIMARY KEY,
                assign_pk VARCHAR NOT NULL,
                company_code VARCHAR NOT NULL,
                company_name VARCHAR NOT NULL,
                city VARCHAR NOT NULL,
                company VARCHAR NOT NULL,
                created_by VARCHAR NOT NULL,
                created_on TIMESTAMP NOT NULL DEFAULT NOW(),
                modified_by VARCHAR,
                modified_on TIMESTAMP
            )",
            &[]
        )
        .await
        .unwrap();
    }

    pub async fn insert(&self, service: &Service) {
        service.client
        .execute(
           "INSERT INTO assign_company_code_to_company (
            assign_pk,
            company_code,
            company_name,
            city,
            company,
            created_by,
            created_on,
            modified_by,
            modified_on
            ) VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6,
                $7,
                $8,
                $9
            )",
            &[
                &self.assign_pk,
                &self.company_code,
                &self.company_name,
                &self.city,
                &self.company,
                &self.created_by,
                &self.created_on,
                &self.modified_by,
                &self.modified_on
            ]
        )
        .await
        .unwrap();
    }

    pub async fn select_all(service: & Service) {
        println!("Assign Company Code to Company Details:");
    for row in &service.client
        .query("SELECT * FROM assign_company_code_to_company", &[])
        .await
        .unwrap()
    {
        let id: i64 = row.get(0);
        let assign_pk: &str = row.get(1);
        let company_code: &str = row.get(2);
        let company_name: &str = row.get(3);
        let city: &str = row.get(4);
        let company: &str = row.get(5);
        let created_by: &str = row.get(6);
        let created_on: SystemTime = row.get(7);
        let modified_by: &str = row.get(8);
        let modified_on: SystemTime = row.get(9);
        println!("{} {} {} {} {} {} {} {:?} {} {:?}", 
            id,
            assign_pk,
            company_code,
            company_name,
            city,
            company,
            created_by,
            created_on,
            modified_by,
            modified_on
        );
    }
    }
}