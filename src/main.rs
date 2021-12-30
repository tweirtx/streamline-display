use std::borrow::Borrow;
use std::ptr::null;
use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::thread;
mod dns;
use serde_derive::Deserialize;
use std::thread::sleep;
use std::time::Duration;
use std::option::Option;
use std::sync::Arc;
use warp::Filter;

static mut BROWSER_TAB: Option<Arc<headless_chrome::Tab>> = None;

#[tokio::main]
async fn main() {
    let dnsargs = dns::Opt {
        multicast_group: "239.255.70.77".parse().unwrap(),
        host: "0.0.0.0".parse().unwrap(),
        port: 50765,
        command: dns::Command::Broadcast {
            name: "streamline-display".to_string(),
        },
    };
    thread::spawn(move || {
        println!("starting DNS");
        dns::run(dnsargs)
    });
    tokio::spawn(async move {
        println!("starting web server");
        unsafe {
            serve().await;
        }
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
                Ok(tab) => unsafe {
                    BROWSER_TAB = Option::from(tab);
                    match &BROWSER_TAB {
                        Some(tabby) => {
                            &tabby.navigate_to("http://localhost:3030/")
                                .expect("Failed to navigate");
                            &tabby.wait_until_navigated().expect("Could not navigate");
                            &tabby.find_element("button")
                                .expect("AA")
                                .click()
                                .expect("AAAAAA");
                        }
                        None => {
                            println!("Something went terribly wrong");
                        }
                    }

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
async unsafe fn serve() {
    let waiting = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("./waiting.html"));

    // TODO: route with a POST request with Scorekeeper AD URL, username, and password
    let navigate = warp::path!("navigate")
        .and(warp::post())
        .and(json_body())
        .map(|data: DisplayURL| navigate(data));

    let routes = waiting.or(navigate);
    println!("routes constructed");

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
    println!("served");
}

unsafe fn navigate(input: DisplayURL) -> &'static str {
    return match &BROWSER_TAB {
        Some(t) => {
            t.navigate_to(&input.url).expect("Navigation error");
            println!("{}", input.url);
            "Navigated!"
        }
        None => {
            "Error getting tab!"
        }
    }
}

fn json_body() -> impl Filter<Extract = (DisplayURL,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[derive(Deserialize, Debug)]
struct DisplayURL {
    url: Box<str>
}
