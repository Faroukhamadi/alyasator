use requestty::{Answers, ErrorKind, Question};
use std::fs;
use std::io::{Error, ErrorKind as IoErrorKind};
use std::process::Command;
use std::str;

pub struct Config {
    pub shell: String,
    pub alias: String,
    pub original: String,
}

fn answers() -> Result<Answers, ErrorKind> {
    let shell_select = Question::select("shell")
        .message("What is your default shell?")
        .choices(vec!["Bash", "Zsh", "Fish"])
        .build();

    let alias_input = Question::input("alias")
        .message("What is the alias you want to assign?")
        .build();

    let original_input = Question::input("original")
        .message("What is the original command?")
        .build();

    let answers = requestty::prompt(vec![shell_select, alias_input, original_input]);
    answers
}

impl Config {
    pub fn build() -> Result<Config, &'static str> {
        match answers() {
            Ok(answers) => {
                let shell_answer = answers.get("shell").unwrap();
                let alias_answer = answers.get("alias").unwrap();
                let original_answer = answers.get("original").unwrap();

                Ok(Config {
                    shell: shell_answer.as_list_item().unwrap().to_owned().text,
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

fn exists(query: &str, contents: &str) -> bool {
    contents.lines().any(|line| line.contains(query))
}

fn path(config: &Config) -> Result<String, Error> {
    let username = Command::new("sh")
        .arg("-c")
        .arg("echo $USER")
        .output()
        .expect("failed to execute process")
        .stdout;

    let username = str::from_utf8(&username).unwrap().trim_end();

    match config.shell.to_lowercase().as_str() {
        "fish" => Ok(format!("/home/{}/.config/fish/config.fish", username)),
        "bash" => Ok(format!("/home/{}/.bashrc", username)),
        "zsh" => Ok(format!("/home/{}/.zshrc", username)),
        _ => return Err(Error::new(IoErrorKind::Other, "Unknown shell")),
    }
}

pub fn run(config: Config) -> Result<(), Error> {
    let p: String;
    match path(&config) {
        Ok(s) => {
            p = s;
        }
        Err(e) => return Err(e),
    }

    match fs::read_to_string(&p) {
        Ok(mut contents) => {
            let query = format!("alias {}=", config.alias);

            if exists(&query, &contents) {
                return Err(Error::new(IoErrorKind::Other, "Alias already exists"));
            } else {
                contents.push_str(&format!("\nalias {}=\"{}\"", config.alias, config.original));
                fs::write(p, &contents)?;

                println!("✅ Alias successfully added");
                return Ok(());
            }
        }
        Err(e) => return Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Doesn't work in CI"]
    fn returns_correct_path_bash() {
        let config = Config {
            shell: "bash".to_string(),
            alias: "true".to_string(),
            original: "true".to_string(),
        };

        assert_eq!(path(&config).unwrap(), "/home/farouk/.bashrc");
    }

    #[test]
    #[ignore = "Doesn't work in CI"]
    fn returns_correct_path_zsh() {
        let config = Config {
            shell: "zsh".to_string(),
            alias: "true".to_string(),
            original: "true".to_string(),
        };

        assert_eq!(path(&config).unwrap(), "/home/farouk/.zshrc");
    }

    #[test]
    #[ignore = "Doesn't work in CI"]
    fn returns_correct_path_fish() {
        let config = Config {
            shell: "fish".to_string(),
            alias: "true".to_string(),
            original: "true".to_string(),
        };

        assert_eq!(
            path(&config).unwrap(),
            "/home/farouk/.config/fish/config.fish"
        );
    }

    #[test]
    fn alias_exists() {
        let query = "alias c=";
        let contents = r#"\
hello this is a cool string
it contains an alias below
alias c="code""#;

        assert!(exists(query, contents));
    }

    #[test]
    fn alias_doesnt_exist() {
        let query = "alias c=";
        let contents = r"\
hello this string is not cool
it doesn't contain an alias
";
        assert!(!exists(query, contents));
    }

    #[test]
    #[ignore = "not implemented yet"]
    fn set_as_temp() {
        unimplemented!();
    }
}
