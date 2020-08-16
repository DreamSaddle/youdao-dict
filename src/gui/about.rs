
use cpp_core::{Ptr};
use qt_core::{qs, QFlags, AlignmentFlag};
use qt_widgets::{QMainWindow, QWidget, QDialog, QLabel, QVBoxLayout};

use crate::gui::constants::Constants;

pub struct About {

}

impl About {
    pub fn show(mw: Ptr<QMainWindow>) {
        unsafe {
            let about_dialog = QDialog::new_1a(mw);

            let w = QWidget::new_1a(&about_dialog);
            let vbox = QVBoxLayout::new_1a(&w);
            let app_name_label = QLabel::from_q_string(&qs(Constants::application_name()));
            app_name_label.set_style_sheet(&qs("font-size:20px;font-weight:bold;"));
            vbox.add_widget(&app_name_label);

            let authors = QLabel::from_q_string(&qs("<span style='font-size:13px;font-weight:bold;'>Authors:&emsp;</span> <a href='https://github.com/DreamSaddle'>DreamSaddle</a>"));
            authors.set_open_external_links(true);
            vbox.add_widget(&authors);

            let version = QLabel::from_q_string(&qs("<span style='font-size:13px;font-weight:bold;'>Version:&emsp;</span> 0.2.0"));
            vbox.add_widget(&version);

            let project_repo = QLabel::from_q_string(&qs("<span style='font-size:13px;font-weight:bold;'>项目地址:&emsp;</span><a href='https://github.com/DreamSaddle/youdao-dict'>Github</a>"));
            project_repo.set_open_external_links(true);
            vbox.add_widget(&project_repo);

            let app_desc_label = QLabel::from_q_string(&qs("此应用使用 Rust+Qt 开发而成, 翻译接口使用<a href='http://fanyi.youdao.com/'>有道翻译Api</a><br/>本项目作为学习使用, 欢迎前往GITHUB查看详情!"));
            app_desc_label.set_style_sheet(&qs("font-size:12px;"));
            app_desc_label.set_alignment(QFlags::from(AlignmentFlag::AlignTop));
            app_desc_label.set_open_external_links(true);
            vbox.add_widget(&app_desc_label);
            
            about_dialog.set_window_title(&qs("关于"));
            about_dialog.set_fixed_size_2a(400, 300);
            about_dialog.exec();
        }
    }
}