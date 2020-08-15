///
/// 输入框
/// 

use std::rc::Rc;

use cpp_core::{NullPtr, Ptr, StaticUpcast};
use qt_core::{qs, SlotNoArgs, QBox, slot, QObject, QSize, ShortcutContext};
use qt_widgets::{QWidget, QVBoxLayout, QHBoxLayout, QTextEdit, QShortcut};
use qt_gui::{QKeySequence};

use regex::Regex;
use serde_json::json;

use crate::utils;
use crate::structs;
use crate::gui::{
    startQt::MainWindowWidgets,
    opt::optLine::{OptLine},
    result::pronounce::{Pronounce},
    result::phrase::{Phrase},
};

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
            se.set_maximum_height(100);
            line.add_widget(&se);
        
            let targetEditWidget = QWidget::new_0a();
            let te = QTextEdit::from_q_widget(&targetEditWidget);
            te.set_read_only(true);
            // line.add_widget(&te);
        
            vBox.add_spacing(0);
            vBox.add_widget(&vBoxWidget);
            vBox.add_stretch_0a();

            let this = Rc::new(TransText {
                sourceEdit: se,
                targetEdit: te
            });
            this
        }
    }
    

    ///
    /// 插槽初始化 
    /// 
    pub unsafe fn init_slots(mww: &Rc<MainWindowWidgets>) {
        mww.transText.init(&mww.optLine);
    }


    unsafe fn init(self: &Rc<Self>, optLine: &Rc<OptLine>) {
        self.source_clear_shotcut_bind();

        self.source_edit_text_changed(optLine);
    }


    ///
    /// 输入框快速清空快捷键绑定
    /// 
    unsafe fn source_clear_shotcut_bind(self: &Rc<Self>) {
        let enterKey = QKeySequence::from_q_string(&qs("ctrl+u"));
        let transBtnShotcut = QShortcut::new_2a(&enterKey, &self.sourceEdit);
        transBtnShotcut.set_context(ShortcutContext::ApplicationShortcut);
        transBtnShotcut.activated().connect(&self.slot_on_source_clear_shotcut_activated());
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_source_clear_shotcut_activated(self: &Rc<Self>) {
        self.sourceEdit.set_plain_text(&qs(""));
    }
    
    ///
    /// 翻译字符改变
    /// 
    unsafe fn source_edit_text_changed(self: &Rc<Self>, optLine: &Rc<OptLine>) {
        let _optLine = optLine.clone();
        let _self = self.clone();
        self.sourceEdit.text_changed().connect(&SlotNoArgs::new(&self.sourceEdit, move || {
            OptLine::on_change_lang_select_actived(&_optLine, &_self);
        }));
    }

    ///
    /// 执行翻译
    /// 
    pub unsafe fn do_trans(self: &Rc<Self>, mww: &Rc<MainWindowWidgets>) {
        let sourceWord = self.sourceEdit.to_plain_text().to_std_string();
        if sourceWord != "" {
            self.do_request(&sourceWord, mww);
        }
    }

    ///
    /// 翻译请求发送
    /// 
    unsafe fn do_request(self: &Rc<Self>, sourceWord: &String, mww: &Rc<MainWindowWidgets>) {
        let regex = Regex::new(r#"[\u4e00-\u9fa5]"#).unwrap();
        let yd_result = utils::request::do_concise_get(sourceWord);
        if regex.is_match(sourceWord) {
            self.yd_request_result_handle_zh_to_eng(&yd_result, mww);
        } else {
            self.yd_request_result_handle_en_to_zh(&yd_result, mww);
        }

        let yd_full_result = utils::request::do_full_get(sourceWord);
        self.yd_request_full_result_handle(&yd_full_result, mww);
    }

    ///
    /// 翻译请求结果处理
    /// 英译中
    /// 
    unsafe fn yd_request_result_handle_en_to_zh(self: &Rc<Self>, yd_result: &String, mww: &Rc<MainWindowWidgets>) {
        let resultObj: structs::engConciseInfo::EngConciseInfo = serde_json::from_str(&yd_result).unwrap();
        let _pronounce = mww.transResult.pronounce.clone();
        //解析且展示翻译结果
        Pronounce::full_en_to_zh_trans_result(&_pronounce, &resultObj, mww);
    }

    ///
    /// 翻译请求结果处理
    /// 中译英
    /// 
    unsafe fn yd_request_result_handle_zh_to_eng(self: &Rc<Self>, yd_result: &String, mww: &Rc<MainWindowWidgets>) {
        let resultObj: structs::zhConciseInfo::ZhConciseInfo = serde_json::from_str(&yd_result).unwrap();
        let _pronounce = mww.transResult.pronounce.clone();
        //解析且展示翻译结果
        Pronounce::full_zh_to_en_trans_result(&_pronounce, &resultObj, mww);
    }


    ///
    /// 详细请求结果处理
    /// 包含网络释义, 短语
    /// 
    unsafe fn yd_request_full_result_handle(self: &Rc<Self>, yd_full_result: &String, mww: &Rc<MainWindowWidgets>) {
        let result: serde_json::Value = serde_json::from_str(&yd_full_result).unwrap();
        let john = json!(result["web_trans"]["web-translation"]);
        // println!("{:?}", john);
        
        let webTrans: serde_json::Value = serde_json::from_value(john).unwrap();
        //短语列表
        let mut phraseTransList: Vec<structs::webTrans::PhraseModel> = Vec::new();
        if webTrans.is_array() {
            let webTransArr: &Vec<serde_json::Value> = webTrans.as_array().unwrap();
            //网络释义, 暂时不展示
            // let np: structs::webTrans::NetworkParaphrase = serde_json::from_value(webTransArr.get(0).unwrap().clone()).unwrap();
            // println!("{:?}", np);
            
            let webTransLen = webTransArr.len();
            let mut idx = 1;  //短语需要从第1个元素开始, 第0个元素为网络释义
            loop {
                if idx >= webTransLen {
                    break;
                }
                let bean: structs::webTrans::PhraseModel = serde_json::from_value(webTransArr.get(idx).unwrap().clone()).unwrap();
                phraseTransList.push(bean);
                idx += 1;
            }
        }

        //执行短语填充
        let _phrase = mww.transResult.phrase.clone();
        Phrase::full_phrase_list(&_phrase, phraseTransList);
    }
}