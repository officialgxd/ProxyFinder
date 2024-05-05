use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use futures::future::join_all;
use reqwest::Client;
use scraper::{Html, Selector};
use teloxide::requests::Requester;
use teloxide::Bot;
use teloxide_core::types::*;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[tokio::main]
async fn main() {

let bot = Bot::new("7015908466:AAGQ74yCkuF_I8_zlrI308Cyhby2ajTLup8");
let _bot_task = tokio::spawn(async move {
    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if let Some(text) = msg.text() {
            // Check if the message is the /new command
            if text.starts_with("/new") {
                proxydata().await;
                bot.send_message(msg.chat.id, "Proxy Created").await?;
              
            } else if text.starts_with("/get") {
                // Handle other messages
                let document = InputFile::file("./proxies.txt");
                bot.send_document(msg.chat.id, document).await?;
            }
        }
        Ok(())
    })
    .await;
});

let port = std::env::var("PORT").unwrap_or("4000".to_string());
let server = HttpServer::new(|| {
    App::new()
        .service(hello)
})
.bind(format!("0.0.0.0:{}", port)).unwrap()
.run();

let _ = server.await;
}

async fn proxydata() {
    let advanced_proxy_url = "https://advanced.name/freeproxy";
    let free_proxy_url = "https://free-proxy-list.net/#";

    let tasks = join_all(vec![
        scrape_proxies(advanced_proxy_url, "table#table_proxies tbody tr", "td[data-ip]", "td[data-port]"),
        scrape_proxies(free_proxy_url, "tr", "td:nth-child(1)", "td:nth-child(2)"),
    ])
    .await;

    let mut proxies = Vec::new();
    for task in tasks {
        proxies.extend(task);
    }

    save_proxies_to_file(&proxies).await;
}

async fn scrape_proxies(
    url: &str,
    tr_selector: &str,
    ip_selector: &str,
    port_selector: &str,
) -> Vec<String> {
    let client = Client::new();
    let mut proxies = Vec::new();

    if let Ok(response) = client.get(url).send().await {
        if let Ok(body) = response.text().await {
            let document = Html::parse_document(&body);
            let ip_selector = Selector::parse(ip_selector).unwrap();
            let port_selector = Selector::parse(port_selector).unwrap();

            for tr in document.select(&Selector::parse(tr_selector).unwrap()) {
                if let (Some(ip), Some(port)) = (
                    tr.select(&ip_selector).next(),
                    tr.select(&port_selector).next(),
                ) {
                    let ip_text = ip.text().collect::<String>();
                    let port_text = port.text().collect::<String>();
                    if ip_text.len() + port_text.len() > 13 {
                        let task = format!("{}:{}", ip_text, port_text);
                        proxies.push(task);
                    }
                }
            }
        }
    }

    proxies
}

async fn save_proxies_to_file(proxies: &[String]) {
    if let Ok(mut file) = tokio::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("proxies.txt")
        .await
    {
        for proxy in proxies {
            if let Err(_) = tokio::io::AsyncWriteExt::write_all(&mut file, format!("{}\n", proxy).as_bytes()).await {
                eprintln!("Error writing proxy to file.");
            }
        }
    }
}