use crate::tokenizer::Token;
use serde::Serialize;

#[derive(Serialize, Debug, Eq, PartialEq)]
pub enum ASTNode {
    Text(String),
    Bold(Vec<ASTNode>),
    Italic(Vec<ASTNode>),
    Strikethrough(Vec<ASTNode>),
    Spoiler(Vec<ASTNode>),
}

macro_rules! md_basic {
    ($token_peekable: expr, $token_type: path, $node_type: path) => {{
        let mut sub_tokens = $token_peekable.take_while(|token| **token != $token_type);
        let children = parser(&mut sub_tokens);
        $node_type(children)
    }};
}

pub fn parser(token_peekable: &mut dyn Iterator<Item = &Token>) -> Vec<ASTNode> {
    let mut nodes = vec![];

    while let Some(token) = token_peekable.next() {
        let node = match token {
            Token::Bold => md_basic!(token_peekable, Token::Bold, ASTNode::Bold),
            Token::Italic => md_basic!(token_peekable, Token::Italic, ASTNode::Italic),
            Token::Strikethrough => md_basic!(
                token_peekable,
                Token::Strikethrough,
                ASTNode::Strikethrough
            ),
            Token::Spoiler => md_basic!(
                token_peekable,
                Token::Spoiler,
                ASTNode::Spoiler
            ),
            Token::Text(text) => ASTNode::Text(text.clone()),
            _ => todo!(),
        };

        nodes.push(node);
    }

    nodes
}

#[cfg(test)]
mod tests {
    use crate::parser::{parser, ASTNode};
    use crate::tokenizer::Token;

    #[test]
    fn parse_normal_text() {
        let text = "Hi I am a piece of text".to_string();
        let tokens = vec![Token::Text(text.clone())];

        let ast = parser(&mut tokens.iter().peekable());

        assert_eq!(ast, vec![ASTNode::Text(text)]);
    }

    #[test]
    fn parse_italic_text() {
        let text = "Hi I am an italic piece of text".to_string();
        let tokens = vec![Token::Italic, Token::Text(text.clone()), Token::Italic];

        let ast = parser(&mut tokens.iter().peekable());

        assert_eq!(
            ast,
            vec![ASTNode::Italic(vec![ASTNode::Text(text)])]
        );
    }

    #[test]
    fn parse_bold_text() {
        let text = "Hi I am a bold piece of text".to_string();
        let tokens = vec![Token::Bold, Token::Text(text.clone()), Token::Bold];

        let ast = parser(&mut tokens.iter().peekable());

        assert_eq!(
            ast,
            vec![ASTNode::Bold(vec![ASTNode::Text(text)])]
        );
    }

    #[test]
    fn parse_mix() {
        let tokens = vec![
            Token::Text("Hi I am a ".to_string()),
            Token::Bold,
            Token::Text("bold ".to_string()),
            Token::Strikethrough,
            Token::Text("or".to_string()),
            Token::Strikethrough,
            Token::Text(" and ".to_string()),
            Token::Italic,
            Token::Text("italic".to_string()),
            Token::Italic,
            Token::Bold,
            Token::Text(" piece of text".to_string()),
        ];

        let ast = parser(&mut tokens.iter().peekable());

        assert_eq!(
            ast,
            vec![
                ASTNode::Text("Hi I am a ".to_string()),
                ASTNode::Bold(vec![
                    ASTNode::Text("bold ".to_string()),
                    ASTNode::Strikethrough(vec![ASTNode::Text("or".to_string())]),
                    ASTNode::Text(" and ".to_string()),
                    ASTNode::Italic(vec![ASTNode::Text("italic".to_string())])
                ]),
                ASTNode::Text(" piece of text".to_string())
            ]
        );
    }

    #[test]
    fn parse_strikethrough_text() {
        let text = "Hi I am a strikethrough piece of text".to_string();
        let tokens = vec![
            Token::Strikethrough,
            Token::Text(text.clone()),
            Token::Strikethrough,
        ];

        let ast = parser(&mut tokens.iter().peekable());

        assert_eq!(
            ast,
            vec![ASTNode::Strikethrough(vec![ASTNode::Text(
                text
            )])]
        );
    }
}
