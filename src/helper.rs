pub fn clear_terminal() {
    print!("{}[2J", 27 as char);
}
