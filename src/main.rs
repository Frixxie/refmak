use reqwest::get;
use serde_json::Value;
use std::io;

struct Reference {
    name : String,
    authors : String,
    publisher : String,
    year: u32,
    address: String,
    isbn: u32,
}

/// Gets the author from openlibrary.org/author...
async fn get_author(url: &str, author: &str) -> Value {
    let new_url = format!("{}{}.json", url, author);
    let res = get(new_url).await.unwrap().text().await.unwrap();
    serde_json::from_str(res.as_str()).unwrap()
}

async fn get_by_isbn(url: &str, isbn: &str) -> Value {
    let new_url = format!("{}{}.json", url, isbn);
    let res = get(new_url).await.unwrap().text().await.unwrap();
    serde_json::from_str(res.as_str()).unwrap()
}

fn get_authors_links(authors: &[Value]) -> Vec<String> {
    authors
        .into_iter()
        .map(|val| val["key"].as_str().unwrap().to_string())
        .collect()
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let res = get_by_isbn("https://openlibrary.org/isbn/", "9781292025407").await;
    println!("{:?}", res);
    // println!("{:?}", author);
    let authors_links = get_authors_links(res["authors"].as_array().unwrap());
    let mut authors: Vec<String> = Vec::new();
    for author_link in authors_links {
        let res = get_author("https://openlibrary.org", &author_link).await;
        authors.push(res["name"].to_string());
    }
    let tmp = authors.join(" and ");
    println!("{}", tmp);
    print!("@book{{}}");
    Ok(())
}
