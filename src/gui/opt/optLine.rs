///
/// 操作行
/// 

use std::rc::Rc;

use cpp_core::{Ptr, StaticUpcast};
use qt_core::{qs, slot, SlotNoArgs, QBox, ShortcutContext, QObject};
use qt_widgets::{QWidget, QVBoxLayout, QHBoxLayout, QPushButton, QComboBox, QShortcut};
use qt_gui::{QKeySequence};

use regex::Regex;

use crate::gui::{
    startQt::MainWindowWidgets,
    text::transText::{TransText},
    result::transResult::{TransResult},
};


#[derive(Debug)]
pub struct OptLine {
    pub widget: QBox<QWidget>,
    pub langSelect: QBox<QComboBox>,
    pub transBtn: QBox<QPushButton>
}

impl StaticUpcast<QObject> for OptLine {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.widget.as_ptr().static_upcast()
    }
}

impl OptLine {
    pub fn new(vBox: &QVBoxLayout) -> Rc<Self> {
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
                transBtn: transBtn
            });

            this
        }
    }
    

    ///
    /// 插槽初始化 
    /// 
    pub unsafe fn init_slots(mww: &Rc<MainWindowWidgets>) {
        mww.optLine.init(mww);
    }


    unsafe fn init(self: &Rc<Self>, mww: &Rc<MainWindowWidgets>) {
        //搜索按钮点击
        let _mww = mww.clone();
        let _transText = mww.transText.clone();
        let _transResult = mww.transResult.clone();
        self.transBtn.clicked().connect(&SlotNoArgs::new(&self.transBtn, move || {
            TransText::do_trans(&_transText, &_mww);
        }));

        //输入框文本变化
        // transText.sourceEdit.text_changed().connect(&self.slot_on_source_edit_text_changed());

        //翻译快捷键绑定
        self.trans_shotcut_bind(&mww.transText, mww);
    }
    

    ///
    /// 翻译快捷键绑定
    /// 
    unsafe fn trans_shotcut_bind(self: &Rc<Self>, transText: &Rc<TransText>, mww: &Rc<MainWindowWidgets>) {
        let enterKey = QKeySequence::from_q_string(&qs("ctrl+return"));
        let transBtnShotcut = QShortcut::new_2a(&enterKey, &self.transBtn);
        transBtnShotcut.set_context(ShortcutContext::ApplicationShortcut);
        let _transText = transText.clone();
        let _mww = mww.clone();
        transBtnShotcut.activated().connect(&SlotNoArgs::new(&self.transBtn, move || {
            TransText::do_trans(&_transText, &_mww);
        }));
    }


    ///
    /// 翻译词输入后切换对于翻译类型
    /// 
    pub unsafe fn on_change_lang_select_actived(self: &Rc<Self>, transText: &Rc<TransText>) {
        let source = &transText.sourceEdit.to_plain_text().to_std_string();
        let regex = Regex::new(r#"[\u4e00-\u9fa5]"#).unwrap();
        if regex.is_match(source) {
            self.langSelect.set_current_index(0);
        } else {
            self.langSelect.set_current_index(1);
        }
    }
}

