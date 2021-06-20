# Nocion
A Notion client for pages and databases handling.
> Note: Nocion is still a work in progress and the API might change a lot during its development.

## Installation

Nocion requires Rust 1.53 to run properly.

Make sure to export your NOTION_KEY as environment variable. Check [Notion's API reference](https://developers.notion.com/docs/getting-started) for more info.

Afterwards clone the repo by running:

```sh
git clone https://github.com/bluretrece/nocion.git
cd nocion
```
As an example, and to have an idea of the API, consider the following code to retrieve a page from our workspace:
```rust
async fn get_page_works() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let page_id = PAGE_ID;
    let secret = get_secret();
    let notion_cli = Notion::new(secret);
    let res = notion_cli.get_page(page_id).await?;
    let status = res.status();
    let res_json: serde_json::Value = serde_json::from_str(&res_body)?;
    println!("Response json: {:?}", serde_json::to_string_pretty(&res_json));
    Ok(())
}
```
See the [tests](https://github.com/bluretrece/nocion/blob/main/tests/notion_test.rs) folder for more examples

## To do
- [x] Lists databases.
- [x] Retrieve database.
- [ ] Query Database.
- [x] Retrieve a page.
- [ ] Create a page.
- [ ] Update a page.
- [ ] Prettyprint JSON in tables.

## Future design decisions
To make the code more ergonomic, it would be practical to create pages with something like:
```rust
Page::parent(page_id).with_properties("").children_paragraph("Something something something...");
```
Besides that, make the code less cumbersome and isolate all of the abstractions for a future command line interface.
