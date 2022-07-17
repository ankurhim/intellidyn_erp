use uuid::Uuid;
use serde_derive::{ Serialize, Deserialize };
use crate::db::service::Service;
use std::{
    rc::Rc,
    cell::RefCell
};

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessArea {
    pub business_area_pk: String,
    pub b_area: String,
    pub description: String,
    pub request: String,
    pub short_desc: String,
    pub created_by: String,
    pub created_on: std::time::SystemTime,
    pub modified_by: String,
    pub modified_on: std::time::SystemTime,
}

impl Default for BusinessArea {
    fn default() -> Self {
        BusinessArea {
            business_area_pk: Uuid::new_v4().to_string(),
            b_area: "".to_string(),
            description: "".to_string(),
            request: "".to_string(),
            short_desc: "".to_string(),
            created_by: "".to_string(),
            created_on: std::time::SystemTime::now(),
            modified_by: "".to_string(),
            modified_on: std::time::SystemTime::now(),
        }
    }
}

impl BusinessArea {
    pub fn new(
        b_area: String,
        description: String,
        request: String,
        short_desc: String,
        created_by: String,
    ) -> Self {
        let mut new_bus_area = BusinessArea::default();

        new_bus_area.b_area = b_area.to_string();
        new_bus_area.description = description.to_string();
        new_bus_area.request = request.to_string();
        new_bus_area.short_desc = short_desc.to_string();
        new_bus_area.created_by = created_by.to_string();

        new_bus_area
    }

    pub async fn create_table(service: &Service) {

        service.client
        .execute(
            "CREATE TABLE IF NOT EXISTS business_area (
                id SERIAL PRIMARY KEY,
                business_area_pk VARCHAR NOT NULL,
                b_area VARCHAR NOT NULL,
                description VARCHAR NOT NULL,
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
           "INSERT INTO business_area (
            business_area_pk,
            b_area,
            description,
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
                $9
            )",
            &[
                &self.business_area_pk,
                &self.b_area,
                &self.description,
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
        println!("Business Area Details:");
    for row in &service.client
        .query("SELECT * FROM business_area", &[])
        .await
        .unwrap()
    {
        let id: i64 = row.get(0);
        let business_area_pk: &str = row.get(1);
        let b_area: &str = row.get(2);
        let description: &str = row.get(3);
        let request: &str = row.get(4);
        let short_desc: &str = row.get(5);
        let created_by: &str = row.get(6);
        let created_on: std::time::SystemTime = row.get(7);
        let modified_by: &str = row.get(8);
        let modified_on: std::time::SystemTime = row.get(9);
        println!("{} {} {} {} {} {} {} {:?} {} {:?}", 
            id,
            business_area_pk,
            b_area,
            description,
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