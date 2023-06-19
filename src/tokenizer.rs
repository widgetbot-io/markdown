use std::iter::Peekable;
use std::str::Chars;

#[derive(Eq, PartialEq, Debug)]
pub enum Token {
    Bold,
    Italic,
    BoldOrItalic,
    Strikethrough,
    Spoiler,
    Text(String),
}

fn add_to_text(tokens: &mut Vec<Token>, ch: char) {
    if let Some(Token::Text(str)) = tokens.last_mut() {
        str.push(ch);
    } else {
        tokens.push(Token::Text(String::from(ch)));
    }
}

pub fn tokenize(char_peekable: &mut Peekable<Chars>) -> Vec<Token> {
    let mut tokens = vec![];

    while let Some(char) = char_peekable.next() {
        // todo: escaping

        let token = match char {
            '*' => {
                if char_peekable.next_if_eq(&'*').is_some() {
                    if char_peekable.next_if_eq(&'*').is_some() {
                        Some(Token::BoldOrItalic)
                    } else {
                        Some(Token::Bold)
                    }
                } else {
                    Some(Token::Italic)
                }
            }
            '~' => {
                if char_peekable.next_if_eq(&'~').is_some() {
                    Some(Token::Strikethrough)
                } else {
                    add_to_text(&mut tokens, '~');
                    None
                }
            }
            '|' => {
                if char_peekable.next_if_eq(&'|').is_some() {
                    Some(Token::Spoiler)
                } else {
                    add_to_text(&mut tokens, '|');
                    None
                }
            }
            _ => {
                add_to_text(&mut tokens, char);
                None
            }
        };

        if let Some(token) = token {
            tokens.push(token)
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::{add_to_text, tokenize, Token};

    #[test]
    fn add_to_text_empty() {
        let mut tokens = vec![];

        add_to_text(&mut tokens, 'H');

        assert_eq!(tokens, vec![Token::Text(String::from("H"))]);
    }

    #[test]
    fn add_to_text_non_empty() {
        let mut tokens = vec![Token::Text(String::from("H"))];

        add_to_text(&mut tokens, 'i');

        assert_eq!(tokens, vec![Token::Text(String::from("Hi"))]);
    }

    #[test]
    fn normal_text() {
        let text = String::from("Hi I am a piece of text");
        let mut peekable = text.chars().peekable();

        let tokens = tokenize(&mut peekable);

        assert_eq!(tokens, vec![Token::Text(text)]);
    }

    #[test]
    fn bold_text() {
        let inner_text = "Hi I am a bold piece of text";
        let text = format!("**{}**", inner_text);
        let mut peekable = text.chars().peekable();

        let tokens = tokenize(&mut peekable);

        assert_eq!(
            tokens,
            vec![
                Token::Bold,
                Token::Text(inner_text.to_string()),
                Token::Bold
            ]
        );
    }

    #[test]
    fn strikethrough_text() {
        let inner_text = "Hi I am a strikethrough piece of text";
        let text = format!("~~{}~~", inner_text);
        let mut peekable = text.chars().peekable();

        let tokens = tokenize(&mut peekable);

        assert_eq!(
            tokens,
            vec![
                Token::Strikethrough,
                Token::Text(inner_text.to_string()),
                Token::Strikethrough
            ]
        );
    }

    #[test]
    fn italic_text() {
        let inner_text = "Hi I am an italic piece of text";
        let text = format!("*{}*", inner_text);
        let mut peekable = text.chars().peekable();

        let tokens = tokenize(&mut peekable);

        assert_eq!(
            tokens,
            vec![
                Token::Italic,
                Token::Text(inner_text.to_string()),
                Token::Italic
            ]
        );
    }

    #[test]
    fn italic_and_bold_text() {
        let text = "Hi I am an ***italic and bold*** piece of text";
        let mut peekable = text.chars().peekable();

        let tokens = tokenize(&mut peekable);

        assert_eq!(
            tokens,
            vec![
                Token::Text("Hi I am an ".to_string()),
                Token::BoldOrItalic,
                Token::Text("italic and bold".to_string()),
                Token::BoldOrItalic,
                Token::Text(" piece of text".to_string())
            ]
        );
    }

    #[test]
    fn spoiler_text() {
        let inner_text = "Hi I am a spoiler";
        let text = format!("||{}||", inner_text);
        let mut peekable = text.chars().peekable();

        let tokens = tokenize(&mut peekable);

        assert_eq!(
            tokens,
            vec![
                Token::Spoiler,
                Token::Text(inner_text.to_string()),
                Token::Spoiler
            ]
        );
    }

    #[test]
    fn no_spoiler_single_char_text() {
        let inner_text = "Hi I am not a spoiler";
        let text = format!("|{}|", inner_text);
        let mut peekable = text.chars().peekable();

        let tokens = tokenize(&mut peekable);

        assert_eq!(
            tokens,
            vec![
                Token::Text(text.to_string()),
            ]
        );
    }

    #[test]
    fn mixed_text() {
        let text = "Hi I am a *mixed* **piece** of text";
        let mut peekable = text.chars().peekable();

        let tokens = tokenize(&mut peekable);

        assert_eq!(
            tokens,
            vec![
                Token::Text("Hi I am a ".to_string()),
                Token::Italic,
                Token::Text("mixed".to_string()),
                Token::Italic,
                Token::Text(" ".to_string()),
                Token::Bold,
                Token::Text("piece".to_string()),
                Token::Bold,
                Token::Text(" of text".to_string())
            ]
        );
    }
}
