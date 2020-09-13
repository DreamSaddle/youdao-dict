
///
/// 短语
///


use std::rc::Rc;

use cpp_core::{NullPtr, Ptr, StaticUpcast,};
use qt_core::{qs, QBox, QObject, QFlags, AlignmentFlag, TextInteractionFlag};
use qt_widgets::{QWidget, QVBoxLayout, QHBoxLayout, QLabel, QLayoutItem};

use crate::structs;


#[derive(Debug)]
pub struct Phrase {
    pub widget: QBox<QWidget>,
    //短语列表容器
    pub phraseVBox: QBox<QVBoxLayout>
}

impl StaticUpcast<QObject> for Phrase {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.widget.as_ptr().static_upcast()
    }
}


impl Phrase {
    pub fn new() -> Rc<Self> {
        unsafe {
            let widget = QWidget::new_0a();
            let v_box = QVBoxLayout::new_1a(&widget);

            let title = QLabel::from_q_string(&qs("短语"));
            title.set_style_sheet(&qs("font-size:20px;font-weight:bold;"));
            v_box.add_widget(&title);

            //短语列表容器
            let phrase_vbox_widget = QWidget::new_0a();
            let phrase_vbox = QVBoxLayout::new_1a(&phrase_vbox_widget);
            v_box.add_widget(&phrase_vbox_widget);


            let this = Rc::new(Phrase{
                widget: widget,
                phraseVBox: phrase_vbox
            });

            this
        }
    }


    ///
    /// 短语列表填充
    /// 
    pub unsafe fn full_phrase_list(self: &Rc<Self>, phraseList: Vec<structs::webTrans::PhraseModel>) {
        self.clear_all_result_content();

        if !phraseList.is_empty() {
            self.widget.show();
            for phrase in phraseList.iter() {
                //行容器
                let phraseWidget = QWidget::new_0a();
                let phraseHBox: QBox<QHBoxLayout> = QHBoxLayout::new_1a(&phraseWidget);
                //短语key
                let phrase_key = QLabel::from_q_string(&qs(phrase.key.as_ref().unwrap()));
                phrase_key.set_text_interaction_flags(QFlags::from(TextInteractionFlag::TextSelectableByMouse));
                phrase_key.set_style_sheet(&qs("font-size:13px;font-weight:bold;color:#00b9f1"));
                phraseHBox.add_widget_3a(&phrase_key, 0, QFlags::from(AlignmentFlag::AlignLeft));
                //value处理
                if phrase.trans.is_some() {
                    let values: &Vec<structs::webTrans::PhraseTransItem> = phrase.trans.as_ref().unwrap();
                        let mut idx = 0;
                        let mut values_str = String::new();
                        let count = values.len() - 1;
                        for value in values.iter() {
                            if idx < count {
                                values_str.push_str(value.value.as_ref().unwrap());
                                values_str.push_str(" ; ");
                            } else {
                                values_str.push_str(value.value.as_ref().unwrap());
                            }
                            idx += 1;
                        }
                        let phrase_value = QLabel::from_q_string(&qs(values_str));
                        phrase_value.set_text_interaction_flags(QFlags::from(TextInteractionFlag::TextSelectableByMouse));
                        phrase_value.set_style_sheet(&qs("font-size:13px;"));
                        phraseHBox.add_widget_3a(&phrase_value, 9, QFlags::from(AlignmentFlag::AlignLeft));
                }

                
                self.phraseVBox.add_widget_3a(&phraseWidget, -5, QFlags::from(AlignmentFlag::AlignTop));
            }
        } else {
            //没有短语就隐藏短语容器
            self.widget.hide();
        }
    }


    ///
    /// 清空上一次翻译结果
    /// 
    unsafe fn clear_all_result_content(self: &Rc<Self>) {
        self.remove_phrase_lines();
    }
    

    ///
    /// 原短语列表清空
    /// 
    unsafe fn remove_phrase_lines(self: &Rc<Self>) {
        let count = self.phraseVBox.count();
        if count == 0 {
            return
        }
        loop {
            let item: Ptr<QLayoutItem> = self.phraseVBox.take_at(0);
            if item.is_null() {
                break;
            }
            item.to_box().unwrap().widget().set_parent(NullPtr);
        }
    }
}