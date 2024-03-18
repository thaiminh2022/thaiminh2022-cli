mod helper;
mod new_term;
mod quick_search;
mod quick_website;

use helper::clear_terminal;
use inquire::Select;
use new_term::new_term;
use quick_search::quick_search;
use quick_website::quick_website;
use std::fmt::Display;
#[derive(Debug, PartialEq)]
enum SelectOption {
    Exit,
    QuickSearch,
    QuickWebsite,
    NewTerm,
}
impl Display for SelectOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() {
    loop {
        if let Some(selection) = prompt_selection() {
            match selection {
                SelectOption::QuickWebsite => quick_website(),
                SelectOption::NewTerm => new_term(),
                SelectOption::QuickSearch => quick_search(),
                SelectOption::Exit => break,
            };
        } else {
            break;
        }
        clear_terminal();
    }
}

fn prompt_selection() -> Option<SelectOption> {
    let options = vec![
        SelectOption::QuickWebsite,
        SelectOption::QuickSearch,
        SelectOption::NewTerm,
        SelectOption::Exit,
    ];

    if let Ok(selection) = Select::new("What do you wanna do today?", options).prompt() {
        Some(selection)
    } else {
        None
    }
}
