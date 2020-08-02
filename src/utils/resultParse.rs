
use regex::Regex;
use serde_json::json;

use crate::structs;

pub struct EnToZh {
    pub zhLines: Vec<String>,
    pub wfs: String
}

///
/// 英译中
/// 中文行列表获取
/// 
pub fn request_result_parse_en_to_zh(resultObj: &structs::engConciseInfo::EngConciseInfo) -> EnToZh {
    let mut zh_lines = Vec::new();
    let mut wfs_str = String::from("");
    if resultObj.ec.is_some() {
        let word: Option<&Vec<structs::engConciseInfo::Word>> = resultObj.ec.as_ref().unwrap().word.as_ref();

        if word.is_some() {
            for w in word.unwrap().iter() {
                let trs: Option<&Vec<structs::engConciseInfo::Trs>> = w.trs.as_ref();
                if trs.is_some() {
                    for trs_ in trs.unwrap().iter() {
                        let tr: Option<&Vec<structs::engConciseInfo::Tr>> = trs_.tr.as_ref();
                        if tr.is_some() {
                            for tr_ in tr.unwrap().iter() {
                                let ls: Option<&structs::engConciseInfo::L> = tr_.l.as_ref();
                                if ls.is_some() {
                                    let is: Option<&Vec<String>> = ls.unwrap().i.as_ref();
                                    if is.is_some() {
                                        for i in is.unwrap().iter() {
                                            zh_lines.push(i.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                //其他时态词汇
                let wfs: Option<&Vec<structs::engConciseInfo::Wfs>> = w.wfs.as_ref();
                if wfs.is_some() {
                    if wfs.unwrap().len() == 0 {
                        break;
                    }
                    wfs_str.push_str(&String::from("[ "));
                    for item in wfs.unwrap().iter() {
                        if item.wf.is_some() {
                            wfs_str.push_str(&String::from(item.wf.as_ref().unwrap().name.as_ref().unwrap()));
                            wfs_str.push_str(&String::from(" "));
                            wfs_str.push_str(&String::from(item.wf.as_ref().unwrap().value.as_ref().unwrap()));
                            wfs_str.push_str(&String::from(" "));
                        }
                    }
                    wfs_str.push_str(&String::from("]"));
                }
            }
        }
    }
    EnToZh {
        zhLines: zh_lines,
        wfs: wfs_str
    }
}


///
/// 中译英
/// 单词行列表获取
/// 
pub fn request_result_parse_zh_to_en(resultObj: &structs::zhConciseInfo::ZhConciseInfo) -> Vec<String> {
    let mut en_lines = Vec::new();
    if resultObj.ce.is_some() {
        let ce = resultObj.ce.as_ref().unwrap();
        if ce.word.is_some() {
            let word = ce.word.as_ref().unwrap();
            let word_0 = word.get(0);
            if word_0.is_some() {
                let w: &structs::zhConciseInfo::Word = word_0.unwrap();
                if w.trs.is_some() {
                    for trs in w.trs.as_ref().unwrap().iter() {
                        let tr: Option<&Vec<structs::zhConciseInfo::Tr>> = trs.tr.as_ref();
                        if tr.is_some() {
                            let tr_0 = tr.unwrap().get(0);
                            if tr_0.is_some() {
                                let tr_obj: &structs::zhConciseInfo::Tr = tr_0.unwrap();
                                let l = tr_obj.l.as_ref();
                                let lu: &structs::zhConciseInfo::L = l.unwrap();
                                //一个tr中的多个i需要用空格拼接, 因为是短语
                                let iArr: &Vec<serde_json::Value> = lu.i.as_array().unwrap();
                                let mut dy: String = String::from("");
                                let mut iIdx = 0;
                                let mut handleCount = 0;
                                for iItem in iArr.iter() {
                                    iIdx += 1;
                                    if iIdx % 2 != 0 {
                                        continue;
                                    }
                                    let john = json!(iItem);
                                    let i: structs::zhConciseInfo::I = serde_json::from_str(&serde_json::to_string(&john).unwrap()).unwrap();
                                    if handleCount == 0 {
                                        dy.push_str(&i.text.unwrap());
                                    } else {
                                        dy.push_str(" ");
                                        dy.push_str(&i.text.unwrap());
                                    }
                                    handleCount += 1;
                                }

                                let mut str: String = String::from("");
                                if lu.pos.is_some() {
                                    str.push_str(&lu.pos.as_ref().unwrap());
                                    str.push_str(" ");
                                }
                                str.push_str(&dy);
                                str.push_str(&String::from(" "));
                                if lu.tran.is_some() {
                                    let tran = lu.tran.as_ref().unwrap();
                                    str.push_str(&tran);
                                }
                                en_lines.push(str);
                            }
                        }
                    }
                }
            }
        }
    }
    en_lines
}