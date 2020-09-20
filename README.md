<p align="center">
    <a href="https://www.rust-lang.org" target="_blank">
    <img src="https://img.shields.io/badge/-Rust v1.45-ef5285" alt="RUST"/>
    </a>
    <a href="https://doc.qt.io/" target="_blank">
    <img src="https://img.shields.io/badge/-Qt5 or higher-379392" alt="Qt5 or higher"/>
    </a>
    <a href="https://github.com/rust-qt" target="_blank">
    <img src="https://img.shields.io/badge/-Rust Qt v0.5.0-7200da" alt="RUST-QT"/>
    </a>
</p>
<p align="center">
    <a href="https://github.com/DreamSaddle/youdao-dict/releases">
    <img src="https://img.shields.io/github/v/release/DreamSaddle/youdao-dict?style=badge&color=58C9B9" alt="RELEASE"/>
    </a>
    <a href="https://github.com/DreamSaddle/youdao-dict/commits/master">
    <img src="https://img.shields.io/github/last-commit/DreamSaddle/youdao-dict?style=badge&color=30A9DE" alt="LAST COMMIT"/>
    </a>
    <a href="https://github.com/DreamSaddle/youdao-dict/issues">
    <img src="https://img.shields.io/github/issues/DreamSaddle/youdao-dict?style=badge&color=E71D36" alt="ISSUES"/>
    </a>
    <a href="https://github.com/DreamSaddle/youdao-dict/blob/master/LICENSE">
    <img src="https://img.shields.io/github/license/DreamSaddle/youdao-dict?style=badge&color=EFDC05" alt="LICENSE"/>
    </a>
</p>


# YDict

**YDict** 是一个使用 `Rust` + `Qt` 开发的仿<u>有道词典</u>的**翻译** + **词典** 工具.

此项目所用到的Api为 [有道翻译Api](http://fanyi.youdao.com/?_blank).



## New In 0.3.1

+ :bug: 解决翻译特殊符号以及无网络闪退



## Full Changelog

[Available Here](CHANGELOG.md)



## Help

[See the help](HELP.md)



## Installation Or Upgrade

### archlinux(aur)

```shell
# 终端执行
yay -S ydict

# 升级注意
通过aur的方式安装其实也是下载release文件之后通过 install.sh 脚本进行安装,所以在升级 YDict 时不能通过 pamac-manager(Discover之类的应用商店) 进行更新,因为 install.sh 脚本会要求输入root账户密码,而此类软件不能输入密码,所以会安装失败

# 正确的升级方式, 请使用下面命令, 你也可以选择性进行升级
# 在终端中执行升级是可以输入root账户密码的
yay -Syu
```



### 其他方式(手动安装)

```shell
# 下载
# 请自行前往Release下载最新版本
wget https://github.com/DreamSaddle/youdao-dict/releases/download/0.3.1/YDict-0.3.1.tar.gz

# 解压
tar -zxvf YDict-0.3.1.tar.gz

# 安装/更新
cd YDict-0.3.1/scripts
sudo chmod +x install.sh
./install.sh

# 执行完成即安装完成
# 可在启动器 工具 类目中查看启动, 或直接终端运行
YDict

# 卸载
# 你可以单独获取项目 scripts目录中的 uninstall.sh执行卸载

```



## Usage

输入框输入待翻译的词汇点击翻译按钮接口完成翻译.

### ShotCut

**Ctrl+U:** 清空输入框内容

**Ctrl+Return(Enter):** 翻译



### 划词翻译

首先安装好**YDict**, 接着你需要在系统中为YDict的划词功能配置一个全局快捷键, 此快捷键的命令为 `YDict clipboard`

例如:

![划词翻译全局快捷键配置](media/screenshots/hc_shotcut.png)



## Development

### Requirements

+ Rust
+ Qt5 or higher



建议开发使用`Linux`桌面环境开发. **此项目也只提供Linux版本Release**.

首先请确保你已经安装好了`Rust`环境以及`Qt5`.

```shell
git clone https://github.com/DreamSaddle/youdao-dict.git
cd youdao-dict
cargo run

# 构建Release
cargo build --release
```



## Screenshots

![中译英](media/screenshots/003.png)

![短语](media/screenshots/004.png)

![段落翻译](media/screenshots/001.png)

![划词翻译](media/screenshots/006.png)

![段落划词翻译](media/screenshots/007.png)

## 为什么会有这个项目

我自从桌面切换为Linux环境后, 苦于英语差, 但是又没有找到好用的翻译软件. 便想着自己搞一个, 顺便还可以练习一下 `Rust` .

本项目主要用于学习和个人使用, 目前`Rust`资料比较少, `Rust-Qt`资料更是少, 希望此项目能对有兴趣的同学起到一定的帮助作用吧.