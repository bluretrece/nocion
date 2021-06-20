pub use dotenv::dotenv;
use reqwest::{Client, Error, Response};
pub use serde_json;
pub mod utils;
pub mod base;
use utils::*;
use base::{API_DATABASE_URL, API_PAGE_URL};
use std::collections::HashMap;

// Decide whether to support (for now) pages or databases.
//
// TODO
// - Prettyprint JSON in tables -if possible-.
// Databases:
// [X] Lists databases
// [X] Retrieve database
// [] Query Database
//
//
// Pages:
// [X] Retrieve a page
// [] Create a page
// [] Update a page

// Few notes:
// Each item in a Notion database is a child page.
// We can add an item to a database by creating
// a new page while setting its parent to the database.
// Let's send an HTTP request to the create a page endpoint
// to add a new item.


#[derive(Debug)]
pub struct Notion {
    pub secret: String,
    pub bearer: String,
}

/// Should be something like: 
/// Page::parent(page_id).with_properties("").children_paragraph("Something something something...");
///
/// API idea:
// pub struct Page {
//     parent: &'static str,
//     properties: HashMap<String, String>,
//     children: Block
// }

// pub struct Block {
//     of: Paragraph // type,

// }

// pub struct Paragraph {
//     text: String,
//     children: Vec<Block>
// }


impl Notion {
    pub fn new(secret: String) -> Self {
        Self {
            secret: get_secret(),
            bearer: get_bearer(&secret),
        }
    }

    /// Retrieves a database from our Workspace.
    pub async fn get_database(
        &self,
        database_id: &'static str,
    ) -> Result<Response, Error> {
        let client = Client::new();
        let build_url = "https://api.notion.com/v1/databases/".to_owned() + &database_id.to_owned();

        let res = client
            .get(build_url)
            .headers(set_get_headers(self.bearer.clone()).await)
            .send()
            .await?;

        Ok(res)
    }

    /// Retrieves all active databases from our Workspace.
    pub async fn get_databases(
        &self,
    ) -> Result<Response, Error> {

        let client = Client::new();
        let res = client
            .get(API_DATABASE_URL)
            .headers(set_get_headers(self.bearer.clone()).await)
            .send()
            .await?;

        Ok(res)
    }

    pub async fn get_page(&self,
        page_id: &'static str
        ) -> Result<Response, Error> {

        let build_url = "https://api.notion.com/v1/pages/".to_owned() + &page_id.to_owned();
        let client = Client::new();
        let result = client
            .get(build_url)
            .headers(set_get_headers(self.bearer.clone()).await)
            .send()
            .await?;

        Ok(result)
    }
    
    // FIXME: body can not be hardcoded.
    //
    // TODO: Pages can be created with child blocks using the create a page endpoint. 
    // This endpoint supports creating a page within another page, or creating a page within a database.
    pub async fn post_page(
        &self,
        body: &'static str,
    ) -> Result<Response, Error> {
        let client = Client::new();
        let client_response = client
            .post(API_PAGE_URL)
            .headers(set_headers(self.bearer.clone()).await)
            .body(body)
            .send()
            .await?;

        Ok(client_response)
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let res_json: serde_json::Value = serde_json::from_str(&res_body)?;
    // println!(
    //     "Response json: {:?}",
    //     serde_json::to_string_pretty(&res_json)
    // );
    Ok(())
}
