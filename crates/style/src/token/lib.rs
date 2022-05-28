pub struct Token(String);

///
/// 词根处理
///
impl Token {
  pub fn new(value: String) -> Self {
    Token(value)
  }

  ///
  /// 是否是 词根
  ///
  pub fn is_token(char: Option<&char>) -> bool {
    if let Some(cc) = char {
      matches!(
        cc,
        '.'
          | '|'
          | '!'
          | '?'
          | '^'
          | '#'
          | '~'
          | ' '
          | '\n'
          | '\r'
          | ':'
          | '%'
          | '$'
          | '&'
          | '['
          | ']'
          | '@'
          | '/'
          | '+'
          | '>'
          | '<'
          | '}'
          | '{'
          | '*'
          | '-'
          | '='
          | '`'
          | '('
          | ')'
          | ';'
          | '\''
          | '"'
          | '\\'
          | ','
      )
    } else {
      false
    }
  }

  ///
  /// 是否是空白字符串
  ///
  pub fn is_space_token(char: Option<&char>) -> bool {
    if let Some(cc) = char {
      *cc == ' ' || *cc == '\n' || *cc == '\r'
    } else {
      false
    }
  }
}
