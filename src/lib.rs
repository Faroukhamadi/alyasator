use requestty::{Answers, ErrorKind, Question};

pub struct Config {
    pub shell: String,
    pub is_permanent: bool,
    pub alias: String,
    pub original: String,
}

fn answers() -> Result<Answers, ErrorKind> {
    let shell_select = Question::select("shell")
        .message("What is your default shell?")
        .choices(vec!["Bash", "Zsh", "Fish"])
        .build();

    let is_permanent_confirm = Question::confirm("permanent")
        .message("Do you want the alias to be permanent?")
        .build();

    let alias_input = Question::input("alias")
        .message("What is the alias you want to assign?")
        .build();

    let original_input = Question::input("original")
        .message("What is the original command?")
        .build();

    let answers = requestty::prompt(vec![
        shell_select,
        is_permanent_confirm,
        alias_input,
        original_input,
    ]);
    answers
}

impl Config {
    pub fn build() -> Result<Config, &'static str> {
        match answers() {
            Ok(answers) => {
                let shell_answer = answers.get("shell").unwrap().clone();
                let is_permanent_answer = answers.get("permanent").unwrap().clone();
                let alias_answer = answers.get("alias").unwrap().clone();
                let original_answer = answers.get("original").unwrap().clone();

                Ok(Config {
                    shell: shell_answer.as_list_item().unwrap().to_owned().text,
                    is_permanent: is_permanent_answer.as_bool().unwrap(),
                    alias: alias_answer.as_string().unwrap().to_owned(),
                    original: original_answer.as_string().unwrap().to_owned(),
                })
            }
            Err(_) => {
                return Err("Interrupted");
            }
        }
    }
}
