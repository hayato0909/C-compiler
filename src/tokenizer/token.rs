pub enum TokenKind {
    TK_RESERVED,
    TK_IDENT,
    TK_NUM,
    TK_EOF,
}

pub struct Token {
    kind: TokenKind,
    val: i32,
    pub s: String,
}

pub struct Tokens {
    tokens: Vec<Token>,
    idx: usize,
}

impl Tokens {
    pub fn new() -> Self {
        Tokens {tokens : Vec::<Token>::new(), idx : 0,}
    }

    fn get_token(&self) -> &Token {
        &self.tokens[self.idx]
    }

    pub fn is_EOF(&self) -> bool {
        let token: &Token = self.get_token();
        if matches!(token.kind, TokenKind::TK_EOF) { true }
        else { false }
    }

    pub fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn tokenize(&mut self, v: Vec<char>) {
        let mut num: i32 = 0;
        let mut num_flag: bool = false;
        let mut skip: bool = false;
        for i in 0..v.len() {
            if skip { 
                skip = false;
                continue; 
            }
            let c = v[i];
            if c == ' ' {
                continue;
            } else if c == '+' || c == '-' || c == '*' || c == '/' || c == ')' || c == '(' {
                if num_flag {
                    let num_token = Token{kind:TokenKind::TK_NUM, val:num, s:num.to_string()};
                    self.add_token(num_token);
                    num = 0;
                    num_flag = false;
                }
                let token = Token{kind:TokenKind::TK_RESERVED, val:0, s:c.to_string()};
                self.add_token(token);
            } else if c == '<' || c == '>' {
                if num_flag {
                    let num_token = Token{kind:TokenKind::TK_NUM, val:num, s:num.to_string()};
                    self.add_token(num_token);
                    num = 0;
                    num_flag = false;
                }
                let mut s: String = c.to_string();
                if i + 1 < v.len() && v[i+1] == '=' {
                    s += &v[i+1].to_string();
                    skip = true;
                }
                let token = Token{kind:TokenKind::TK_RESERVED, val:0, s:s};
                self.add_token(token);
            } else if c == '=' || c == '!' {
                if num_flag {
                    let num_token = Token{kind:TokenKind::TK_NUM, val:num, s:num.to_string()};
                    self.add_token(num_token);
                    num = 0;
                    num_flag = false;
                }
                let mut s: String = c.to_string();
                if i + 1 < v.len() && v[i+1] == '=' {
                    s += &v[i+1].to_string();
                    skip = true;
                }
                let token = Token{kind:TokenKind::TK_RESERVED, val:0, s:s};
                self.add_token(token);
            } else if '0' <= c && c <= '9' {
                num_flag = true;
                num = num * 10 + (c as i32 - '0' as i32);
            } else if 'a' <= c && c <= 'z' {
                if num_flag {
                    let num_token = Token{kind:TokenKind::TK_NUM, val:num, s:num.to_string()};
                    self.add_token(num_token);
                    num = 0;
                    num_flag = false;
                }
                let token = Token{kind:TokenKind::TK_IDENT, val:0, s:c.to_string()};
                self.add_token(token);
            } else {
                panic!("予期しない文字です: {}", c);
            }
        }
        let token: Token = Token{kind:TokenKind::TK_NUM, val:num, s:num.to_string()};
        self.add_token(token);
        // 終端トークンを追加
        self.add_token(Token{kind:TokenKind::TK_EOF, val:0, s:String::from("")});
    }

    // 次のトークンが期待している記号の時には、トークンを1つ進めてtrueを返す
    // それ以外の場合にはfalseを返す
    pub fn consume(&mut self, op: String) -> bool {
        let token: &Token = self.get_token();
        if !matches!(token.kind, TokenKind::TK_RESERVED) || token.s != op {
            false
        } else { 
            self.idx += 1;
            true 
        }
    }

    // 次のトークンが変数の時には、トークンを返す
    // それ以外の場合にはNoneを返す
    pub fn consume_ident(&mut self) -> Option<&Token> {
        let token: &Token = self.get_token();
        if matches!(token.kind, TokenKind::TK_IDENT) {
            return Some(token);
        }
        return None;
    }

    // 次のトークンが期待している記号以外の場合にエラーを発生
    pub fn expect(&mut self, op: String) {
        let token: &Token = self.get_token();
        if !matches!(token.kind, TokenKind::TK_RESERVED) || token.s != op { panic!("'{}'ではありません", op); }
        self.idx += 1
    }

    // 次のトークンが数値の場合
    pub fn expect_number(&mut self) -> i32 {
        let token: &Token = self.get_token();
        if ! matches!(token.kind, TokenKind::TK_NUM) {
            panic!("数ではありません: {}", token.val);
        }
        let val: i32 = token.val;
        self.idx += 1;
        val
    }
}

