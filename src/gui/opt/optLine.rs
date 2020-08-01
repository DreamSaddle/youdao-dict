///
/// 操作行
/// 

use std::rc::Rc;

use cpp_core::{Ptr, StaticUpcast};
use qt_core::{qs, slot, SlotNoArgs, QBox, ShortcutContext, QObject};
use qt_widgets::{QWidget, QVBoxLayout, QHBoxLayout, QPushButton, QComboBox, QShortcut};
use qt_gui::{QKeySequence};

use regex::Regex;

use crate::gui::text::transText::{TransText};


#[derive(Debug)]
pub struct OptLine {
    pub widget: QBox<QWidget>,
    pub langSelect: QBox<QComboBox>,
    pub transBtn: QBox<QPushButton>,
    pub transText: Rc<TransText>,
}

impl StaticUpcast<QObject> for OptLine {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.widget.as_ptr().static_upcast()
    }
}

impl OptLine {
    pub fn new(vBox: &QVBoxLayout, transText: &Rc<TransText>) -> Rc<Self> {
        unsafe {
            let vBoxWidget = QWidget::new_0a();
            let line = QHBoxLayout::new_1a(&vBoxWidget);
        
        
            let langSelect = QComboBox::new_0a();
            langSelect.add_item_q_string(&qs("中文 >> 英语"));
            langSelect.add_item_q_string(&qs("英语 >> 中文"));
            langSelect.set_maximum_size_2a(200, 30);
            langSelect.set_minimum_size_2a(150, 30);
            line.add_widget(&langSelect);
        
            let transBtnWidget = QWidget::new_0a();
            transBtnWidget.resize_2a(100, 30);
            let transBtn = QPushButton::from_q_widget(&transBtnWidget);
            transBtn.set_text(&qs("翻 译"));
            transBtn.resize_2a(100, 30);
            line.add_widget(&transBtnWidget);
            
            vBoxWidget.show();
            vBox.add_widget(&vBoxWidget);

            let this = Rc::new(OptLine {
                widget: vBoxWidget,
                langSelect: langSelect,
                transBtn: transBtn,
                transText: transText.clone()
            });

            this.init();
            this
        }
    }
    

    unsafe fn init(self: &Rc<Self>) {
        // self.transBtn.clicked().connect(&SlotNoArgs::new(&self.transBtn, move || {
        //     println!("fsdafsd");
        // }));
        //这种方式定义插槽, 比如让 OptLine 有足够长的生命周期, 否则插槽不会被触发
        self.transBtn.clicked().connect(&self.slot_on_trans_btn_clicked());

        self.trans_shotcut_bind();
    }
    

    ///
    /// 翻译快捷键绑定
    /// 
    unsafe fn trans_shotcut_bind(self: &Rc<Self>) {
        let enterKey = QKeySequence::from_q_string(&qs("ctrl+return"));
        let transBtnShotcut = QShortcut::new_2a(&enterKey, &self.transBtn);
        transBtnShotcut.set_context(ShortcutContext::ApplicationShortcut);
        transBtnShotcut.activated().connect(&self.slot_on_trans_btn_clicked());
    }


    ///
    /// 执行翻译插槽回调
    /// 
    #[slot(SlotNoArgs)]
    unsafe fn on_trans_btn_clicked(self: &Rc<Self>) {
        TransText::do_trans(&self.transText);
    }


    ///
    /// 翻译词输入后切换对于翻译类型
    /// 
    pub unsafe fn slot_change_lang_select_actived(self: &Rc<Self>) {
        let regex = Regex::new(r#"[\u4e00-\u9fa5]"#).unwrap();
        let w = self.transText.sourceEdit.to_plain_text().to_std_string();
        if regex.is_match(&w) {
            self.langSelect.set_current_index(0);
        } else {
            self.langSelect.set_current_index(1);
        }
    }
}

