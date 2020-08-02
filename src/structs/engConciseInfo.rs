
///
/// 英译中API响应结果
/// 
/// 

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EngConciseInfo {
    pub pic_dict: Option<PicDict>,
    pub input: Option<String>,
    pub le: Option<String>,
    pub lang: Option<String>,
    pub ec: Option<Ec>
}



#[derive(Serialize, Deserialize, Debug)]
pub struct PicDict {
    pic: Option<Vec<Pic>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pic {
    image: Option<String>,
    host: Option<String>,
    url: Option<String>
}


pub struct Meta {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ec {
    pub exam_type: Option<Vec<String>>,
    pub word: Option<Vec<Word>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Word {
    pub usphone: Option<String>,
    pub ukphone: Option<String>,
    pub ukspeech: Option<String>,
    pub trs: Option<Vec<Trs>>,
    pub wfs: Option<Vec<Wfs>>
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
    pub i: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wfs {
    pub wf: Option<Wf>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wf {
    pub name: Option<String>,
    pub value: Option<String>
}