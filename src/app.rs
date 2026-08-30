use crate::api::CatFactClient;
use jstdlib::console::Console;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let client = CatFactClient::new(reqwest::Client::new());

    loop {
        let input: String = Console::input("Maximum fact length (leave blank for none): ");
        let trimmed = input.trim();
        
        let max_length = if trimmed.is_empty() {
            None
        } else {
            match trimmed.parse::<u32>() {
                Ok(num) => Some(num),
                Err(_) => {
                    Console::write_line("Please enter a valid positive number.");
                    continue;
                }
            }
        };

        let fact = client.get_fact(max_length).await?;
        
        let msg = format!("{}\n({} characters)", fact.fact, fact.length);

        Console::write_line(msg);

        if !Console::confirm("Get another fact? (y/n): ") {
            break;
        }
    }

    Ok(())
}