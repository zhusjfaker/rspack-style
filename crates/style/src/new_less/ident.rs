use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Serialize, Debug, PartialEq, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum IdentType {
  // 10px 100% 100vh
  Number(String, Option<String>),
  // + - * /
  Operator(String),
  // @abc
  Var(String),
  // $abc
  Prop(String),
  // @{abc}
  InsertVar(String),
  // "abc"
  StringConst(String),
  // solid
  Word(String),
  // #abc17fc
  Color(String),
  // !important
  KeyWord(String),
  // " " ,"\n"
  Space,
  //  ~"(min-width: 768px)" (min-width: 768px) -> Only for MediaRule
  Escaping(String),
  // ( ) [ ] 计算运算可能性
  Brackets(String),
}

impl IdentType {
  ///
  /// 反序列化
  ///
  pub fn deserializer(val: &Value) -> Self {
    serde_json::from_str(&serde_json::to_string(val).unwrap()).unwrap()
  }

  pub fn is_number(&self) -> bool {
    matches!(self, IdentType::Number(_, _))
  }

  pub fn is_space(&self) -> bool {
    matches!(self, IdentType::Space)
  }

  pub fn is_operator(&self) -> bool {
    matches!(self, IdentType::Operator(..))
  }

  pub fn is_var(&self) -> bool {
    matches!(self, IdentType::Var(..))
  }

  ///
  /// 计算取值
  ///
  pub fn calc_value(list: Vec<Self>) -> Result<Self, String> {
    let mut exper = "".to_string();
    let mut base_unit: Option<String> = None;
    for item in list {
      match item {
        IdentType::Number(value, unit) => {
          let mut convert_value = value.clone();
          if base_unit.is_none() && unit.is_some() {
            base_unit = unit;
          } else if base_unit.is_some() && unit.is_some() {
            // todo 进行单位转化
            convert_value = value;
          }
          exper += &convert_value;
        }
        IdentType::Operator(op) => {
          exper += &op;
        }
        IdentType::Space => {
          exper += " ";
        }
        IdentType::Brackets(br) => {
          exper += &br;
        }
        _ => return Err(format!("calc exper is not support -> {:#?}", item)),
      }
    }
    let mut ns = fasteval::EmptyNamespace;
    let final_value = match fasteval::ez_eval(&exper, &mut ns) {
      Ok(val) => val,
      Err(msg) => {
        return Err(format!("{} \n exper -> {}", msg, exper));
      }
    };
    Ok(Self::Number(final_value.to_string(), base_unit))
  }
}
