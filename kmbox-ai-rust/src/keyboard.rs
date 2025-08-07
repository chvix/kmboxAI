//! 键盘操作模块
//!
//! 提供键盘按键模拟、检测和状态管理功能
//! 完整包装了KeyboardTable.h.rs中的所有常量、结构体和函数
//!
//! ## 功能特性
//!
//! - 支持所有标准键盘按键（200+个常量）
//! - 按键按下、释放、点击操作
//! - 按键状态检测
//! - 字符串输入功能
//! - 按键掩码设置
//! - 按键表显示
//! - 按键名称到十六进制值转换
//!
//! ## 使用示例
//!
//! ```rust
//! use kmbox_ai_rust::keyboard::{Keyboard, Key};
//!
//! let keyboard = Keyboard::new()?;
//! keyboard.press_key(Key::A)?;
//! keyboard.release_key(Key::A)?;
//! keyboard.click_key(Key::Enter, 100)?;
//! keyboard.type_string("Hello, World!")?;
//! ```

use crate::error::{check_result, KmboxError, KmboxResult};
use std::ffi::CString;

// 导入所有键盘常量
pub mod constants {
    // 基础按键常量
    pub const KEY_NONE: u32 = 0;
    pub const KEY_ERRORROLLOVER: u32 = 1;
    pub const KEY_POSTFAIL: u32 = 2;
    pub const KEY_ERRORUNDEFINED: u32 = 3;

    // 字母键 A-Z
    pub const KEY_A: u32 = 4;
    pub const KEY_B: u32 = 5;
    pub const KEY_C: u32 = 6;
    pub const KEY_D: u32 = 7;
    pub const KEY_E: u32 = 8;
    pub const KEY_F: u32 = 9;
    pub const KEY_G: u32 = 10;
    pub const KEY_H: u32 = 11;
    pub const KEY_I: u32 = 12;
    pub const KEY_J: u32 = 13;
    pub const KEY_K: u32 = 14;
    pub const KEY_L: u32 = 15;
    pub const KEY_M: u32 = 16;
    pub const KEY_N: u32 = 17;
    pub const KEY_O: u32 = 18;
    pub const KEY_P: u32 = 19;
    pub const KEY_Q: u32 = 20;
    pub const KEY_R: u32 = 21;
    pub const KEY_S: u32 = 22;
    pub const KEY_T: u32 = 23;
    pub const KEY_U: u32 = 24;
    pub const KEY_V: u32 = 25;
    pub const KEY_W: u32 = 26;
    pub const KEY_X: u32 = 27;
    pub const KEY_Y: u32 = 28;
    pub const KEY_Z: u32 = 29;

    // 数字键 0-9
    pub const KEY_1_EXCLAMATION_MARK: u32 = 30;
    pub const KEY_2_AT: u32 = 31;
    pub const KEY_3_NUMBER_SIGN: u32 = 32;
    pub const KEY_4_DOLLAR: u32 = 33;
    pub const KEY_5_PERCENT: u32 = 34;
    pub const KEY_6_CARET: u32 = 35;
    pub const KEY_7_AMPERSAND: u32 = 36;
    pub const KEY_8_ASTERISK: u32 = 37;
    pub const KEY_9_OPARENTHESIS: u32 = 38;
    pub const KEY_0_CPARENTHESIS: u32 = 39;

    // 特殊键
    pub const KEY_ENTER: u32 = 40;
    pub const KEY_ESCAPE: u32 = 41;
    pub const KEY_BACKSPACE: u32 = 42;
    pub const KEY_TAB: u32 = 43;
    pub const KEY_SPACEBAR: u32 = 44;

    // 符号键
    pub const KEY_MINUS_UNDERSCORE: u32 = 45;
    pub const KEY_EQUAL_PLUS: u32 = 46;
    pub const KEY_OBRACKET_AND_OBRACE: u32 = 47;
    pub const KEY_CBRACKET_AND_CBRACE: u32 = 48;
    pub const KEY_BACKSLASH_VERTICAL_BAR: u32 = 49;
    pub const KEY_NONUS_NUMBER_SIGN_TILDE: u32 = 50;
    pub const KEY_SEMICOLON_COLON: u32 = 51;
    pub const KEY_SINGLE_AND_DOUBLE_QUOTE: u32 = 52;
    pub const KEY_COMMA_AND_LESS: u32 = 54;
    pub const KEY_DOT_GREATER: u32 = 55;
    pub const KEY_SLASH_QUESTION: u32 = 56;

    // 功能键 F1-F24
    pub const KEY_F1: u32 = 58;
    pub const KEY_F2: u32 = 59;
    pub const KEY_F3: u32 = 60;
    pub const KEY_F4: u32 = 61;
    pub const KEY_F5: u32 = 62;
    pub const KEY_F6: u32 = 63;
    pub const KEY_F7: u32 = 64;
    pub const KEY_F8: u32 = 65;
    pub const KEY_F9: u32 = 66;
    pub const KEY_F10: u32 = 67;
    pub const KEY_F11: u32 = 68;
    pub const KEY_F12: u32 = 69;
    pub const KEY_F13: u32 = 104;
    pub const KEY_F14: u32 = 105;
    pub const KEY_F15: u32 = 106;
    pub const KEY_F16: u32 = 107;
    pub const KEY_F17: u32 = 108;
    pub const KEY_F18: u32 = 109;
    pub const KEY_F19: u32 = 110;
    pub const KEY_F20: u32 = 111;
    pub const KEY_F21: u32 = 112;
    pub const KEY_F22: u32 = 113;
    pub const KEY_F23: u32 = 114;
    pub const KEY_F24: u32 = 115;

    // 其他特殊键
    pub const KEY_PRINTSCREEN: u32 = 70;
    pub const KEY_PAUSE: u32 = 72;
    pub const KEY_INSERT: u32 = 73;
    pub const KEY_HOME: u32 = 74;
    pub const KEY_PAGEUP: u32 = 75;
    pub const KEY_DELETE: u32 = 76;
    pub const KEY_END1: u32 = 77;
    pub const KEY_PAGEDOWN: u32 = 78;
    pub const KEY_RIGHTARROW: u32 = 79;
    pub const KEY_LEFTARROW: u32 = 80;
    pub const KEY_DOWNARROW: u32 = 81;
    pub const KEY_UPARROW: u32 = 82;

    // 数字键盘
    pub const KEY_KEYPAD_NUM_LOCK_AND_CLEAR: u32 = 83;
    pub const KEY_KEYPAD_SLASH: u32 = 84;
    pub const KEY_KEYPAD_ASTERIKS: u32 = 85;
    pub const KEY_KEYPAD_MINUS: u32 = 86;
    pub const KEY_KEYPAD_PLUS: u32 = 87;
    pub const KEY_KEYPAD_ENTER: u32 = 88;
    pub const KEY_KEYPAD_1_END: u32 = 89;
    pub const KEY_KEYPAD_2_DOWN_ARROW: u32 = 90;
    pub const KEY_KEYPAD_3_PAGEDN: u32 = 91;
    pub const KEY_KEYPAD_4_LEFT_ARROW: u32 = 92;
    pub const KEY_KEYPAD_5: u32 = 93;
    pub const KEY_KEYPAD_6_RIGHT_ARROW: u32 = 94;
    pub const KEY_KEYPAD_7_HOME: u32 = 95;
    pub const KEY_KEYPAD_8_UP_ARROW: u32 = 96;
    pub const KEY_KEYPAD_9_PAGEUP: u32 = 97;
    pub const KEY_KEYPAD_0_INSERT: u32 = 98;
    pub const KEY_KEYPAD_DECIMAL_SEPARATOR_DELETE: u32 = 99;

    // 其他按键
    pub const KEY_NONUS_BACK_SLASH_VERTICAL_BAR: u32 = 100;
    pub const KEY_APPLICATION: u32 = 101;
    pub const KEY_POWER: u32 = 102;
    pub const KEY_KEYPAD_EQUAL: u32 = 103;

    // 控制键
    pub const KEY_LEFTCONTROL: u32 = 224;
    pub const KEY_LEFTSHIFT: u32 = 225;
    pub const KEY_LEFTALT: u32 = 226;
    pub const KEY_LEFT_GUI: u32 = 227;
    pub const KEY_RIGHTCONTROL: u32 = 228;
    pub const KEY_RIGHTSHIFT: u32 = 229;
    pub const KEY_RIGHTALT: u32 = 230;
    pub const KEY_RIGHT_GUI: u32 = 231;

    // 其他功能键
    pub const KEY_EXECUTE: u32 = 116;
    pub const KEY_HELP: u32 = 117;
    pub const KEY_MENU: u32 = 118;
    pub const KEY_SELECT: u32 = 119;
    pub const KEY_STOP: u32 = 120;
    pub const KEY_AGAIN: u32 = 121;
    pub const KEY_UNDO: u32 = 122;
    pub const KEY_CUT: u32 = 123;
    pub const KEY_COPY: u32 = 124;
    pub const KEY_PASTE: u32 = 125;
    pub const KEY_FIND: u32 = 126;
    pub const KEY_MUTE: u32 = 127;
    pub const KEY_VOLUME_UP: u32 = 128;
    pub const KEY_VOLUME_DOWN: u32 = 129;
    pub const KEY_LOCKING_CAPS_LOCK: u32 = 130;
    pub const KEY_LOCKING_NUM_LOCK: u32 = 131;
    pub const KEY_LOCKING_SCROLL_LOCK: u32 = 132;
    pub const KEY_KEYPAD_COMMA: u32 = 133;
    pub const KEY_KEYPAD_EQUAL_SIGN: u32 = 134;

    // 国际化按键
    pub const KEY_INTERNATIONAL1: u32 = 135;
    pub const KEY_INTERNATIONAL2: u32 = 136;
    pub const KEY_INTERNATIONAL3: u32 = 137;
    pub const KEY_INTERNATIONAL4: u32 = 138;
    pub const KEY_INTERNATIONAL5: u32 = 139;
    pub const KEY_INTERNATIONAL6: u32 = 140;
    pub const KEY_INTERNATIONAL7: u32 = 141;
    pub const KEY_INTERNATIONAL8: u32 = 142;
    pub const KEY_INTERNATIONAL9: u32 = 143;

    // 语言按键
    pub const KEY_LANG1: u32 = 144;
    pub const KEY_LANG2: u32 = 145;
    pub const KEY_LANG3: u32 = 146;
    pub const KEY_LANG4: u32 = 147;
    pub const KEY_LANG5: u32 = 148;
    pub const KEY_LANG6: u32 = 149;
    pub const KEY_LANG7: u32 = 150;
    pub const KEY_LANG8: u32 = 151;
    pub const KEY_LANG9: u32 = 152;

    // 其他系统按键
    pub const KEY_ALTERNATE_ERASE: u32 = 153;
    pub const KEY_SYSREQ: u32 = 154;
    pub const KEY_CANCEL: u32 = 155;
    pub const KEY_CLEAR: u32 = 156;
    pub const KEY_PRIOR: u32 = 157;
    pub const KEY_RETURN: u32 = 158;
    pub const KEY_SEPARATOR: u32 = 159;
    pub const KEY_OUT: u32 = 160;
    pub const KEY_OPER: u32 = 161;
    pub const KEY_CLEAR_AGAIN: u32 = 162;
    pub const KEY_CRSEL: u32 = 163;
    pub const KEY_EXSEL: u32 = 164;

    // 扩展数字键盘按键
    pub const KEY_KEYPAD_00: u32 = 176;
    pub const KEY_KEYPAD_000: u32 = 177;
    pub const KEY_THOUSANDS_SEPARATOR: u32 = 178;
    pub const KEY_DECIMAL_SEPARATOR: u32 = 179;
    pub const KEY_CURRENCY_UNIT: u32 = 180;
    pub const KEY_CURRENCY_SUB_UNIT: u32 = 181;
    pub const KEY_KEYPAD_OPARENTHESIS: u32 = 182;
    pub const KEY_KEYPAD_CPARENTHESIS: u32 = 183;
    pub const KEY_KEYPAD_OBRACE: u32 = 184;
    pub const KEY_KEYPAD_CBRACE: u32 = 185;
    pub const KEY_KEYPAD_TAB: u32 = 186;
    pub const KEY_KEYPAD_BACKSPACE: u32 = 187;
    pub const KEY_KEYPAD_A: u32 = 188;
    pub const KEY_KEYPAD_B: u32 = 189;
    pub const KEY_KEYPAD_C: u32 = 190;
    pub const KEY_KEYPAD_D: u32 = 191;
    pub const KEY_KEYPAD_E: u32 = 192;
    pub const KEY_KEYPAD_F: u32 = 193;
    pub const KEY_KEYPAD_XOR: u32 = 194;
    pub const KEY_KEYPAD_CARET: u32 = 195;
    pub const KEY_KEYPAD_PERCENT: u32 = 196;
    pub const KEY_KEYPAD_LESS: u32 = 197;
    pub const KEY_KEYPAD_GREATER: u32 = 198;
    pub const KEY_KEYPAD_AMPERSAND: u32 = 199;
    pub const KEY_KEYPAD_LOGICAL_AND: u32 = 200;
    pub const KEY_KEYPAD_VERTICAL_BAR: u32 = 201;
    pub const KEY_KEYPAD_LOGIACL_OR: u32 = 202;
    pub const KEY_KEYPAD_COLON: u32 = 203;
    pub const KEY_KEYPAD_NUMBER_SIGN: u32 = 204;
    pub const KEY_KEYPAD_SPACE: u32 = 205;
    pub const KEY_KEYPAD_AT: u32 = 206;
    pub const KEY_KEYPAD_EXCLAMATION_MARK: u32 = 207;
    pub const KEY_KEYPAD_MEMORY_STORE: u32 = 208;
    pub const KEY_KEYPAD_MEMORY_RECALL: u32 = 209;
    pub const KEY_KEYPAD_MEMORY_CLEAR: u32 = 210;
    pub const KEY_KEYPAD_MEMORY_ADD: u32 = 211;
    pub const KEY_KEYPAD_MEMORY_SUBTRACT: u32 = 212;
    pub const KEY_KEYPAD_MEMORY_MULTIPLY: u32 = 213;
    pub const KEY_KEYPAD_MEMORY_DIVIDE: u32 = 214;
    pub const KEY_KEYPAD_PLUSMINUS: u32 = 215;
    pub const KEY_KEYPAD_CLEAR: u32 = 216;
    pub const KEY_KEYPAD_CLEAR_ENTRY: u32 = 217;
    pub const KEY_KEYPAD_BINARY: u32 = 218;
    pub const KEY_KEYPAD_OCTAL: u32 = 219;
    pub const KEY_KEYPAD_DECIMAL: u32 = 220;
    pub const KEY_KEYPAD_HEXADECIMAL: u32 = 221;
}

// 导入C函数 - 使用bindgen生成的格式
unsafe extern "C" {
    #[link_name = "\u{1}_Z15getKeyHexByNamePKc"]
    fn getKeyHexByName(str_: *const std::os::raw::c_char) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z12soft_keydowniPKc"]
    fn soft_keydown(value: std::os::raw::c_int, str_name: *const std::os::raw::c_char);

    #[link_name = "\u{1}_Z10soft_keyupiPKc"]
    fn soft_keyup(value: std::os::raw::c_int, str_name: *const std::os::raw::c_char);

    #[link_name = "\u{1}_Z13soft_keypressiPKci"]
    fn soft_keypress(
        value: std::os::raw::c_int,
        str_name: *const std::os::raw::c_char,
        t1: std::os::raw::c_int,
    );

    #[link_name = "\u{1}_Z15check_keyisdowniPKc"]
    fn check_keyisdown(
        value: std::os::raw::c_int,
        str_name: *const std::os::raw::c_char,
    ) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z9showTablev"]
    fn showTable();

    #[link_name = "\u{1}_Z9soft_maskiPKci"]
    fn soft_mask(
        value: std::os::raw::c_int,
        str_name: *const std::os::raw::c_char,
        val: std::os::raw::c_int,
    ) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z11soft_ismaskiPKc"]
    fn soft_ismask(
        value: std::os::raw::c_int,
        str_name: *const std::os::raw::c_char,
    ) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z11soft_StringPKc"]
    fn soft_String(string: *const std::os::raw::c_char) -> std::os::raw::c_int;
}

/// 键盘按键名称值结构体（对应C结构体key_name_val_t）
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KeyNameValue {
    pub str_: *mut std::os::raw::c_char,
    pub hex: std::os::raw::c_char,
}

/// 键盘按键枚举 - 完整映射所有常量
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    // 基础按键
    None,
    ErrorRollover,
    PostFail,
    ErrorUndefined,

    // 字母键 A-Z
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    // 数字键 0-9
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,

    // 特殊键
    Enter,
    Escape,
    Backspace,
    Tab,
    Space,

    // 符号键
    Minus,
    Equal,
    LeftBracket,
    RightBracket,
    Backslash,
    NonusNumberSignTilde,
    Semicolon,
    Quote,
    Comma,
    Period,
    Slash,

    // 功能键 F1-F24
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    // 其他特殊键
    PrintScreen,
    Pause,
    Insert,
    Home,
    PageUp,
    Delete,
    End,
    PageDown,
    RightArrow,
    LeftArrow,
    DownArrow,
    UpArrow,

    // 数字键盘
    KeypadNumLockAndClear,
    KeypadSlash,
    KeypadAsterisk,
    KeypadMinus,
    KeypadPlus,
    KeypadEnter,
    Keypad1End,
    Keypad2DownArrow,
    Keypad3PageDn,
    Keypad4LeftArrow,
    Keypad5,
    Keypad6RightArrow,
    Keypad7Home,
    Keypad8UpArrow,
    Keypad9PageUp,
    Keypad0Insert,
    KeypadDecimalSeparatorDelete,

    // 其他按键
    NonusBackSlashVerticalBar,
    Application,
    Power,
    KeypadEqual,

    // 控制键
    LeftControl,
    LeftShift,
    LeftAlt,
    LeftGui,
    RightControl,
    RightShift,
    RightAlt,
    RightGui,

    // 其他功能键
    Execute,
    Help,
    Menu,
    Select,
    Stop,
    Again,
    Undo,
    Cut,
    Copy,
    Paste,
    Find,
    Mute,
    VolumeUp,
    VolumeDown,
    LockingCapsLock,
    LockingNumLock,
    LockingScrollLock,
    KeypadComma,
    KeypadEqualSign,

    // 国际化按键
    International1,
    International2,
    International3,
    International4,
    International5,
    International6,
    International7,
    International8,
    International9,

    // 语言按键
    Lang1,
    Lang2,
    Lang3,
    Lang4,
    Lang5,
    Lang6,
    Lang7,
    Lang8,
    Lang9,

    // 其他系统按键
    AlternateErase,
    SysReq,
    Cancel,
    Clear,
    Prior,
    Return,
    Separator,
    Out,
    Oper,
    ClearAgain,
    CrSel,
    ExSel,

    // 扩展数字键盘按键
    Keypad00,
    Keypad000,
    ThousandsSeparator,
    DecimalSeparator,
    CurrencyUnit,
    CurrencySubUnit,
    KeypadOParenthesis,
    KeypadCParenthesis,
    KeypadOBrace,
    KeypadCBrace,
    KeypadTab,
    KeypadBackspace,
    KeypadA,
    KeypadB,
    KeypadC,
    KeypadD,
    KeypadE,
    KeypadF,
    KeypadXor,
    KeypadCaret,
    KeypadPercent,
    KeypadLess,
    KeypadGreater,
    KeypadAmpersand,
    KeypadLogicalAnd,
    KeypadVerticalBar,
    KeypadLogicalOr,
    KeypadColon,
    KeypadNumberSign,
    KeypadSpace,
    KeypadAt,
    KeypadExclamationMark,
    KeypadMemoryStore,
    KeypadMemoryRecall,
    KeypadMemoryClear,
    KeypadMemoryAdd,
    KeypadMemorySubtract,
    KeypadMemoryMultiply,
    KeypadMemoryDivide,
    KeypadPlusMinus,
    KeypadClear,
    KeypadClearEntry,
    KeypadBinary,
    KeypadOctal,
    KeypadDecimal,
    KeypadHexadecimal,

    // 自定义键（通过字符串指定）
    Custom(String),
}

impl Key {
    /// 获取按键的十六进制值
    pub fn hex_value(&self) -> u32 {
        match self {
            // 基础按键
            Key::None => constants::KEY_NONE,
            Key::ErrorRollover => constants::KEY_ERRORROLLOVER,
            Key::PostFail => constants::KEY_POSTFAIL,
            Key::ErrorUndefined => constants::KEY_ERRORUNDEFINED,

            // 字母键 A-Z
            Key::A => constants::KEY_A,
            Key::B => constants::KEY_B,
            Key::C => constants::KEY_C,
            Key::D => constants::KEY_D,
            Key::E => constants::KEY_E,
            Key::F => constants::KEY_F,
            Key::G => constants::KEY_G,
            Key::H => constants::KEY_H,
            Key::I => constants::KEY_I,
            Key::J => constants::KEY_J,
            Key::K => constants::KEY_K,
            Key::L => constants::KEY_L,
            Key::M => constants::KEY_M,
            Key::N => constants::KEY_N,
            Key::O => constants::KEY_O,
            Key::P => constants::KEY_P,
            Key::Q => constants::KEY_Q,
            Key::R => constants::KEY_R,
            Key::S => constants::KEY_S,
            Key::T => constants::KEY_T,
            Key::U => constants::KEY_U,
            Key::V => constants::KEY_V,
            Key::W => constants::KEY_W,
            Key::X => constants::KEY_X,
            Key::Y => constants::KEY_Y,
            Key::Z => constants::KEY_Z,

            // 数字键 0-9
            Key::Num0 => constants::KEY_0_CPARENTHESIS,
            Key::Num1 => constants::KEY_1_EXCLAMATION_MARK,
            Key::Num2 => constants::KEY_2_AT,
            Key::Num3 => constants::KEY_3_NUMBER_SIGN,
            Key::Num4 => constants::KEY_4_DOLLAR,
            Key::Num5 => constants::KEY_5_PERCENT,
            Key::Num6 => constants::KEY_6_CARET,
            Key::Num7 => constants::KEY_7_AMPERSAND,
            Key::Num8 => constants::KEY_8_ASTERISK,
            Key::Num9 => constants::KEY_9_OPARENTHESIS,

            // 特殊键
            Key::Enter => constants::KEY_ENTER,
            Key::Escape => constants::KEY_ESCAPE,
            Key::Backspace => constants::KEY_BACKSPACE,
            Key::Tab => constants::KEY_TAB,
            Key::Space => constants::KEY_SPACEBAR,

            // 符号键
            Key::Minus => constants::KEY_MINUS_UNDERSCORE,
            Key::Equal => constants::KEY_EQUAL_PLUS,
            Key::LeftBracket => constants::KEY_OBRACKET_AND_OBRACE,
            Key::RightBracket => constants::KEY_CBRACKET_AND_CBRACE,
            Key::Backslash => constants::KEY_BACKSLASH_VERTICAL_BAR,
            Key::NonusNumberSignTilde => constants::KEY_NONUS_NUMBER_SIGN_TILDE,
            Key::Semicolon => constants::KEY_SEMICOLON_COLON,
            Key::Quote => constants::KEY_SINGLE_AND_DOUBLE_QUOTE,
            Key::Comma => constants::KEY_COMMA_AND_LESS,
            Key::Period => constants::KEY_DOT_GREATER,
            Key::Slash => constants::KEY_SLASH_QUESTION,

            // 功能键 F1-F24
            Key::F1 => constants::KEY_F1,
            Key::F2 => constants::KEY_F2,
            Key::F3 => constants::KEY_F3,
            Key::F4 => constants::KEY_F4,
            Key::F5 => constants::KEY_F5,
            Key::F6 => constants::KEY_F6,
            Key::F7 => constants::KEY_F7,
            Key::F8 => constants::KEY_F8,
            Key::F9 => constants::KEY_F9,
            Key::F10 => constants::KEY_F10,
            Key::F11 => constants::KEY_F11,
            Key::F12 => constants::KEY_F12,
            Key::F13 => constants::KEY_F13,
            Key::F14 => constants::KEY_F14,
            Key::F15 => constants::KEY_F15,
            Key::F16 => constants::KEY_F16,
            Key::F17 => constants::KEY_F17,
            Key::F18 => constants::KEY_F18,
            Key::F19 => constants::KEY_F19,
            Key::F20 => constants::KEY_F20,
            Key::F21 => constants::KEY_F21,
            Key::F22 => constants::KEY_F22,
            Key::F23 => constants::KEY_F23,
            Key::F24 => constants::KEY_F24,

            // 其他特殊键
            Key::PrintScreen => constants::KEY_PRINTSCREEN,
            Key::Pause => constants::KEY_PAUSE,
            Key::Insert => constants::KEY_INSERT,
            Key::Home => constants::KEY_HOME,
            Key::PageUp => constants::KEY_PAGEUP,
            Key::Delete => constants::KEY_DELETE,
            Key::End => constants::KEY_END1,
            Key::PageDown => constants::KEY_PAGEDOWN,
            Key::RightArrow => constants::KEY_RIGHTARROW,
            Key::LeftArrow => constants::KEY_LEFTARROW,
            Key::DownArrow => constants::KEY_DOWNARROW,
            Key::UpArrow => constants::KEY_UPARROW,

            // 数字键盘
            Key::KeypadNumLockAndClear => constants::KEY_KEYPAD_NUM_LOCK_AND_CLEAR,
            Key::KeypadSlash => constants::KEY_KEYPAD_SLASH,
            Key::KeypadAsterisk => constants::KEY_KEYPAD_ASTERIKS,
            Key::KeypadMinus => constants::KEY_KEYPAD_MINUS,
            Key::KeypadPlus => constants::KEY_KEYPAD_PLUS,
            Key::KeypadEnter => constants::KEY_KEYPAD_ENTER,
            Key::Keypad1End => constants::KEY_KEYPAD_1_END,
            Key::Keypad2DownArrow => constants::KEY_KEYPAD_2_DOWN_ARROW,
            Key::Keypad3PageDn => constants::KEY_KEYPAD_3_PAGEDN,
            Key::Keypad4LeftArrow => constants::KEY_KEYPAD_4_LEFT_ARROW,
            Key::Keypad5 => constants::KEY_KEYPAD_5,
            Key::Keypad6RightArrow => constants::KEY_KEYPAD_6_RIGHT_ARROW,
            Key::Keypad7Home => constants::KEY_KEYPAD_7_HOME,
            Key::Keypad8UpArrow => constants::KEY_KEYPAD_8_UP_ARROW,
            Key::Keypad9PageUp => constants::KEY_KEYPAD_9_PAGEUP,
            Key::Keypad0Insert => constants::KEY_KEYPAD_0_INSERT,
            Key::KeypadDecimalSeparatorDelete => constants::KEY_KEYPAD_DECIMAL_SEPARATOR_DELETE,

            // 其他按键
            Key::NonusBackSlashVerticalBar => constants::KEY_NONUS_BACK_SLASH_VERTICAL_BAR,
            Key::Application => constants::KEY_APPLICATION,
            Key::Power => constants::KEY_POWER,
            Key::KeypadEqual => constants::KEY_KEYPAD_EQUAL,

            // 控制键
            Key::LeftControl => constants::KEY_LEFTCONTROL,
            Key::LeftShift => constants::KEY_LEFTSHIFT,
            Key::LeftAlt => constants::KEY_LEFTALT,
            Key::LeftGui => constants::KEY_LEFT_GUI,
            Key::RightControl => constants::KEY_RIGHTCONTROL,
            Key::RightShift => constants::KEY_RIGHTSHIFT,
            Key::RightAlt => constants::KEY_RIGHTALT,
            Key::RightGui => constants::KEY_RIGHT_GUI,

            // 其他功能键
            Key::Execute => constants::KEY_EXECUTE,
            Key::Help => constants::KEY_HELP,
            Key::Menu => constants::KEY_MENU,
            Key::Select => constants::KEY_SELECT,
            Key::Stop => constants::KEY_STOP,
            Key::Again => constants::KEY_AGAIN,
            Key::Undo => constants::KEY_UNDO,
            Key::Cut => constants::KEY_CUT,
            Key::Copy => constants::KEY_COPY,
            Key::Paste => constants::KEY_PASTE,
            Key::Find => constants::KEY_FIND,
            Key::Mute => constants::KEY_MUTE,
            Key::VolumeUp => constants::KEY_VOLUME_UP,
            Key::VolumeDown => constants::KEY_VOLUME_DOWN,
            Key::LockingCapsLock => constants::KEY_LOCKING_CAPS_LOCK,
            Key::LockingNumLock => constants::KEY_LOCKING_NUM_LOCK,
            Key::LockingScrollLock => constants::KEY_LOCKING_SCROLL_LOCK,
            Key::KeypadComma => constants::KEY_KEYPAD_COMMA,
            Key::KeypadEqualSign => constants::KEY_KEYPAD_EQUAL_SIGN,

            // 国际化按键
            Key::International1 => constants::KEY_INTERNATIONAL1,
            Key::International2 => constants::KEY_INTERNATIONAL2,
            Key::International3 => constants::KEY_INTERNATIONAL3,
            Key::International4 => constants::KEY_INTERNATIONAL4,
            Key::International5 => constants::KEY_INTERNATIONAL5,
            Key::International6 => constants::KEY_INTERNATIONAL6,
            Key::International7 => constants::KEY_INTERNATIONAL7,
            Key::International8 => constants::KEY_INTERNATIONAL8,
            Key::International9 => constants::KEY_INTERNATIONAL9,

            // 语言按键
            Key::Lang1 => constants::KEY_LANG1,
            Key::Lang2 => constants::KEY_LANG2,
            Key::Lang3 => constants::KEY_LANG3,
            Key::Lang4 => constants::KEY_LANG4,
            Key::Lang5 => constants::KEY_LANG5,
            Key::Lang6 => constants::KEY_LANG6,
            Key::Lang7 => constants::KEY_LANG7,
            Key::Lang8 => constants::KEY_LANG8,
            Key::Lang9 => constants::KEY_LANG9,

            // 其他系统按键
            Key::AlternateErase => constants::KEY_ALTERNATE_ERASE,
            Key::SysReq => constants::KEY_SYSREQ,
            Key::Cancel => constants::KEY_CANCEL,
            Key::Clear => constants::KEY_CLEAR,
            Key::Prior => constants::KEY_PRIOR,
            Key::Return => constants::KEY_RETURN,
            Key::Separator => constants::KEY_SEPARATOR,
            Key::Out => constants::KEY_OUT,
            Key::Oper => constants::KEY_OPER,
            Key::ClearAgain => constants::KEY_CLEAR_AGAIN,
            Key::CrSel => constants::KEY_CRSEL,
            Key::ExSel => constants::KEY_EXSEL,

            // 扩展数字键盘按键
            Key::Keypad00 => constants::KEY_KEYPAD_00,
            Key::Keypad000 => constants::KEY_KEYPAD_000,
            Key::ThousandsSeparator => constants::KEY_THOUSANDS_SEPARATOR,
            Key::DecimalSeparator => constants::KEY_DECIMAL_SEPARATOR,
            Key::CurrencyUnit => constants::KEY_CURRENCY_UNIT,
            Key::CurrencySubUnit => constants::KEY_CURRENCY_SUB_UNIT,
            Key::KeypadOParenthesis => constants::KEY_KEYPAD_OPARENTHESIS,
            Key::KeypadCParenthesis => constants::KEY_KEYPAD_CPARENTHESIS,
            Key::KeypadOBrace => constants::KEY_KEYPAD_OBRACE,
            Key::KeypadCBrace => constants::KEY_KEYPAD_CBRACE,
            Key::KeypadTab => constants::KEY_KEYPAD_TAB,
            Key::KeypadBackspace => constants::KEY_KEYPAD_BACKSPACE,
            Key::KeypadA => constants::KEY_KEYPAD_A,
            Key::KeypadB => constants::KEY_KEYPAD_B,
            Key::KeypadC => constants::KEY_KEYPAD_C,
            Key::KeypadD => constants::KEY_KEYPAD_D,
            Key::KeypadE => constants::KEY_KEYPAD_E,
            Key::KeypadF => constants::KEY_KEYPAD_F,
            Key::KeypadXor => constants::KEY_KEYPAD_XOR,
            Key::KeypadCaret => constants::KEY_KEYPAD_CARET,
            Key::KeypadPercent => constants::KEY_KEYPAD_PERCENT,
            Key::KeypadLess => constants::KEY_KEYPAD_LESS,
            Key::KeypadGreater => constants::KEY_KEYPAD_GREATER,
            Key::KeypadAmpersand => constants::KEY_KEYPAD_AMPERSAND,
            Key::KeypadLogicalAnd => constants::KEY_KEYPAD_LOGICAL_AND,
            Key::KeypadVerticalBar => constants::KEY_KEYPAD_VERTICAL_BAR,
            Key::KeypadLogicalOr => constants::KEY_KEYPAD_LOGIACL_OR,
            Key::KeypadColon => constants::KEY_KEYPAD_COLON,
            Key::KeypadNumberSign => constants::KEY_KEYPAD_NUMBER_SIGN,
            Key::KeypadSpace => constants::KEY_KEYPAD_SPACE,
            Key::KeypadAt => constants::KEY_KEYPAD_AT,
            Key::KeypadExclamationMark => constants::KEY_KEYPAD_EXCLAMATION_MARK,
            Key::KeypadMemoryStore => constants::KEY_KEYPAD_MEMORY_STORE,
            Key::KeypadMemoryRecall => constants::KEY_KEYPAD_MEMORY_RECALL,
            Key::KeypadMemoryClear => constants::KEY_KEYPAD_MEMORY_CLEAR,
            Key::KeypadMemoryAdd => constants::KEY_KEYPAD_MEMORY_ADD,
            Key::KeypadMemorySubtract => constants::KEY_KEYPAD_MEMORY_SUBTRACT,
            Key::KeypadMemoryMultiply => constants::KEY_KEYPAD_MEMORY_MULTIPLY,
            Key::KeypadMemoryDivide => constants::KEY_KEYPAD_MEMORY_DIVIDE,
            Key::KeypadPlusMinus => constants::KEY_KEYPAD_PLUSMINUS,
            Key::KeypadClear => constants::KEY_KEYPAD_CLEAR,
            Key::KeypadClearEntry => constants::KEY_KEYPAD_CLEAR_ENTRY,
            Key::KeypadBinary => constants::KEY_KEYPAD_BINARY,
            Key::KeypadOctal => constants::KEY_KEYPAD_OCTAL,
            Key::KeypadDecimal => constants::KEY_KEYPAD_DECIMAL,
            Key::KeypadHexadecimal => constants::KEY_KEYPAD_HEXADECIMAL,

            // 自定义键需要通过字符串查找
            Key::Custom(_) => 0,
        }
    }

    /// 获取按键的字符串名称
    pub fn string_name(&self) -> &str {
        match self {
            Key::Custom(name) => name,
            _ => self.default_string_name(),
        }
    }

    /// 获取默认的字符串名称
    fn default_string_name(&self) -> &str {
        match self {
            // 基础按键
            Key::None => "NONE",
            Key::ErrorRollover => "ERRORROLLOVER",
            Key::PostFail => "POSTFAIL",
            Key::ErrorUndefined => "ERRORUNDEFINED",

            // 字母键 A-Z
            Key::A => "A",
            Key::B => "B",
            Key::C => "C",
            Key::D => "D",
            Key::E => "E",
            Key::F => "F",
            Key::G => "G",
            Key::H => "H",
            Key::I => "I",
            Key::J => "J",
            Key::K => "K",
            Key::L => "L",
            Key::M => "M",
            Key::N => "N",
            Key::O => "O",
            Key::P => "P",
            Key::Q => "Q",
            Key::R => "R",
            Key::S => "S",
            Key::T => "T",
            Key::U => "U",
            Key::V => "V",
            Key::W => "W",
            Key::X => "X",
            Key::Y => "Y",
            Key::Z => "Z",

            // 数字键 0-9
            Key::Num0 => "0",
            Key::Num1 => "1",
            Key::Num2 => "2",
            Key::Num3 => "3",
            Key::Num4 => "4",
            Key::Num5 => "5",
            Key::Num6 => "6",
            Key::Num7 => "7",
            Key::Num8 => "8",
            Key::Num9 => "9",

            // 特殊键
            Key::Enter => "ENTER",
            Key::Escape => "ESCAPE",
            Key::Backspace => "BACKSPACE",
            Key::Tab => "TAB",
            Key::Space => "SPACE",

            // 符号键
            Key::Minus => "-",
            Key::Equal => "=",
            Key::LeftBracket => "[",
            Key::RightBracket => "]",
            Key::Backslash => "\\",
            Key::NonusNumberSignTilde => "NONUS_NUMBER_SIGN_TILDE",
            Key::Semicolon => ";",
            Key::Quote => "'",
            Key::Comma => ",",
            Key::Period => ".",
            Key::Slash => "/",

            // 功能键 F1-F24
            Key::F1 => "F1",
            Key::F2 => "F2",
            Key::F3 => "F3",
            Key::F4 => "F4",
            Key::F5 => "F5",
            Key::F6 => "F6",
            Key::F7 => "F7",
            Key::F8 => "F8",
            Key::F9 => "F9",
            Key::F10 => "F10",
            Key::F11 => "F11",
            Key::F12 => "F12",
            Key::F13 => "F13",
            Key::F14 => "F14",
            Key::F15 => "F15",
            Key::F16 => "F16",
            Key::F17 => "F17",
            Key::F18 => "F18",
            Key::F19 => "F19",
            Key::F20 => "F20",
            Key::F21 => "F21",
            Key::F22 => "F22",
            Key::F23 => "F23",
            Key::F24 => "F24",

            // 其他特殊键
            Key::PrintScreen => "PRINTSCREEN",
            Key::Pause => "PAUSE",
            Key::Insert => "INSERT",
            Key::Home => "HOME",
            Key::PageUp => "PAGEUP",
            Key::Delete => "DELETE",
            Key::End => "END",
            Key::PageDown => "PAGEDOWN",
            Key::RightArrow => "RIGHT",
            Key::LeftArrow => "LEFT",
            Key::DownArrow => "DOWN",
            Key::UpArrow => "UP",

            // 数字键盘
            Key::KeypadNumLockAndClear => "NUMLOCK",
            Key::KeypadSlash => "KEYPAD_SLASH",
            Key::KeypadAsterisk => "KEYPAD_ASTERISK",
            Key::KeypadMinus => "KEYPAD_MINUS",
            Key::KeypadPlus => "KEYPAD_PLUS",
            Key::KeypadEnter => "KEYPAD_ENTER",
            Key::Keypad1End => "KEYPAD_1",
            Key::Keypad2DownArrow => "KEYPAD_2",
            Key::Keypad3PageDn => "KEYPAD_3",
            Key::Keypad4LeftArrow => "KEYPAD_4",
            Key::Keypad5 => "KEYPAD_5",
            Key::Keypad6RightArrow => "KEYPAD_6",
            Key::Keypad7Home => "KEYPAD_7",
            Key::Keypad8UpArrow => "KEYPAD_8",
            Key::Keypad9PageUp => "KEYPAD_9",
            Key::Keypad0Insert => "KEYPAD_0",
            Key::KeypadDecimalSeparatorDelete => "KEYPAD_DECIMAL",

            // 其他按键
            Key::NonusBackSlashVerticalBar => "NONUS_BACK_SLASH_VERTICAL_BAR",
            Key::Application => "APPLICATION",
            Key::Power => "POWER",
            Key::KeypadEqual => "KEYPAD_EQUAL",

            // 控制键
            Key::LeftControl => "LEFTCTRL",
            Key::RightControl => "RIGHTCTRL",
            Key::LeftShift => "LEFTSHIFT",
            Key::RightShift => "RIGHTSHIFT",
            Key::LeftAlt => "LEFTALT",
            Key::RightAlt => "RIGHTALT",
            Key::LeftGui => "LEFTGUI",
            Key::RightGui => "RIGHTGUI",

            // 其他功能键
            Key::Execute => "EXECUTE",
            Key::Help => "HELP",
            Key::Menu => "MENU",
            Key::Select => "SELECT",
            Key::Stop => "STOP",
            Key::Again => "AGAIN",
            Key::Undo => "UNDO",
            Key::Cut => "CUT",
            Key::Copy => "COPY",
            Key::Paste => "PASTE",
            Key::Find => "FIND",
            Key::Mute => "MUTE",
            Key::VolumeUp => "VOLUME_UP",
            Key::VolumeDown => "VOLUME_DOWN",
            Key::LockingCapsLock => "LOCKING_CAPS_LOCK",
            Key::LockingNumLock => "LOCKING_NUM_LOCK",
            Key::LockingScrollLock => "LOCKING_SCROLL_LOCK",
            Key::KeypadComma => "KEYPAD_COMMA",
            Key::KeypadEqualSign => "KEYPAD_EQUAL_SIGN",

            // 国际化按键
            Key::International1 => "INTERNATIONAL1",
            Key::International2 => "INTERNATIONAL2",
            Key::International3 => "INTERNATIONAL3",
            Key::International4 => "INTERNATIONAL4",
            Key::International5 => "INTERNATIONAL5",
            Key::International6 => "INTERNATIONAL6",
            Key::International7 => "INTERNATIONAL7",
            Key::International8 => "INTERNATIONAL8",
            Key::International9 => "INTERNATIONAL9",

            // 语言按键
            Key::Lang1 => "LANG1",
            Key::Lang2 => "LANG2",
            Key::Lang3 => "LANG3",
            Key::Lang4 => "LANG4",
            Key::Lang5 => "LANG5",
            Key::Lang6 => "LANG6",
            Key::Lang7 => "LANG7",
            Key::Lang8 => "LANG8",
            Key::Lang9 => "LANG9",

            // 其他系统按键
            Key::AlternateErase => "ALTERNATE_ERASE",
            Key::SysReq => "SYSREQ",
            Key::Cancel => "CANCEL",
            Key::Clear => "CLEAR",
            Key::Prior => "PRIOR",
            Key::Return => "RETURN",
            Key::Separator => "SEPARATOR",
            Key::Out => "OUT",
            Key::Oper => "OPER",
            Key::ClearAgain => "CLEAR_AGAIN",
            Key::CrSel => "CRSEL",
            Key::ExSel => "EXSEL",

            // 扩展数字键盘按键
            Key::Keypad00 => "KEYPAD_00",
            Key::Keypad000 => "KEYPAD_000",
            Key::ThousandsSeparator => "THOUSANDS_SEPARATOR",
            Key::DecimalSeparator => "DECIMAL_SEPARATOR",
            Key::CurrencyUnit => "CURRENCY_UNIT",
            Key::CurrencySubUnit => "CURRENCY_SUB_UNIT",
            Key::KeypadOParenthesis => "KEYPAD_OPARENTHESIS",
            Key::KeypadCParenthesis => "KEYPAD_CPARENTHESIS",
            Key::KeypadOBrace => "KEYPAD_OBRACE",
            Key::KeypadCBrace => "KEYPAD_CBRACE",
            Key::KeypadTab => "KEYPAD_TAB",
            Key::KeypadBackspace => "KEYPAD_BACKSPACE",
            Key::KeypadA => "KEYPAD_A",
            Key::KeypadB => "KEYPAD_B",
            Key::KeypadC => "KEYPAD_C",
            Key::KeypadD => "KEYPAD_D",
            Key::KeypadE => "KEYPAD_E",
            Key::KeypadF => "KEYPAD_F",
            Key::KeypadXor => "KEYPAD_XOR",
            Key::KeypadCaret => "KEYPAD_CARET",
            Key::KeypadPercent => "KEYPAD_PERCENT",
            Key::KeypadLess => "KEYPAD_LESS",
            Key::KeypadGreater => "KEYPAD_GREATER",
            Key::KeypadAmpersand => "KEYPAD_AMPERSAND",
            Key::KeypadLogicalAnd => "KEYPAD_LOGICAL_AND",
            Key::KeypadVerticalBar => "KEYPAD_VERTICAL_BAR",
            Key::KeypadLogicalOr => "KEYPAD_LOGICAL_OR",
            Key::KeypadColon => "KEYPAD_COLON",
            Key::KeypadNumberSign => "KEYPAD_NUMBER_SIGN",
            Key::KeypadSpace => "KEYPAD_SPACE",
            Key::KeypadAt => "KEYPAD_AT",
            Key::KeypadExclamationMark => "KEYPAD_EXCLAMATION_MARK",
            Key::KeypadMemoryStore => "KEYPAD_MEMORY_STORE",
            Key::KeypadMemoryRecall => "KEYPAD_MEMORY_RECALL",
            Key::KeypadMemoryClear => "KEYPAD_MEMORY_CLEAR",
            Key::KeypadMemoryAdd => "KEYPAD_MEMORY_ADD",
            Key::KeypadMemorySubtract => "KEYPAD_MEMORY_SUBTRACT",
            Key::KeypadMemoryMultiply => "KEYPAD_MEMORY_MULTIPLY",
            Key::KeypadMemoryDivide => "KEYPAD_MEMORY_DIVIDE",
            Key::KeypadPlusMinus => "KEYPAD_PLUSMINUS",
            Key::KeypadClear => "KEYPAD_CLEAR",
            Key::KeypadClearEntry => "KEYPAD_CLEAR_ENTRY",
            Key::KeypadBinary => "KEYPAD_BINARY",
            Key::KeypadOctal => "KEYPAD_OCTAL",
            Key::KeypadDecimal => "KEYPAD_DECIMAL",
            Key::KeypadHexadecimal => "KEYPAD_HEXADECIMAL",

            // 自定义键
            Key::Custom(_) => unreachable!(),
        }
    }
}

/// 按键状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    /// 按键按下
    Pressed,
    /// 按键释放
    Released,
    /// 按键保持按下
    Held,
}

/// 键盘控制器
pub struct Keyboard {
    initialized: bool,
}

impl Keyboard {
    /// 创建新的键盘控制器
    pub fn new() -> KmboxResult<Self> {
        Ok(Self { initialized: true })
    }

    /// 按下指定按键
    pub fn press_key(&self, key: Key) -> KmboxResult<()> {
        if !self.initialized {
            return Err(KmboxError::InitializationError("键盘未初始化".to_string()));
        }

        let key_name = CString::new(key.string_name())
            .map_err(|e| KmboxError::ParameterError(format!("无效的按键名称: {}", e)))?;

        unsafe {
            soft_keydown(key.hex_value() as i32, key_name.as_ptr());
        }

        Ok(())
    }

    /// 释放指定按键
    pub fn release_key(&self, key: Key) -> KmboxResult<()> {
        if !self.initialized {
            return Err(KmboxError::InitializationError("键盘未初始化".to_string()));
        }

        let key_name = CString::new(key.string_name())
            .map_err(|e| KmboxError::ParameterError(format!("无效的按键名称: {}", e)))?;

        unsafe {
            soft_keyup(key.hex_value() as i32, key_name.as_ptr());
        }

        Ok(())
    }

    /// 点击按键（按下后立即释放）
    pub fn click_key(&self, key: Key, duration_ms: u32) -> KmboxResult<()> {
        if !self.initialized {
            return Err(KmboxError::InitializationError("键盘未初始化".to_string()));
        }

        let key_name = CString::new(key.string_name())
            .map_err(|e| KmboxError::ParameterError(format!("无效的按键名称: {}", e)))?;

        unsafe {
            soft_keypress(
                key.hex_value() as i32,
                key_name.as_ptr(),
                duration_ms as i32,
            );
        }

        Ok(())
    }

    /// 检查按键是否处于按下状态
    pub fn is_key_pressed(&self, key: Key) -> KmboxResult<bool> {
        if !self.initialized {
            return Err(KmboxError::InitializationError("键盘未初始化".to_string()));
        }

        let key_name = CString::new(key.string_name())
            .map_err(|e| KmboxError::ParameterError(format!("无效的按键名称: {}", e)))?;

        let result = unsafe { check_keyisdown(key.hex_value() as i32, key_name.as_ptr()) };

        Ok(result != 0)
    }

    /// 设置按键掩码
    pub fn set_key_mask(&self, key: Key, mask_value: i32) -> KmboxResult<i32> {
        if !self.initialized {
            return Err(KmboxError::InitializationError("键盘未初始化".to_string()));
        }

        let key_name = CString::new(key.string_name())
            .map_err(|e| KmboxError::ParameterError(format!("无效的按键名称: {}", e)))?;

        let result = unsafe { soft_mask(key.hex_value() as i32, key_name.as_ptr(), mask_value) };

        Ok(result)
    }

    /// 检查按键掩码状态
    pub fn is_key_masked(&self, key: Key) -> KmboxResult<bool> {
        if !self.initialized {
            return Err(KmboxError::InitializationError("键盘未初始化".to_string()));
        }

        let key_name = CString::new(key.string_name())
            .map_err(|e| KmboxError::ParameterError(format!("无效的按键名称: {}", e)))?;

        let result = unsafe { soft_ismask(key.hex_value() as i32, key_name.as_ptr()) };

        Ok(result != 0)
    }

    /// 输入字符串
    pub fn type_string(&self, text: &str) -> KmboxResult<()> {
        if !self.initialized {
            return Err(KmboxError::InitializationError("键盘未初始化".to_string()));
        }

        let text_cstr = CString::new(text)
            .map_err(|e| KmboxError::ParameterError(format!("无效的字符串: {}", e)))?;

        let result = unsafe { soft_String(text_cstr.as_ptr()) };

        check_result(result, "输入字符串")
    }

    /// 显示按键表
    pub fn show_key_table(&self) -> KmboxResult<()> {
        if !self.initialized {
            return Err(KmboxError::InitializationError("键盘未初始化".to_string()));
        }

        unsafe {
            showTable();
        }

        Ok(())
    }

    /// 根据名称获取按键的十六进制值
    pub fn get_key_hex_by_name(name: &str) -> KmboxResult<i32> {
        let name_cstr = CString::new(name)
            .map_err(|e| KmboxError::ParameterError(format!("无效的按键名称: {}", e)))?;

        let result = unsafe { getKeyHexByName(name_cstr.as_ptr()) };

        Ok(result)
    }
}

impl Drop for Keyboard {
    fn drop(&mut self) {
        // 清理资源（如果需要）
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_hex_values() {
        assert_eq!(Key::A.hex_value(), constants::KEY_A);
        assert_eq!(Key::Z.hex_value(), constants::KEY_Z);
        assert_eq!(Key::Enter.hex_value(), constants::KEY_ENTER);
        assert_eq!(Key::Escape.hex_value(), constants::KEY_ESCAPE);
        assert_eq!(Key::F1.hex_value(), constants::KEY_F1);
        assert_eq!(Key::F12.hex_value(), constants::KEY_F12);
        assert_eq!(Key::LeftControl.hex_value(), constants::KEY_LEFTCONTROL);
        assert_eq!(Key::RightControl.hex_value(), constants::KEY_RIGHTCONTROL);
    }

    #[test]
    fn test_key_string_names() {
        assert_eq!(Key::A.string_name(), "A");
        assert_eq!(Key::Enter.string_name(), "ENTER");
        assert_eq!(Key::LeftControl.string_name(), "LEFTCTRL");
        assert_eq!(Key::Custom("TEST".to_string()).string_name(), "TEST");
    }

    #[test]
    fn test_keyboard_creation() {
        let keyboard = Keyboard::new();
        assert!(keyboard.is_ok());
    }

    #[test]
    fn test_custom_key() {
        let custom_key = Key::Custom("CUSTOM_KEY".to_string());
        assert_eq!(custom_key.string_name(), "CUSTOM_KEY");
        assert_eq!(custom_key.hex_value(), 0); // 自定义键需要通过字符串查找
    }

    #[test]
    fn test_key_state() {
        let pressed = KeyState::Pressed;
        let released = KeyState::Released;
        let held = KeyState::Held;

        assert_ne!(pressed, released);
        assert_ne!(pressed, held);
        assert_ne!(released, held);
    }
}
