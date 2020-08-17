
extern crate rodio;

use std::io::BufReader;
use std::path::Path;
use tempfile::NamedTempFile;
use std::thread;

use crate::utils::request::save_pronounce_tmp_file;


///
/// 播放音标
/// 
/// code: 单词源
/// a_type: 1: 美式, 2: 英式
/// 
pub fn play_phonogram(code: &str, a_type: i16) -> std::io::Result<()> {
    let device = rodio::default_output_device().unwrap();
    let sink = rodio::Sink::new(&device);

    let tempFile = NamedTempFile::new()?;
    let url = format!("http://dict.youdao.com/dictvoice?audio={}&type={}", code, a_type);
    save_pronounce_tmp_file(&url, &tempFile);
    thread::spawn(move || {
        let f = std::fs::File::open(Path::new(tempFile.path())).unwrap();
        let d = rodio::Decoder::new(BufReader::new(f)).unwrap();

        let device = rodio::default_output_device().unwrap();
        let sink = rodio::Sink::new(&device);
        sink.append(d);
        sink.sleep_until_end();
    });

    Ok(())
}

///
/// 播放本地音频
/// 
/// path: 音频文件路径
/// 
pub fn play_audio(path: &str) {
    let _path = String::from(path);
    thread::spawn(move || {
        let f = std::fs::File::open(Path::new(&_path)).unwrap();
        let d = rodio::Decoder::new(BufReader::new(f)).unwrap();

        let device = rodio::default_output_device().unwrap();
        let sink = rodio::Sink::new(&device);
        sink.append(d);
        sink.sleep_until_end();
    });
}