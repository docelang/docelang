use crate::lexer::{Token, TokenKind};
use crate::ast::{AstNode, TypeExpr};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or_else(|| self.tokens.last().unwrap())
    }

    fn next(&mut self) -> &Token {
        let tok = self.peek();
        self.pos += 1;
        tok
    }

    fn expect(&mut self, kind: TokenKind) -> &Token {
        let token = self.next();
        if token.kind != kind {
            panic!("Expected {:?}, got {:?}", kind, token.kind);
        }
        token
    }

    pub fn parse_expr(&mut self) -> AstNode {
        self.parse_binary_expr(0)
    }

    fn parse_binary_expr(&mut self, min_prec: u8) -> AstNode {
        let mut left = self.parse_closure_or_expr();

        loop {
            let op_token = self.peek();
            let (op_prec, op_fn) = match op_token.kind {
                TokenKind::Plus => (10, Some("+")),
                TokenKind::Pipe => (5, Some("|")),
                _ => (0, None),
            };

            if op_prec < min_prec || op_fn.is_none() {
                break;
            }

            self.next(); // consume operator
            let right = self.parse_binary_expr(op_prec + 1);
            left = AstNode::BinaryOp {
                op: op_fn.unwrap().to_string(),
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_closure_or_expr(&mut self) -> AstNode {
        // Closure detection: only valid if `|` is at the beginning
        if self.peek().kind == TokenKind::Pipe {
            self.parse_closure()
        } else {
            self.parse_primary_expr()
        }
    }

    fn parse_primary_expr(&mut self) -> AstNode {
        let token = self.next();
        match &token.kind {
            TokenKind::Ident => AstNode::Var(token.text.clone()),
            TokenKind::Number => AstNode::Number(token.text.parse().unwrap()),
            TokenKind::LParen => {
                let expr = self.parse_expr();
                self.expect(TokenKind::RParen);
                expr
            }
            _ => panic!("Unexpected token in primary expression: {:?}", token),
        }
    }

    fn parse_closure(&mut self) -> AstNode {
        self.expect(TokenKind::Pipe);

        let mut params = Vec::new();

        while self.peek().kind != TokenKind::Pipe {
            let name = self.expect(TokenKind::Ident).text.clone();
            let ty = if self.peek().kind == TokenKind::Colon {
                self.next(); // consume ':'
                let ty_name = self.expect(TokenKind::Ident).text.clone();
                Some(TypeExpr::Named(ty_name))
            } else {
                None
            };

            params.push((name, ty));

            if self.peek().kind == TokenKind::Comma {
                self.next(); // consume ','
            } else {
                break;
            }
        }

        self.expect(TokenKind::Pipe);

        let return_type = if self.peek().kind == TokenKind::Arrow {
            self.next(); // consume '->'
            let ty_name = self.expect(TokenKind::Ident).text.clone();
            Some(TypeExpr::Named(ty_name))
        } else {
            None
        };

        self.expect(TokenKind::LBrace);
        let body_expr = self.parse_expr();
        self.expect(TokenKind::RBrace);

        AstNode::Closure {
            params,
            return_type,
            body: Box::new(body_expr),
        }
    }
}
