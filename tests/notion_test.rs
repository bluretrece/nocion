use notion_rs::*;
use serde_json;
use utils::get_secret;

#[tokio::test]
async fn get_databases_works() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let secret = get_secret();
    let notion_cli = Notion::new(secret);
    let res = notion_cli.get_databases().await?;
    let status = res.status();

    assert_eq!(status.as_str(), "200");

    Ok(())
}

#[tokio::test]
async fn post_page_works() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let secret = get_secret();
    let notion_cli = Notion::new(secret);
    let database_id = "b456e36be1e64e419f3b75ec9639ecfc".to_string();
    let body = Body::new(database_id);

    // Thes ought to be serializable body struct
    let body = r#"{
    "parent": { "database_id": "b456e36be1e64e419f3b75ec9639ecfc" },
    "properties": {
      "title": {
        "title": [
          {
            "text": {
              "content": "Testing post request"
            }
          }
        ]
      }
    }
  }"#;

    // Receives a JSON body
    let res = notion_cli.post_page(body).await?;
    let status = res.status();
    println!("{}", res.text().await?);

    assert_eq!(status.as_str(), "200");

    Ok(())
}

#[tokio::test]
async fn get_page_works() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let page_id = "608da87e268d471dac837a3101031b3f";
    let secret = get_secret();
    let notion_cli = Notion::new(secret);
    let res = notion_cli.get_page(page_id).await?;
    let status = res.status();

    assert_eq!(status.as_str(), "200");

    Ok(())
}

#[tokio::test]
async fn get_database_works() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_id = "5cc7665f7b524388b7ad195d2bb4f110";
    let secret = get_secret();
    let notion_cli = Notion::new(secret);
    let res = notion_cli.get_database(database_id).await?;
    let status = res.status();

    assert_eq!(status.as_str(), "200");

    Ok(())
}
