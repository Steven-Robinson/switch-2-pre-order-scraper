fn main() {
    log("Scraping for availability...");
    
    let product_url = "https://amzn.eu/d/0AxGUBX";
    
    check_for_availability(product_url);
}

fn check_for_availability(product_url: &str) {

    let response = reqwest::blocking::get(product_url).expect("Failed to send request");

    let document = scraper::Html::parse_document(&response.text().unwrap());
    let selector = scraper::Selector::parse("input#buy-now-button").unwrap();
    let buy_now_button = document.select(&selector).next();

    if let Some(_element) = buy_now_button {
        let content = format!("This is now available for purchase... {}", product_url);
        email(&content);
    }
}

fn email(content: &str) {
    let email_addr = env::var("EMAIL_ADDRESS").expect("EMAIL_ADDRESS not set");
    let gmail_app_pw = env::var("GMAIL_APP_PW").expect("GMAIL_APP_PW not set");

    let email = Message::builder()
        .from(format!("Sender <{}>", email_addr).parse().unwrap())
        .to(format!("Receiver <{}>", email_addr).parse().unwrap())
        .subject("Scraper Notification!")
        .body(String::from(content))
        .unwrap();

    let creds = Credentials::new(email_addr.to_string(), gmail_app_pw.to_string());

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}

fn log(content: &str) {
    let log_file = "/Users/stevenrobinson/Desktop/scrape_log.txt";

    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_file)
        .expect("Unable to open file");

    let cur_time = Local::now();

    writeln!(file, "{} : {}", cur_time, content).expect("Unable to write to file");
}
