use headless_chrome::{Browser, LaunchOptionsBuilder};
use astro_dnssd;

fn main() {
    let mut servicebrowser = astro_dnssd::browser::ServiceBrowserBuilder::new("streamline").build().expect("Error building DNS browser!");
    let result = servicebrowser.process_result();
    println!("{}", result);
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
