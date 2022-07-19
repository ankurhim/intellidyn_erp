use uuid::Uuid;
use serde_derive::{ Serialize, Deserialize };
use std::time::SystemTime;
use crate::db::service::Service;
use crate::enums::{
    country::Country,
    language::Language,
    currency::Currency
};
use std::str::FromStr;

/// Company is an independent organizational element which is required to consolidate 
/// two are more company codes data at country level or segment level or continental 
/// level. Under group company, you generate consolidated financial statements as per 
/// requirements of an organization.
#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyCode {
    pub company_code_pk: String,
    pub company_code: String,
    pub company_name: String,
    pub city: String,
    pub country: Country,
    pub language: Language,
    pub currency: Currency,
    pub view_maintenance: Option<String>,
    pub request: Option<String>,
    pub short_desc: Option<String>,
    pub created_by: String,
    pub created_on: SystemTime,
    pub modified_by: Option<String>,
    pub modified_on: Option<SystemTime>,
}

impl Company {
    pub fn new(
        company_key: String,
        company_name: String,
        company_name_2: String,
        street: String,
        postal_code: String,
        city: String,
        country: &str,
        language: &str,
        currency: &str,
        view_maintenance: String,
        request: String,
        short_desc: String,
        created_by: String
    ) -> Self {
        Company {
            company_pk: Uuid::new_v4().to_string(),
            company_key: company_key.to_string(),
            company_name: company_name.to_string(),
            company_name_2: match company_name_2.len() {
                0 => None,
                _ => Some(company_name_2.to_string())
            },
            street: street.to_string(),
            postal_code: postal_code.to_string(),
            city: city.to_string(),
            country: Country::from_str(country).unwrap(),
            language: Language::from_str(language).unwrap(),
            currency: Currency::from_str(currency).unwrap(),
            view_maintenance: match view_maintenance.len() {
                0 => None,
                _ => Some(view_maintenance.to_string())
            },
            request: match request.len() {
                0 => None,
                _ => Some(request.to_string())
            },
            short_desc: match short_desc.len() {
                0 => None,
                _ => Some(short_desc.to_string())
            },
            created_by: created_by.to_string(),
            created_on: SystemTime::now(),
            modified_by: None,
            modified_on: None,
        }
    }

    pub async fn create_table(service: &Service) {
        service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS company (
                id SERIAL PRIMARY KEY,
                company_pk TEXT NOT NULL,
                company_key TEXT NOT NULL,
                company_name TEXT NOT NULL,
                company_name_2 TEXT,
                street TEXT NOT NULL,
                postal_code TEXT NOT NULL,
                city TEXT NOT NULL,
                country TEXT NOT NULL,
                language TEXT NOT NULL,
                currency TEXT NOT NULL,
                view_maintenance TEXT,
                request TEXT,
                short_desc TEXT,
                created_by TEXT NOT NULL,
                created_on TIMESTAMP NOT NULL DEFAULT NOW(),
                modified_by TEXT,
                modified_on TIMESTAMP,
                CONSTRAINT company_key_unique UNIQUE (company_key)
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
                &match &self.company_name_2 {
                    Some(c) => c,
                    None => ""
                },
                &self.street,
                &self.postal_code,
                &self.city,
                &self.country.to_string(),
                &self.language.to_string(),
                &self.currency.to_string(),
                &match &self.view_maintenance {
                    Some(c) => c,
                    None => ""
                },
                &match &self.request {
                    Some(c) => c,
                    None => ""
                },
                &match &self.short_desc {
                    Some(c) => c,
                    None => ""
                },
                &self.created_by,
                &self.created_on,
                &match &self.modified_by {
                    Some(c) => c,
                    None => ""
                },
                &match &self.modified_on {
                    Some(c) => tokio_postgres::types::Timestamp::Value(c),
                    None => tokio_postgres::types::Timestamp::NegInfinity
                },
            ]
        )
        .await
        .unwrap();
    }
    pub async fn select_all(service: & Service) {
        println!("Company Details:");
    for row in &service.client
        .query("SELECT * FROM company", &[])
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
        let created_on: SystemTime = row.get(15);
        let modified_by: &str = row.get(16);
        let modified_on: SystemTime = row.get(17);
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