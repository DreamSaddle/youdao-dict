
///
/// 翻译结果展示组件
///

use std::rc::Rc;

use cpp_core::{NullPtr, Ptr, StaticUpcast};
use qt_core::{qs, SlotNoArgs, QBox, slot, QObject};
use qt_widgets::{QWidget, QVBoxLayout, QHBoxLayout, QTextEdit};

use regex::Regex;
use serde_json::json;

use crate::utils;
use crate::structs;
use crate::gui::{
    startQt::MainWindowWidgets,
    result::pronounce::Pronounce,
};


#[derive(Debug)]
pub struct TransResult {
    pub widget: QBox<QWidget>,
    pub pronounce: Rc<Pronounce>
}


impl TransResult {
    pub fn new(vBox: &QVBoxLayout) -> Rc<Self> {
        unsafe {
            let result_widget = QWidget::new_0a();
            let result_vbox = QVBoxLayout::new_1a(&result_widget);
            
            let pronounce = Pronounce::new();
            result_vbox.add_widget(pronounce.as_ref().widget.as_ptr());

            vBox.add_widget(&result_widget);
            let this = Rc::new(TransResult {
                widget: result_widget,
                pronounce: pronounce,
            });
            this.hide();
            this
        }
    }

    pub unsafe fn toogle_show(self: &Rc<Self>) {
        if self.widget.is_hidden() {
            self.show();
        } else {
            self.hide();
        }
    }

    pub unsafe fn show(self: &Rc<Self>) {
        self.widget.show();
    }

    pub unsafe fn hide(self: &Rc<Self>) {
        self.widget.hide();
    }
}