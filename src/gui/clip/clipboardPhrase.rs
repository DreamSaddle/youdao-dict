
///
/// 短语[剪贴板翻译]
///


use std::rc::Rc;

use cpp_core::{Ptr, StaticUpcast,};
use qt_core::{qs, QBox, QObject, QFlags, TextInteractionFlag};
use qt_widgets::{QWidget, QVBoxLayout, QLabel};

use crate::structs;


#[derive(Debug)]
pub struct ClipboardPhrase {
    pub widget: QBox<QWidget>,
    pub phareseContent: QBox<QLabel>,
}

impl StaticUpcast<QObject> for ClipboardPhrase {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.widget.as_ptr().static_upcast()
    }
}


impl ClipboardPhrase {
    pub fn new() -> Rc<Self> {
        unsafe {
            let widget = QWidget::new_0a();
            let v_box = QVBoxLayout::new_1a(&widget);

            let title = QLabel::from_q_string(&qs("短语"));
            title.set_style_sheet(&qs("font-size:14px;font-weight:bold;color:#F8F8FF;"));
            v_box.add_widget(&title);
            
            let pharese_content = QLabel::new();
            pharese_content.set_text_interaction_flags(QFlags::from(TextInteractionFlag::TextSelectableByMouse));
            pharese_content.adjust_size();
            pharese_content.set_word_wrap(true);
            v_box.add_widget(&pharese_content);

            let this = Rc::new(ClipboardPhrase{
                widget: widget,
                phareseContent: pharese_content
            });

            this
        }
    }


    ///
    /// 短语列表填充
    /// 
    pub unsafe fn full_phrase_list(self: &Rc<Self>, phraseList: Vec<structs::webTrans::PhraseModel>) {
        if !phraseList.is_empty() {
            let mut counter = 0;
            let mut content = String::from("<p style='font-size:12px;'>");
            for phrase in phraseList.iter() {
                if counter == 3 {break;}
                counter += 1;
                //短语key
                content.push_str("<span style='color:#F8F8FF;font-weight:500;'>");
                content.push_str(phrase.key.as_ref().unwrap());
                content.push_str(": </span>");
                //value处理
                if phrase.trans.is_some() {
                    let values: &Vec<structs::webTrans::PhraseTransItem> = phrase.trans.as_ref().unwrap();
                    let mut values_str = String::from(values[0].value.as_ref().unwrap());
                    content.push_str("<span style='color:#ffffff'>");
                    content.push_str(values_str.as_str());
                    content.push_str("</span><br/>");
                }
            }
            content.push_str("</p>");
            self.phareseContent.set_text(&qs(content.as_str()));
        } else {
            //没有短语就隐藏短语容器
            self.widget.hide();
        }
    }
}