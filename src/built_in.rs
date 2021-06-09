/// Executes the "say" command
pub(crate) fn say(text: String) -> bool {

    let mut skip_next = false;
    for char in 0..text.len() {
        if skip_next { continue; }

        if text.chars().nth(char) == Some('\\') && text.chars().nth(char + 1) == Some('n') {
            println!();
            skip_next = true;
        } else {
            print!("{}", text.chars().nth(char).unwrap());
        }
    }

    true

}