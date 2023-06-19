use crate::tokenizer::Token;
use serde::Serialize;

#[derive(Serialize, Debug, Eq, PartialEq)]
pub enum ASTNodeInternal {
    Text(String),
    Bold(Vec<ASTNodeInternal>),
    Italic(Vec<ASTNodeInternal>),
    Strikethrough(Vec<ASTNodeInternal>),
}

macro_rules! md_basic {
    ($char_peekable: expr, $token_type: path, $node_type: path) => {{
        let mut sub_tokens = $char_peekable.take_while(|token| **token != $token_type);
        let children = parser(&mut sub_tokens);
        $node_type(children)
    }}
}

pub fn parser(char_peekable: &mut dyn Iterator<Item = &Token>) -> Vec<ASTNodeInternal> {
    let mut nodes = vec![];

    while let Some(token) = char_peekable.next() {
        let node = match token {
            Token::Bold => md_basic!(char_peekable, Token::Bold, ASTNodeInternal::Bold),
            Token::Italic => md_basic!(char_peekable, Token::Italic, ASTNodeInternal::Italic),
            Token::Strikethrough => md_basic!(char_peekable, Token::Strikethrough, ASTNodeInternal::Strikethrough),
            Token::Text(text) => ASTNodeInternal::Text(text.clone()),
            _ => todo!(),
        };

        nodes.push(node);
    }

    nodes
}

#[cfg(test)]
mod tests {
    use crate::parser::{ASTNodeInternal, parser};
    use crate::tokenizer::Token;

    #[test]
    fn parse_normal_text() {
        let text = "Hi I am a piece of text".to_string();
        let tokens = vec![Token::Text(text.clone())];

        let ast = parser(&mut tokens.iter().peekable());

        assert_eq!(ast, vec![ASTNodeInternal::Text(text)]);
    }

    #[test]
    fn parse_italic_text() {
        let text = "Hi I am an italic piece of text".to_string();
        let tokens = vec![Token::Italic, Token::Text(text.clone()), Token::Italic];

        let ast = parser(&mut tokens.iter().peekable());

        assert_eq!(ast, vec![ASTNodeInternal::Italic(vec![ASTNodeInternal::Text(text)])]);
    }

    #[test]
    fn parse_bold_text() {
        let text = "Hi I am a bold piece of text".to_string();
        let tokens = vec![Token::Bold, Token::Text(text.clone()), Token::Bold];

        let ast = parser(&mut tokens.iter().peekable());

        assert_eq!(ast, vec![ASTNodeInternal::Bold(vec![ASTNodeInternal::Text(text)])]);
    }

    #[test]
    fn parse_strikethrough_text() {
        let text = "Hi I am a strikethrough piece of text".to_string();
        let tokens = vec![Token::Strikethrough, Token::Text(text.clone()), Token::Strikethrough];

        let ast = parser(&mut tokens.iter().peekable());

        assert_eq!(ast, vec![ASTNodeInternal::Strikethrough(vec![ASTNodeInternal::Text(text)])]);
    }
}
