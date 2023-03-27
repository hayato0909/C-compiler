#[allow(non_camel_case_types)]
pub enum TokenKind {
    TK_RESERVED,
    TK_IDENT,
    TK_NUM,
    TK_EOF,
    TK_RETURN,
    TK_IF,
    TK_ELSE,
    TK_WHILE,
    TK_FOR,
}

pub struct Token {
    pub kind: TokenKind, // トークンの種類
    val: Option<i32>, // 値
    pub s: String, // トークン
}

pub struct Tokens {
    tokens: Vec<Token>,
    idx: usize,
}

impl Tokens {
    pub fn new() -> Self {
        Tokens {tokens : Vec::<Token>::new(), idx : 0}
    }

    pub fn get_token(&self) -> &Token {
        &self.tokens[self.idx]
    }

    pub fn is_eof(&self) -> bool {
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
        let mut var: String = "".to_string();
        let mut var_flag: bool = false;
        let mut skip: bool = false;
        for i in 0..v.len() {
            if skip { 
                skip = false;
                continue; 
            }
            let c = v[i];

            if num_flag && '0' <= c && c <= '9' {
                num = num * 10 + (c as i32 - '0' as i32);
                continue;
            } else if num_flag {
                let num_token = Token{kind:TokenKind::TK_NUM, val:Some(num), s:num.to_string()};
                self.add_token(num_token);
                num = 0;
                num_flag = false;
            }

            if var_flag && 'a' <= c && c <= 'z' {
                var += &c.to_string();
                continue;
            } else if var_flag {
                let token;
                // キーワードと一致するか判定
                match &*var {  // Stringから&strにすることでmatchに対応させる
                    "return" => { token = Token{kind:TokenKind::TK_RETURN, val:None, s:var.clone()}; },
                    "if" => { token = Token{kind:TokenKind::TK_IF, val:None, s:var.clone()}; },
                    "else" => { token = Token{kind:TokenKind::TK_ELSE, val:None, s:var.clone()}; },
                    "while" => { token = Token{kind:TokenKind::TK_WHILE, val:None, s:var.clone()}; },
                    "for" => { token = Token{kind:TokenKind::TK_FOR, val:None, s:var.clone()}; },
                    _ => { token = Token{kind:TokenKind::TK_IDENT, val:None, s:var.clone()}; },
                }
                self.add_token(token);
                var = "".to_string();
                var_flag = false;
            }

            if c == ' ' {
                continue;
            } else if c == '+' || c == '-' || c == '*' || c == '/' || c == ')' || c == '(' || c == ';' || c == '{' || c == '}' {
                let token = Token{kind:TokenKind::TK_RESERVED, val:None, s:c.to_string()};
                self.add_token(token);
            } else if c == '<' || c == '>' {
                let mut s: String = c.to_string();
                if i + 1 < v.len() && v[i+1] == '=' {
                    s += &v[i+1].to_string();
                    skip = true;
                }
                let token = Token{kind:TokenKind::TK_RESERVED, val:None, s:s};
                self.add_token(token);
            } else if c == '=' || c == '!' {
                let mut s: String = c.to_string();
                if i + 1 < v.len() && v[i+1] == '=' {
                    s += &v[i+1].to_string();
                    skip = true;
                }
                let token = Token{kind:TokenKind::TK_RESERVED, val:None, s:s};
                self.add_token(token);
            } else if '0' <= c && c <= '9' {
                num_flag = true;
                num = c as i32 - '0' as i32;
            } else if 'a' <= c && c <= 'z' {
                var_flag = true;
                var = c.to_string();
            } else {
                panic!("予期しない文字です: {}", c);
            }
        }
        if num_flag {
            let token: Token = Token{kind:TokenKind::TK_NUM, val:Some(num), s:num.to_string()};
            self.add_token(token);
        }
        // 終端トークンを追加
        self.add_token(Token{kind:TokenKind::TK_EOF, val:None, s:String::from("")});
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

    // トークンを1つ読み進める
    pub fn next(&mut self) {
        self.idx += 1;
    }

    // 次のトークンが変数の時には、変数文字列を返す
    // それ以外の場合にはNoneを返す
    pub fn consume_ident(&mut self) -> Option<String> {
        let token: &Token = self.get_token();
        let var: String = token.s.clone();
        if !matches!(token.kind, TokenKind::TK_IDENT) {
            return None;
        } else {
            self.idx += 1;
            return Some(var);
        }
    }

    // 次のトークンが期待している記号以外の場合にエラーを発生
    pub fn expect(&mut self, op: String) {
        let token: &Token = self.get_token();
        if !matches!(token.kind, TokenKind::TK_RESERVED) || token.s != op { panic!("'{}'ではありません", op); }
        self.idx += 1;
    }

    // 次のトークンが数値の場合
    pub fn expect_number(&mut self) -> i32 {
        let token: &Token = self.get_token();
        if ! matches!(token.kind, TokenKind::TK_NUM) {
            panic!("数ではありません: {}", token.s);
        }
        let val: i32 = token.val.unwrap();
        self.idx += 1;
        val
    }
}

