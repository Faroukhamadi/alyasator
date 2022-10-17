use std::{fs, io::Error};

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

/* NOTES: - Bash: alias ll='ls -a'
          - Zsh: alias ll=""
          - Fish: alias p = "pnpm"
*/

fn path(config: &Config) -> Result<&'static str, Error> {
    match config.shell.to_lowercase().as_str() {
        "fish" => Ok("/home/farouk/.config/fish/config.fish"),
        "bash" => Ok("/home/farouk/.bashrc"),
        "zsh" => Ok("/home/farouk/.zshrc"),
        _ => return Err(Error::new(std::io::ErrorKind::Other, "Unknown shell")),
    }
}

pub fn append_to_rc(config: Config) -> Result<(), Error> {
    let p: &str;
    match path(&config) {
        Ok(s) => {
            p = s;
        }
        Err(e) => return Err(e),
    }
    match fs::read_to_string(p) {
        Ok(mut file_content) => {
            file_content.push_str(&format!("alias {}=\"{}\"", config.alias, config.original));
            println!("new file contents: {}", file_content);
            return Ok(());
        }
        Err(e) => return Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_correct_path_bash() {
        let config = Config {
            shell: "bash".to_string(),
            is_permanent: true,
            alias: "true".to_string(),
            original: "true".to_string(),
        };

        assert_eq!(path(&config).unwrap(), "/home/farouk/.bashrc");
    }

    #[test]
    fn returns_correct_path_zsh() {
        let config = Config {
            shell: "zsh".to_string(),
            is_permanent: true,
            alias: "true".to_string(),
            original: "true".to_string(),
        };

        assert_eq!(path(&config).unwrap(), "/home/farouk/.zshrc");
    }

    #[test]
    fn returns_correct_path_fish() {
        let config = Config {
            shell: "fish".to_string(),
            is_permanent: true,
            alias: "true".to_string(),
            original: "true".to_string(),
        };

        assert_eq!(
            path(&config).unwrap(),
            "/home/farouk/.config/fish/config.fish"
        );
    }

    #[test]
    fn append_to_bash() {
        unimplemented!();
    }

    fn append_to_zsh() {
        unimplemented!();
    }

    fn append_to_fish() {
        unimplemented!();
    }

    fn set_as_temp() {
        unimplemented!();
    }
}
