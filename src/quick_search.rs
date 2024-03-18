use std::process::Command;

use inquire::Text;

pub fn quick_search() {
    let search_term_prompt = Text::new("what do you wanna search for > ").prompt();

    if let Ok(search_term) = search_term_prompt {
        let formated_search_term = search_term.split(' ').collect::<Vec<_>>().join("+");
        let link = format!("www.google.com/search?q={}", formated_search_term);
        Command::new("cmd")
            .args(["/C", "start", &link])
            .status()
            .unwrap();
    }
}
