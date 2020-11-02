use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::thread;
mod dns;

fn main() {
    let dnsargs = dns::Opt {
        multicast_group: "239.255.70.77".parse().unwrap(),
        host: "0.0.0.0".parse().unwrap(),
        port: 50765,
        command: dns::Command::Find { name: "streamline-control".parse().unwrap() }
    };
    thread::spawn(move || {
        dns::run(dnsargs)
    });
    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .headless(false)
            .build()
            .expect("Could not launch!")
    );
    match browser {
        Ok(b) => {
            let taba = b.wait_for_initial_tab();
            match taba {
                Ok(tab) => {
                    tab.navigate_to("https://tweirtx.github.io/streamline-display/waiting").expect("Failed to navigate");
                    tab.wait_until_navigated().expect("Could not navigate");
                    tab.find_element("button").expect("AA").click().expect("AAAAAA");
                    println!("Waited");
                    loop {
                        println!("Stay open");
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
