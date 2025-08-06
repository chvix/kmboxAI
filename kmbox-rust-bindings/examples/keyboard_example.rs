//! 键盘模块使用示例
//!
//! 演示如何使用键盘模块进行按键操作

use kmbox_ai_rust::{
    error::KmboxResult,
    keyboard::{Key, Keyboard},
};

fn main() -> KmboxResult<()> {
    println!("=== KmboxAI 键盘模块示例 ===");

    // 创建键盘控制器
    let keyboard = Keyboard::new()?;
    println!("✓ 键盘控制器初始化成功");

    // 显示按键表
    println!("显示按键表:");
    keyboard.show_key_table()?;

    // 测试按键操作
    println!("\n测试按键操作:");

    // 按下A键
    println!("按下A键...");
    keyboard.press_key(Key::A)?;
    std::thread::sleep(std::time::Duration::from_millis(100));

    // 释放A键
    println!("释放A键...");
    keyboard.release_key(Key::A)?;

    // 点击Enter键
    println!("点击Enter键...");
    keyboard.click_key(Key::Enter, 50)?;

    // 检查按键状态
    println!("检查A键是否按下: {}", keyboard.is_key_pressed(Key::A)?);

    // 输入字符串
    println!("输入字符串 'Hello World'...");
    keyboard.type_string("Hello World")?;

    // 测试组合键
    println!("测试组合键 Ctrl+A...");
    keyboard.press_key(Key::LeftControl)?;
    keyboard.press_key(Key::A)?;
    std::thread::sleep(std::time::Duration::from_millis(100));
    keyboard.release_key(Key::A)?;
    keyboard.release_key(Key::LeftControl)?;

    // 测试自定义键
    let custom_key = Key::Custom("CUSTOM_KEY".to_string());
    println!("测试自定义键: {:?}", custom_key);

    // 获取按键十六进制值
    let hex_value = Keyboard::get_key_hex_by_name("A")?;
    println!("A键的十六进制值: {}", hex_value);

    // 测试按键掩码
    println!("设置A键掩码...");
    let mask_result = keyboard.set_key_mask(Key::A, 1)?;
    println!("掩码设置结果: {}", mask_result);

    let is_masked = keyboard.is_key_masked(Key::A)?;
    println!("A键是否被掩码: {}", is_masked);

    println!("\n=== 示例完成 ===");
    Ok(())
}
