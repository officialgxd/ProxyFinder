use reqwest::Client;
use scraper::{Html, Selector};
use tokio;
extern crate base64;
use std::str;

use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() {
    proxyscapper().await;
    advancedproxy().await;
    freeproxy().await;
}

async fn proxyscapper () {
    let url = "https://api.proxyscrape.com/v3/free-proxy-list/get?request=displayproxies&proxy_format=ipport&format=text";
    let client = Client::new();
    let response = client.get(url).send().await.unwrap();
    let body = response.text().await.unwrap();

    println!("{}", body);
}


async fn advancedproxy (){
    let url = "https://advanced.name/freeproxy";
    let client = Client::new();

    let response = client.get(url).send().await.unwrap();
    let body = response.text().await.unwrap();
    let document = Html::parse_document(&body);

    let tr_selector = Selector::parse("table#table_proxies tbody tr").unwrap();
    let ip_attr = "data-ip";
    let port_attr = "data-port";

    for tr in document.select(&tr_selector) {
        if let Some(ip) = tr.select(&Selector::parse("td[data-ip]").unwrap()).next() {
            if let Some(port) = tr.select(&Selector::parse("td[data-port]").unwrap()).next() {
                let ip_text = ip.value().attr(ip_attr).unwrap_or("");
                let port_text = port.value().attr(port_attr).unwrap_or("");
                let ip_bytes = base64::decode(ip_text).unwrap();
                let port_bytes = base64::decode(port_text).unwrap();
                let ip = str::from_utf8(&ip_bytes).unwrap();
                let port = str::from_utf8(&port_bytes).unwrap();
                println!("{}:{}", ip, port);
            }
        }
    }
}


async fn freeproxy() {
    let url = "https://free-proxy-list.net/#";
    let client = Client::new();

    let response = client.get(url).send().await.unwrap();
    let body = response.text().await.unwrap();
    let document = Html::parse_document(&body);

    let ip_selector = Selector::parse("td:nth-child(1)").unwrap();
    let port_selector = Selector::parse("td:nth-child(2)").unwrap();

    let mut tasks = vec![];

    for tr in document.select(&Selector::parse("tr").unwrap()) {
        let ip_element = tr.select(&ip_selector).next();
        let port_element = tr.select(&port_selector).next();

        if let (Some(ip), Some(port)) = (ip_element, port_element) {
            let ip_text = ip.text().collect::<Vec<_>>().join("");
            let port_text = port.text().collect::<Vec<_>>().join("");
            if ip_text.len() + port_text.len() > 13 {
                let task = tokio::spawn(async move {
                    println!("{}:{}", ip_text, port_text);
                    // let mut file = OpenOptions::new().append(true).open("src/ip.txt").unwrap();
                    // let outputtext = format!("{}:{} \n", ip_text, port_text);
                    // file.write_all(outputtext.as_bytes()).unwrap();
                  
                });
                tasks.push(task);
            }
        }
    }

    for task in tasks {
        task.await.unwrap();
    }
}