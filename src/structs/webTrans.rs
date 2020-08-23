
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WebTrans {

    #[serde(rename = "web-translation")]
    pub wts: Option<WebTranslation>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct WebTranslation {
    pub v: Option<Vec<serde_json::Value>>
}

///
/// 网络释义
///
#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkParaphrase {
    pub key: Option<String>,
    pub trans: Option<Vec<Trans>>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Trans {
    pub summary: Option<Summary>,
    pub cls: Option<Cls>,
    pub value: Option<String>,
    pub support: Option<i32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Summary {
    pub line: Option<Vec<String>>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Cls {
    pub cl: Option<Vec<String>>
}


///
/// 短语
/// 
#[derive(Serialize, Deserialize, Debug)]
pub struct PhraseModel {
    pub key: Option<String>,
    pub trans: Option<Vec<PhraseTransItem>>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PhraseTransItem {
    pub value: Option<String>,
}