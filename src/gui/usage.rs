
use cpp_core::{Ptr};
use qt_core::{qs, QFlags, TextInteractionFlag};
use qt_widgets::{QMainWindow, QWidget, QDialog, QLabel, QVBoxLayout};

pub struct Usage {

}

impl Usage {
    pub fn show(mw: Ptr<QMainWindow>) {
        unsafe {
            let usage_dialog = QDialog::new_1a(mw);

            let w = QWidget::new_1a(&usage_dialog);
            let vbox = QVBoxLayout::new_1a(&w);
            let shotcut_label = QLabel::from_q_string(&qs("快捷键"));
            shotcut_label.set_style_sheet(&qs("font-size:15px;font-weight:bold;"));
            vbox.add_widget(&shotcut_label);

            let do_tran_desc = QLabel::from_q_string(&qs("<span style='font-size:13px;font-weight:bold;'>ctrl+return(enter):&emsp;</span>执行翻译"));
            vbox.add_widget(&do_tran_desc);

            let clear_se_desc = QLabel::from_q_string(&qs("<span style='font-size:13px;font-weight:bold;'>ctrl+u:&emsp;</span>清空输入框内容"));
            vbox.add_widget(&clear_se_desc);


            let shotcut_label = QLabel::from_q_string(&qs("划词翻译"));
            shotcut_label.set_style_sheet(&qs("font-size:15px;font-weight:bold;"));
            vbox.add_widget(&shotcut_label);
            
            let do_tran_desc = QLabel::from_q_string(&qs("<p>划词翻译需自行添加一个全局快捷键, 快捷键对应命令为 <b style='color:red;'>YDict clipboard</b>, 比如我添加快捷键为Alt+A,之后就可以鼠标选中内容后按下Alt+A即可翻译.</p>"));
            do_tran_desc.set_text_interaction_flags(QFlags::from(TextInteractionFlag::TextSelectableByMouse));
            do_tran_desc.adjust_size();
            do_tran_desc.set_word_wrap(true);
            vbox.add_widget(&do_tran_desc);

            let other_desc = QLabel::from_q_string(&qs("其他说明： 音标发音间隔时间为1s, 翻译执行间隔时间为1s, 如果翻译一个特殊字符, 比如 |,@, 本软件不会执行翻译"));
            other_desc.adjust_size();
            other_desc.set_word_wrap(true);
            vbox.add_widget(&other_desc);
            
            usage_dialog.set_window_title(&qs("如何使用"));
            usage_dialog.set_fixed_size_2a(400, 300);
            usage_dialog.exec();
        }
    }
}