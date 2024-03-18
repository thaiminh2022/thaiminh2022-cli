use std::{fmt::Display, process::Command};

use inquire::{Select, Text};

#[derive(Debug, PartialEq)]
enum QuickWebsiteOptions {
    Google,
    Lms,
    Youtube,
    Github,
    TuyenSinh247,
    Custom,
}

impl Display for QuickWebsiteOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl QuickWebsiteOptions {
    fn get_link(&self) -> &str {
        match self {
            QuickWebsiteOptions::Google => "www.google.com",
            QuickWebsiteOptions::Github => "www.github.com",
            QuickWebsiteOptions::Youtube => "www.youtube.com",
            QuickWebsiteOptions::TuyenSinh247 => "on.tuyensinh247.com",
            QuickWebsiteOptions::Lms => "lms.hcm.edu.vn",
            QuickWebsiteOptions::Custom => "",
        }
    }
}

pub fn quick_website() {
    let options = vec![
        QuickWebsiteOptions::Google,
        QuickWebsiteOptions::Youtube,
        QuickWebsiteOptions::Github,
        QuickWebsiteOptions::TuyenSinh247,
        QuickWebsiteOptions::Lms,
        QuickWebsiteOptions::Custom,
    ];
    if let Ok(selection) = Select::new("What website you wanna be on?", options)
        .with_help_message(
            r"You can use: https://www.website.com or www.website.com or website.com",
        )
        .prompt()
    {
        if selection == QuickWebsiteOptions::Custom {
            if let Ok(text_option) = Text::new("What's the website:").prompt() {
                if let Err(e) = open_website(&text_option) {
                    println!("{}", e);
                }
            }
        } else {
            open_website(selection.get_link()).expect("What the hell? This is not possible");
        }
    }
}

pub fn open_website(link: &str) -> Result<(), &str> {
    let mut new_link = String::new();
    if !link.starts_with("https://") && !link.starts_with("http://") {
        new_link = format!("https://{}", link);
    }

    let result = Command::new("cmd").args(["/C", "start", &new_link]).status();

    if result.is_err() {
        Err("Cannot parse website")
    } else {
        Ok(())
    }
}
