use uuid::Uuid;
use serde_derive::{ Serialize, Deserialize };
use crate::db::service::Service;
use std::{
    rc::Rc,
    cell::RefCell
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Company {
    pub company_pk: String,
    pub company_key: String,
    pub company_name: String,
    pub company_name_2: String,
    pub street: String,
    pub postal_code: String,
    pub city: String,
    pub country: String,
    pub language: String,
    pub currency: String,
    pub view_maintenance: String,
    pub request: String,
    pub short_desc: String,
    pub created_by: String,
    pub created_on: std::time::SystemTime,
    pub modified_by: String,
    pub modified_on: std::time::SystemTime,
}

impl Default for Company {
    fn default() -> Self {
        Company {
            company_pk: Uuid::new_v4().to_string(),
            company_key: "201".to_string(),
            company_name: "".to_string(),
            company_name_2: "".to_string(),
            street: "".to_string(),
            postal_code: "".to_string(),
            city: "".to_string(),
            country: "IN".to_string(),
            language: "ENG".to_string(),
            currency: "INR".to_string(),
            view_maintenance: "".to_string(),
            request: "".to_string(),
            short_desc: "".to_string(),
            created_by: "".to_string(),
            created_on: std::time::SystemTime::now(),
            modified_by: "".to_string(),
            modified_on: std::time::SystemTime::now(),
        }
    }
}

impl Company {
    pub fn new(
        c_key: String,
        c_name: String,
        c_name_2: String,
        strt: String,
        p_code: String,
        city: String,
        cntry: String,
        lang: String,
        curr: String,
        view_maint: String,
        requ: String,
        short_desc: String,
        c_by: String,
    ) -> Self {
        let mut new_company = Company::default();

        new_company.company_key = c_key;
        new_company.company_name = c_name.to_string();
        new_company.company_name_2 = c_name_2.to_string();
        new_company.street = strt.to_string();
        new_company.postal_code = p_code.to_string();
        new_company.city = city.to_string();
        new_company.country = cntry;
        new_company.language = lang;
        new_company.currency = curr;
        new_company.view_maintenance = view_maint.to_string();
        new_company.request = requ.to_string();
        new_company.short_desc = short_desc.to_string();
        new_company.created_by = c_by.to_string();

        new_company
    }

    pub async fn create_table(service: &Service) {

        service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS company (
                id SERIAL PRIMARY KEY,
                company_pk VARCHAR NOT NULL,
                company_key VARCHAR NOT NULL,
                company_name VARCHAR NOT NULL,
                company_name_2 VARCHAR,
                street VARCHAR NOT NULL,
                postal_code VARCHAR NOT NULL,
                city VARCHAR NOT NULL,
                country VARCHAR NOT NULL,
                language VARCHAR NOT NULL,
                currency VARCHAR NOT NULL,
                view_maintenance VARCHAR,
                request VARCHAR,
                short_desc VARCHAR,
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
           "INSERT INTO company (
            company_pk,
            company_key,
            company_name,
            company_name_2,
            street,
            postal_code,
            city,
            country,
            language,
            currency,
            view_maintenance,
            request,
            short_desc,
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
                $9,
                $10,
                $11,
                $12,
                $13,
                $14,
                $15,
                $16,
                $17
            )",
            &[
                &self.company_pk,
                &self.company_key,
                &self.company_name,
                &self.company_name_2,
                &self.street,
                &self.postal_code,
                &self.city,
                &self.country,
                &self.language,
                &self.currency,
                &self.view_maintenance,
                &self.request,
                &self.short_desc,
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
        println!("Company Details:");
    for row in &service.client
        .query("SELECT 
        id,
        company_pk,
        company_key,
        company_name,
        company_name_2,
        street,
        postal_code,
        city,
        country,
        language,
        currency,
        view_maintenance,
        request,
        short_desc,
        created_by,
        created_on,
        modified_by,
        modified_on FROM company", &[])
        .await
        .unwrap()
    {
        let id: i64 = row.get(0);
        let company_pk: &str = row.get(1);
        let company_key: &str = row.get(2);
        let company_name: &str = row.get(3);
        let company_name_2: &str = row.get(4);
        let street: &str = row.get(5);
        let postal_code: &str = row.get(6);
        let city: &str = row.get(7);
        let country: &str = row.get(8);
        let language: &str = row.get(9);
        let currency: &str = row.get(10);
        let view_maintenance: &str = row.get(11);
        let request: &str = row.get(12);
        let short_desc: &str = row.get(13);
        let created_by: &str = row.get(14);
        let created_on: std::time::SystemTime = row.get(15);
        let modified_by: &str = row.get(16);
        let modified_on: std::time::SystemTime = row.get(17);
        println!("{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {:?} {} {:?}", 
            id,
            company_pk,
            company_key,
            company_name,
            company_name_2,
            street,
            postal_code,
            city,
            country,
            language,
            currency,
            view_maintenance,
            request,
            short_desc,
            created_by,
            created_on,
            modified_by,
            modified_on
        );
    }
    }
}