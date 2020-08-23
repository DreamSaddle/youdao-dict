
///
/// 翻译结果展示组件
///

use std::rc::Rc;

use cpp_core::{NullPtr, Ptr, StaticUpcast};
use qt_core::{qs, SlotNoArgs, QBox, slot, QObject, QFlags, AlignmentFlag};
use qt_widgets::{QWidget, QVBoxLayout};


use crate::gui::{
    result::pronounce::Pronounce,
    result::phrase::Phrase,
};


#[derive(Debug)]
pub struct TransResult {
    pub widget: QBox<QWidget>,
    pub pronounce: Rc<Pronounce>,
    pub phrase: Rc<Phrase>,
}


impl TransResult {
    pub fn new(vBox: &QVBoxLayout) -> Rc<Self> {
        unsafe {
            let result_widget = QWidget::new_0a();
            let result_vbox = QVBoxLayout::new_1a(&result_widget);
            
            //基础结果信息, 发音栏
            let pronounce = Pronounce::new();
            result_vbox.add_widget_3a(pronounce.as_ref().widget.as_ptr(), 0, QFlags::from(AlignmentFlag::AlignTop));

            //短语
            let phrase = Phrase::new();
            result_vbox.add_widget_3a(phrase.as_ref().widget.as_ptr(), 0, QFlags::from(AlignmentFlag::AlignTop));

            vBox.add_widget_3a(&result_widget, 0, QFlags::from(AlignmentFlag::AlignTop));
            vBox.add_stretch_0a();
            let this = Rc::new(TransResult {
                widget: result_widget,
                pronounce: pronounce,
                phrase: phrase
            });
            this.hide();
            this
        }
    }

    
    ///
    /// 翻译结果容器显示状态切换
    /// 
    pub unsafe fn toggle_show(self: &Rc<Self>) {
        if self.widget.is_hidden() {
            self.show();
        } else {
            self.hide();
        }
    }


    ///
    /// 显示翻译结果容器
    /// 
    pub unsafe fn show(self: &Rc<Self>) {
        self.widget.show();
    }


    ///
    /// 隐藏翻译结果容器
    /// 
    pub unsafe fn hide(self: &Rc<Self>) {
        self.widget.hide();
    }
}