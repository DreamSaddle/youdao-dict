use serde::{Serialize, Deserialize};

// type Other = std::collections::BTreeMap<>;


#[derive(Serialize, Deserialize, Debug)]
pub struct ZhConciseInfo {
    pub input: Option<String>,
    pub le: Option<String>,
    pub lang: Option<String>,
    pub ce: Option<Ce>,

    pub fanyi: Option<FanYi>
}

///
/// 翻译内容为短语使用
/// 
#[derive(Serialize, Deserialize, Debug)]
pub struct FanYi {
    //翻译短语源
    pub input: Option<String>,
    //翻译结果
    pub tran: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ce {
    pub word: Option<Vec<Word>>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Word {
    pub trs: Option<Vec<Trs>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trs {
    pub tr: Option<Vec<Tr>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tr {
    pub l: Option<L>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct L {
    pub pos: Option<String>,

    pub i: serde_json::Value,
    
    #[serde(rename = "#tran")]
    pub tran: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct I {
    #[serde(rename = "#text")]
    pub text: Option<String>
}