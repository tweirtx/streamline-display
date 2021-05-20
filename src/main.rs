use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::thread;
mod dns;
use serde_derive::Deserialize;
use std::thread::sleep;
use std::time::Duration;
use warp::Filter;

#[tokio::main]
async fn main() {
    let dnsargs = dns::Opt {
        multicast_group: "239.255.70.77".parse().unwrap(),
        host: "0.0.0.0".parse().unwrap(),
        port: 50765,
        command: dns::Command::Broadcast {
            name: Option::from("streamline-display".to_string()),
        },
    };
    thread::spawn(move || {
        println!("starting DNS");
        dns::run(dnsargs)
    });
    tokio::spawn(async move {
        println!("starting web server");
        serve().await;
    });
    println!("after thread spawn");
    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .headless(false)
            .build()
            .expect("Could not launch!"),
    );
    match browser {
        Ok(b) => {
            let taba = b.wait_for_initial_tab();
            match taba {
                Ok(tab) => {
                    tab.navigate_to("http://localhost:3030/")
                        .expect("Failed to navigate");
                    tab.wait_until_navigated().expect("Could not navigate");
                    tab.find_element("button")
                        .expect("AA")
                        .click()
                        .expect("AAAAAA");
                    loop {
                        sleep(Duration::new(1000, 0));
                    }
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
async fn serve() {
    let waiting = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("./waiting.html"));

    // TODO: route with a POST request with Scorekeeper AD URL, username, and password
    let navigate = warp::path!("navigate")
        .and(warp::post())
        .and(json_body())
        .map(|data: Loginfo| sk_login(data));

    let routes = waiting.or(navigate);
    println!("routes constructed");

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
    println!("served");
}

// TODO: Function to handle logging into Scorekeeper
fn sk_login(input: Loginfo) -> &'static str {
    println!("{}", input.url);
    return "aaaaaaaaaa";
}

fn json_body() -> impl Filter<Extract = (Loginfo,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[derive(Deserialize, Debug)]
struct Loginfo {
    url: Box<str>,
    username: Box<str>,
    password: Box<str>,
}
