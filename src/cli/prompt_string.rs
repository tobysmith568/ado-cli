use dialoguer::Input;

pub fn prompt_for_string(question: &str) -> String {
    println!("{}", question);

    Input::<String>::new().interact_text().unwrap()
}
