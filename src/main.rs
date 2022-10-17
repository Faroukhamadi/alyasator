use requestty::{Answer, ListItem, Question};

fn main() {
    let shell_select = Question::select("shell")
        .message("What is your default shell?")
        .choices(vec!["Bash", "Zsh", "Fish"])
        .build();

    let is_permanent_confirm = Question::confirm("permanent")
        .message("Do you want the alias to be permanent?")
        .build();

    let answers = requestty::prompt(vec![shell_select, is_permanent_confirm]).unwrap();

    for (_, answer) in answers {
        match answer {
            Answer::String(s) => println!("{}", s),
            Answer::Bool(l) => println!("{}", l),
            Answer::ListItem(l) => println!("{}", l.text),

            _ => (),
        }
    }
}
