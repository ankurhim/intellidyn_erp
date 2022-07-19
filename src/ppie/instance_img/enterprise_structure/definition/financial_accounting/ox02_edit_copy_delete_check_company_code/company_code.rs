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
    pub company_code_pk: Uuid,
    pub title: String,
    pub company_code: String,
    pub company_name: String,
    pub city: String,
    pub country: Country,
    pub language: Language,
    pub currency: Currency,
    pub search_terms: Vec<String>,
    pub street_house_no: String,
    pub postal_code: String,
    pub telephone: Option<String>,
    pub tel_ext: Option<String>,
    pub mobile_phone: Option<String>,
    pub fax: Option<String>,
    pub fax_ext: Option<String>,
    pub email: String,
    pub comments: Option<String>,
    pub request: Option<String>,
    pub short_desc: Option<String>,
    pub created_by: String,
    pub created_on: SystemTime,
    pub modified_by: Option<String>,
    pub modified_on: Option<SystemTime>,
}

impl CompanyCode {
    pub fn new(
        company_code_pk: Uuid,
        title: String,
        company_code: String,
        company_name: String,
        city: String,
        country: &str,
        language: &str,
        currency: &str,
        search_terms: Vec<String>,
        street_house_no: String,
        postal_code: String,
        telephone: String,
        tel_ext: String,
        mobile_phone: String,
        fax: String,
        fax_ext: String,
        email: String,
        comments: String,
        request: String,
        short_desc: String,
        created_by: String,
    ) -> Self {
        CompanyCode {
            company_code_pk: Uuid::new_v4(),
            title: title.to_string(),
            company_code: company_code.to_string(),
            company_name: company_name.to_string(),
            city: city.to_string(),
            country: Country::from_str(country).unwrap(),
            language: Language::from_str(language).unwrap(),
            currency: Currency::from_str(currency).unwrap(),
            search_terms: search_terms,
            street_house_no: street_house_no.to_string(),
            postal_code: postal_code.to_string(),
            telephone: match &telephone.len() {
                0 => None,
                _ => Some(telephone.to_string())
            },
            tel_ext: match &tel_ext.len() {
                0 => None,
                _ => Some(tel_ext.to_string())
            },
            mobile_phone: match &mobile_phone.len() {
                0 => None,
                _ => Some(mobile_phone.to_string())
            },
            fax: match &fax.len() {
                0 => None,
                _ => Some(fax.to_string())
            },
            fax_ext: match &fax_ext.len() {
                0 => None,
                _ => Some(fax_ext.to_string())
            },
            email: email.to_string(),
            comments: match &comments.len() {
                0 => None,
                _ => Some(comments.to_string())
            },
            request: match &request.len() {
                0 => None,
                _ => Some(request.to_string())
            },
            short_desc: match &short_desc.len() {
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
            "CREATE TABLE IF NOT EXISTS company_code (
                id SERIAL PRIMARY KEY,
                company_code_pk VARCHAR NOT NULL,
                title VARCHAR NOT NULL,
                company_code VARCHAR NOT NULL,
                company_name VARCHAR NOT NULL,
                city VARCHAR NOT NULL,
                country VARCHAR NOT NULL,
                language VARCHAR NOT NULL,
                currency VARCHAR NOT NULL,
                search_terms VARCHAR[] NOT NULL,
                street_house_no VARCHAR NOT NULL,
                postal_code VARCHAR NOT NULL,
                telephone VARCHAR,
                tel_ext VARCHAR,
                mobile_phone VARCHAR,
                fax VARCHAR,
                fax_ext VARCHAR,
                email VARCHAR NOT NULL,
                comments VARCHAR,
                request VARCHAR,
                short_desc VARCHAR,
                created_by VARCHAR NOT NULL,
                created_on TIMESTAMP NOT NULL DEFAULT NOW(),
                modified_by TEXT,
                modified_on TIMESTAMP,
                CONSTRAINT company_code_unique UNIQUE (company_code)
            )",
            &[]
        )
        .await
        .unwrap();
    }

    pub async fn insert(&self, service: &Service) {
        service.client
        .execute(
           "INSERT INTO company_code (
            company_code_pk,
            title,
            company_code,
            company_name,
            city,
            country,
            language,
            currency,
            search_terms,
            street_house_no,
            postal_code,
            telephone,
            tel_ext,
            mobile_phone,
            fax,
            fax_ext,
            email,
            comments,
            request,
            short_desc,
            created_by,
            created_on,
            modified_by,
            modified_on
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23 $24)",
            &[
                &self.company_code_pk,
                &self.title,
                &self.company_code,
                &self.company_name,
                &self.city,
                &self.country.to_string(),
                &self.language.to_string(),
                &self.currency.to_string(),
                &self.search_terms,
                &self.street_house_no,
                &self.postal_code,
                &match &self.telephone {
                    Some(c) => c,
                    None => ""
                },
                &match &self.tel_ext {
                    Some(c) => c,
                    None => ""
                },
                &match &self.mobile_phone {
                    Some(c) => c,
                    None => ""
                },
                &match &self.fax {
                    Some(c) => c,
                    None => ""
                },
                &match &self.fax_ext {
                    Some(c) => c,
                    None => ""
                },
                &self.email,
                &match &self.comments {
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
        println!("Company Code Details:");
    for row in &service.client
        .query("SELECT * FROM company_code", &[])
        .await
        .unwrap()
        {
            let id: i64 = row.get(0);
            let company_code_pk: &str = row.get(1);
            let title: &str = row.get(2);
            let company_code: &str = row.get(3);
            let company_name: &str = row.get(4);
            let city: &str = row.get(5);
            let country: &str = row.get(6);
            let language: &str = row.get(7);
            let currency: &str = row.get(8);
            let search_terms: Vec<String> = row.get(9);
            let street_house_no: &str = row.get(10);
            let postal_code: &str = row.get(11);
            let telephone: &str = row.get(12);
            let tel_ext: &str = row.get(13);
            let mobile_phone: &str = row.get(14);
            let fax: &str = row.get(15);
            let fax_ext: &str = row.get(16);
            let email: &str = row.get(17);
            let comments: &str = row.get(18);
            let request: &str = row.get(19);
            let short_desc: &str = row.get(20);
            let created_by: &str = row.get(21);
            let created_on: SystemTime = row.get(22);
            let modified_by: &str = row.get(23);
            let modified_on: SystemTime = row.get(24);
            println!("{} {} {} {} {} {} {} {} {} {:?} {} {} {} {} {} {} {} {} {} {} {} {} {:?} {} {:?}", 
                id,
                company_code_pk,
                title,
                company_code,
                company_name,
                city,
                country,
                language,
                currency,
                search_terms,
                street_house_no,
                postal_code,
                telephone,
                tel_ext,
                mobile_phone,
                fax,
                fax_ext,
                email,
                comments,
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