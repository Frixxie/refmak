use reqwest::get;
use serde_json::Value;
use std::fmt;
use std::io;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "REFerence MAKer",
    about = "Search after bibtex references using ISBN"
)]
struct Opt {
    ///ISBN
    #[structopt(short, long, default_value = "")]
    isbn: String,
}

struct Reference {
    name: String,
    authors: String,
    pages: String,
    title: String,
    publisher: String,
    year: String,
    isbn: String,
}

impl Reference {
    fn new(
        name: String,
        authors: String,
        pages: String,
        title: String,
        publisher: String,
        year: String,
        isbn: String,
    ) -> Reference {
        Reference {
            name,
            authors,
            pages,
            title,
            publisher,
            year,
            isbn,
        }
    }
}

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "@book{{{},\n  author = {{{}}},\n  title = {{{}}},\n  pages = {{{}}},\n  publisher = {{{}}},\n  year = {{{}}},\n  isbn = {{{}}}\n}}",
            self.name, self.authors, self.title, self.pages, self.publisher, self.year, self.isbn
        )
    }
}

/// Gets the author from openlibrary.org/author/...
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

async fn openlibrary(isbn: &str) -> Reference {
    let res = get_by_isbn("https://openlibrary.org/isbn/", isbn).await;
    let authors_links = get_authors_links(res["authors"].as_array().unwrap());
    let mut authors: Vec<String> = Vec::new();
    for author_link in authors_links {
        let res = get_author("https://openlibrary.org", &author_link).await;
        authors.push(res["name"].to_string());
    }

    let tmp = authors.join(" and ").replace('"', "");

    let title = res["title"].to_string().replace('"', "");

    let publishers: String = res["publishers"]
        .as_array()
        .unwrap()
        .into_iter()
        .map(|publisher| publisher.to_string())
        .collect::<Vec<String>>()
        .join(" and ")
        .replace('"', "");

    let mut date = res["created"]["value"].to_string();
    date.remove(0);
    date.truncate(4);

    let pages = res["number_of_pages"].to_string().replace('"', "");

    Reference::new(
        title.to_lowercase().replace(" ", ""),
        tmp,
        pages,
        title,
        publishers,
        date,
        isbn.to_string(),
    )
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let opt = Opt::from_args();
    match opt.isbn.as_str() {
        "" => {
            panic!("ISBN is missing use -i <ISBN>")
        }
        _ => {
            let reference = openlibrary(&opt.isbn).await;
            println!("{}", reference);
        }
    }
    Ok(())
}
