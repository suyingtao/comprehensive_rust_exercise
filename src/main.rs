use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use reqwest::blocking::{get, Response};
use reqwest::Url;
use scraper::{Html, Selector};
use thiserror::Error;
use std::thread;

#[derive(Error, Debug)]
enum Error {
    #[error("request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

fn extract_links(response: Response) -> Result<Vec<Url>, Error> {
    let base_url = response.url().to_owned();
    let document = response.text()?;
    let html = Html::parse_document(&document);
    let selector = Selector::parse("a").unwrap();

    let mut valid_urls = Vec::new();
    for element in html.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            match base_url.join(href) {
                Ok(url) => valid_urls.push(url),
                Err(err) => {
                    println!("On {base_url}: could not parse {href:?}: {err} (ignored)",);
                }
            }
        }
    }
    Ok(valid_urls)
}

fn check_url(url: Url, checked_urls: &Arc<Mutex<HashSet<String>>>) -> Result<Vec<Url>, Error> {
    checked_urls.lock().unwrap().insert(url.to_string());
    let s = url.to_string();
    println!("start check: {s}");
    let response = get(url.to_owned()).unwrap();
    let valid_links = extract_links(response)?;
    let valid_urls = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for link in valid_links {
        let checked_urls_clone = Arc::clone(&checked_urls.clone());
        let valid_urls = valid_urls.clone();
        let link_str = link.to_string();
        if link.domain() == url.domain() {
            println!("same domain {link_str} and {s}");
            if !checked_urls_clone.lock().unwrap().contains(&link.to_string()) {
                handles.push(thread::spawn(move || {
                    let r = check_url(link, &checked_urls_clone).unwrap();
                    valid_urls.lock().unwrap().extend(r);
                }));
            }
        } else {
            println!("not same domain {link_str} and {s}");
            valid_urls.lock().unwrap().push(link);
        };
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let v = valid_urls.lock().unwrap().clone();
    Ok(v)
}

fn main() {
    let checked_urls = Arc::new(Mutex::new(HashSet::new()));
    let start_url = Url::parse("https://www.douyin.com").unwrap();
    let valid_urls = check_url(start_url, &checked_urls).unwrap();
    for url in valid_urls {
        let s = url.as_str();
        println!("valid_url: {s}");
    }
    for url in checked_urls.lock().unwrap().iter() {
        println!("checked_url: {url}");
    }
}
