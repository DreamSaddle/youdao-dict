///
/// 输入框
/// 

use std::rc::Rc;

use cpp_core::{NullPtr, Ptr, StaticUpcast};
use qt_core::{qs, SlotNoArgs, QBox, slot, QObject};
use qt_widgets::{QWidget, QVBoxLayout, QHBoxLayout, QTextEdit};

use regex::Regex;
use serde_json::json;

use crate::utils;
use crate::structs;

#[derive(Debug)]
pub struct TransText {
    pub sourceEdit: QBox<QTextEdit>,
    pub targetEdit: QBox<QTextEdit>,
}


impl StaticUpcast<QObject> for TransText {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.sourceEdit.as_ptr().static_upcast()
    }
}


impl TransText {
    pub fn new(vBox: &QVBoxLayout) -> Rc<Self> {
        unsafe {
            let vBoxWidget = QWidget::new_0a();
            let line = QHBoxLayout::new_1a(&vBoxWidget);
            line.set_spacing(20);
        
            let sourceEditWidget = QWidget::new_0a();
            let se = QTextEdit::from_q_widget(&sourceEditWidget);
            line.add_widget(&se);
        
            let targetEditWidget = QWidget::new_0a();
            let te = QTextEdit::from_q_widget(&targetEditWidget);
            te.set_read_only(true);
            line.add_widget(&te);
        
        
            vBox.add_widget(&vBoxWidget);

            let this = Rc::new(TransText {
                sourceEdit: se,
                targetEdit: te
            });
            this.init();
            this
        }
    }

    unsafe fn init(self: &Rc<Self>) {
    }


    ///
    /// 执行翻译
    /// 
    pub unsafe fn do_trans(self: &Rc<Self>) {
        let sourceWord = self.sourceEdit.to_plain_text().to_std_string();
        if sourceWord != "" {
            self.do_request(&sourceWord);
        }
    }

    ///
    /// 翻译请求发送
    /// 
    unsafe fn do_request(self: &Rc<Self>, sourceWord: &String) {
        let regex = Regex::new(r#"[\u4e00-\u9fa5]"#).unwrap();
        let yd_result = utils::request::do_concise_get(sourceWord);
        if regex.is_match(sourceWord) {
            self.yd_request_result_handle_zh_to_eng(&yd_result);
        } else {
            self.yd_request_result_handle_eng_to_zh(&yd_result);
        }
    }

    ///
    /// 翻译请求结果处理
    /// 英译中
    /// 
    unsafe fn yd_request_result_handle_eng_to_zh(self: &Rc<Self>, yd_result: &String) {
        let resultObj: structs::engConciseInfo::EngConciseInfo = serde_json::from_str(&yd_result).unwrap();
        let word: Option<Vec<structs::engConciseInfo::Word>> = resultObj.ec.unwrap().word;
        let mut str: String = String::from("");


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
                                            str.push_str(i);
                                            str.push_str(&String::from("\n"));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        self.targetEdit.set_text(&qs(str));
    }

    ///
    /// 翻译请求结果处理
    /// 中译英
    /// 
    unsafe fn yd_request_result_handle_zh_to_eng(self: &Rc<Self>, yd_result: &String) {
        let resultObj: structs::zhConciseInfo::ZhConciseInfo = serde_json::from_str(&yd_result).unwrap();
        let mut str: String = String::from("");

        if resultObj.ce.is_some() {
            let ce = resultObj.ce.unwrap();
            if ce.word.is_some() {
                let word = ce.word.unwrap();
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

                                    str.push_str(&dy);
                                    str.push_str(&String::from(" "));
                                    let tran = lu.tran.as_ref().unwrap();
                                    str.push_str(&tran);
                                    str.push_str(&String::from("\n"));
                                }
                            }
                        }
                    }
                }
            }
        }

        self.targetEdit.set_text(&qs(str));
    }
}