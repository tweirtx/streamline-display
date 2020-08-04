use headless_chrome::{Browser, LaunchOptionsBuilder};

fn main() {
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
                    tab.navigate_to("https://github.com/tweirtx").expect("Failed to navigate");
                    tab.wait_for_element("fulscbut").expect("AA").click();
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
