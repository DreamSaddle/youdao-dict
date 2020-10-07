
use std::rc::Rc;

use cpp_core::{Ptr, StaticUpcast};
use qt_core::{qs, SlotNoArgs, QFlags, QCoreApplication, QObject, WindowType, AlignmentFlag};
use qt_widgets::{QApplication, QWidget, QVBoxLayout};
use qt_gui::{QCursor, QGuiApplication};
use regex::Regex;
use serde_json::json;
use inputbot::KeybdKey::{LControlKey, CKey};

use crate::utils;
use crate::structs;
use super::{
    clipboardResult::ClipboardResult,
    clipboardPhrase::ClipboardPhrase,
};

pub struct ClipBoard {
    pub widget: Ptr<QWidget>,
    pub baseResult: Rc<ClipboardResult>,
    pub phraseResult: Rc<ClipboardPhrase>,
}

impl StaticUpcast<QObject> for ClipBoard {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.widget.static_upcast()
    }
}


///
/// 模拟Ctrl+C
/// 
fn simulation_copy() {
    std::thread::sleep(std::time::Duration::from_millis(50));
    LControlKey.press();
    CKey.press();
    std::thread::sleep(std::time::Duration::from_millis(50));
    CKey.release();
    LControlKey.release();
}


impl ClipBoard {

    pub fn show() {
        unsafe {
            QApplication::init(|app| {
                simulation_copy();

                let clipboard= QGuiApplication::clipboard();
                let clipboard_word = clipboard.text().to_std_string();


                let base_result = ClipboardResult::new();
                let phrase_result = ClipboardPhrase::new();

                let contentWidget = QWidget::new_0a();

                let v_box = QVBoxLayout::new_1a(&contentWidget);
                v_box.add_widget_3a(&base_result.widget, 0, QFlags::from(AlignmentFlag::AlignTop));
                v_box.add_widget_3a(&phrase_result.widget, 0, QFlags::from(AlignmentFlag::AlignTop));
                
                contentWidget.set_window_flags(QFlags::from(WindowType::FramelessWindowHint));
                contentWidget.set_style_sheet(&qs("background-color:#54546c;"));
                contentWidget.set_minimum_width(300);
                contentWidget.set_minimum_height(150);
                
                let this = Rc::new(ClipBoard {
                    widget: contentWidget.as_ptr(),
                    baseResult: base_result,
                    phraseResult: phrase_result,
                });
                
                this.do_request(&clipboard_word);
                this.detection_screen_margin();
                contentWidget.show();
                
                // 应用状态改变监听, 目前此事件主要作用于 鼠标点击翻译窗口外部自动关闭翻译窗口(包括Alt+Tab切换窗口也会), 只要翻译创建不是激活窗口就自动关闭
                app.application_state_changed().connect(&SlotNoArgs::new(app, move || {
                    if contentWidget.is_active_window() {
                        QCoreApplication::exit_0a();
                    }
                }));

                QApplication::exec()
            });
            
        }
    }
    

    ///
    /// 屏幕边缘检测
    /// 
    unsafe fn detection_screen_margin(self: &Rc<Self>) {
        let point = QCursor::pos_0a();
        let desktop = QApplication::desktop();
        let desk_rect = desktop.available_geometry();
        let mut x = point.x();
        let mut y = point.y();
        if x + self.widget.width() > desk_rect.width() - 10 {
            x = x - (x + self.widget.width() - desk_rect.width() - 10);
        }
        if y + self.widget.height() > desk_rect.height() - 10 {
            y = y - (y + self.widget.height() - desk_rect.height() - 10);
        }
        self.widget.move_2a(x, y);
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
            self.yd_request_result_handle_en_to_zh(&yd_result);
        }

        let yd_full_result = utils::request::do_full_get(sourceWord);
        self.yd_request_full_result_handle(&yd_full_result);
    }

    ///
    /// 翻译请求结果处理
    /// 英译中
    /// 
    unsafe fn yd_request_result_handle_en_to_zh(self: &Rc<Self>, yd_result: &String) {
        let resultObj: structs::engConciseInfo::EngConciseInfo = serde_json::from_str(&yd_result).unwrap();
        // let _pronounce = mww.transResult.pronounce.clone();
        //解析且展示翻译结果
        ClipboardResult::full_en_to_zh_trans_result(&self.baseResult, &resultObj);
    }

    ///
    /// 翻译请求结果处理
    /// 中译英
    /// 
    unsafe fn yd_request_result_handle_zh_to_eng(self: &Rc<Self>, yd_result: &String) {
        let resultObj: structs::zhConciseInfo::ZhConciseInfo = serde_json::from_str(&yd_result).unwrap();
        //解析且展示翻译结果
        ClipboardResult::full_zh_to_en_trans_result(&self.baseResult, &resultObj);
    }


    unsafe fn yd_request_full_result_handle(self: &Rc<Self>, yd_full_result: &String) {
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
        ClipboardPhrase::full_phrase_list(&self.phraseResult, phraseTransList);
    }
}