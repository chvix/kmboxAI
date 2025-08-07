# KmboxAI Rust 封装库

这是一个为 KmboxAI 硬件设备提供的 Rust 封装库，提供了完整的硬件控制功能。

## 功能特性

### 🎯 核心功能 (kmbox_ai)
- **系统控制**: 初始化、运行、启用/禁用
- **鼠标操作**: 移动、点击、滚轮、多按键支持
- **键盘操作**: 按键按下、释放、点击
- **输入监控**: 实时监控鼠标和键盘状态
- **输入掩码**: 精确控制输入设备的掩码
- **MiniUI控制**: 界面锁定、LCD显示
- **YOLO模型**: 模型加载、接口、绘制功能
- **系统操作**: 重启等系统级功能

### ⌨️ 键盘模块 (keyboard)
- 支持所有标准键盘按键（200+个常量）
- 按键按下、释放、点击操作
- 按键状态检测
- 字符串输入功能
- 按键掩码设置
- 按键表显示

### 🖼️ 图像处理模块 (vision)
- 屏幕截图捕获
- 图像文件加载和保存
- 图像格式转换
- 图像裁剪和缩放
- 多种图像格式支持（RGB、RGBA、YUV等）

### 🎯 YOLO目标检测模块 (yolo)
- 支持80+种目标类型
- 实时目标检测
- 置信度阈值设置
- NMS阈值配置
- 边界框检测结果

### 🧠 RKNN神经网络推理模块 (rknn)
- 模型加载和管理
- 张量操作
- 推理执行
- 性能分析

## 安装

```bash
# 克隆项目
git clone <repository-url>
cd kmbox-rust-bindings

# 检查编译
cargo check

# 运行测试
cargo test

# 生成文档
cargo doc --open
```

## 快速开始

### 基本使用

```rust
use kmbox_ai_rust::{
    kmbox_ai::{KmboxAI, MouseButton, MouseAction},
    keyboard::{Keyboard, Key},
    vision::{ImageProcessor, Image, ImageFormat},
    yolo::{YoloDetector, ObjectType},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建 KmboxAI 实例
    let mut kmbox = KmboxAI::new()?;
    kmbox.init()?;
    kmbox.enable(true)?;

    // 鼠标操作
    kmbox.mouse_move(500, 300)?;
    kmbox.mouse_button(MouseButton::Left, MouseAction::Click)?;

    // 键盘操作
    let keyboard = Keyboard::new()?;
    keyboard.press_key(Key::A)?;
    keyboard.release_key(Key::A)?;
    keyboard.type_string("Hello, World!")?;

    // 图像处理
    let processor = ImageProcessor::new()?;
    let image = processor.capture_screen()?;
    processor.save_to_file(&image, "screenshot.png")?;

    // 目标检测
    let detector = YoloDetector::new("model.rknn")?;
    detector.set_confidence_threshold(0.5)?;
    let results = detector.detect(&image)?;

    Ok(())
}
```

### 核心功能示例

```rust
use kmbox_ai_rust::kmbox_ai::{KmboxAI, MouseButton, MouseAction, MiniUIMode};

fn kmbox_ai_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut kmbox = KmboxAI::new()?;
    
    // 系统初始化
    kmbox.init()?;
    kmbox.enable(true)?;
    
    // 鼠标操作
    kmbox.mouse_move(100, 200)?;
    kmbox.mouse_button(MouseButton::Left, MouseAction::Click)?;
    kmbox.mouse_wheel(120)?;
    
    // 键盘操作
    kmbox.key_init()?;
    kmbox.key_click(65)?; // 按下 'A' 键
    
    // 输入监控
    let position = kmbox.monitor_mouse_position()?;
    let mouse_state = kmbox.get_mouse_state()?;
    
    // 输入掩码
    kmbox.mask_mouse_left(true)?;
    kmbox.mask_keyboard(65)?;
    kmbox.unmask_all()?;
    
    // MiniUI控制
    kmbox.miniui_enable(MiniUIMode::Enabled)?;
    kmbox.miniui_user_lock()?;
    
    // YOLO模型
    kmbox.yolo_load_model("model.rknn")?;
    kmbox.yolo_interface_model()?;
    kmbox.yolo_draw_rectangle(100, 100, 200, 150, 0xFF0000, 2)?;
    
    Ok(())
}
```

### 键盘操作示例

```rust
use kmbox_ai_rust::keyboard::{Keyboard, Key};

fn keyboard_example() -> Result<(), Box<dyn std::error::Error>> {
    let keyboard = Keyboard::new()?;
    
    // 基本按键操作
    keyboard.press_key(Key::A)?;
    keyboard.release_key(Key::A)?;
    keyboard.click_key(Key::Enter, 100)?;
    
    // 字符串输入
    keyboard.type_string("Hello, World!")?;
    
    // 按键状态检测
    let is_pressed = keyboard.is_key_pressed(Key::A)?;
    
    // 按键掩码
    keyboard.set_key_mask(Key::A, 1)?;
    let is_masked = keyboard.is_key_masked(Key::A)?;
    
    Ok(())
}
```

### 图像处理示例

```rust
use kmbox_ai_rust::vision::{ImageProcessor, Image, ImageFormat, ImageRect};

fn vision_example() -> Result<(), Box<dyn std::error::Error>> {
    let processor = ImageProcessor::new()?;
    
    // 屏幕截图
    let image = processor.capture_screen()?;
    
    // 保存图像
    processor.save_to_file(&image, "screenshot.png")?;
    
    // 加载图像
    let loaded_image = processor.load_from_file("image.jpg")?;
    
    // 格式转换
    let converted = processor.convert_format(&loaded_image, ImageFormat::Rgb888)?;
    
    // 图像裁剪
    let rect = ImageRect::new(100, 100, 300, 300);
    let cropped = processor.crop(&image, rect)?;
    
    // 图像缩放
    let resized = processor.resize(&image, 800, 600)?;
    
    Ok(())
}
```

### YOLO检测示例

```rust
use kmbox_ai_rust::yolo::{YoloDetector, ObjectType};

fn yolo_example() -> Result<(), Box<dyn std::error::Error>> {
    let detector = YoloDetector::new("model.rknn")?;
    
    // 设置检测参数
    detector.set_confidence_threshold(0.5)?;
    detector.set_nms_threshold(0.4)?;
    
    // 执行检测
    let image = Image::new(640, 480, ImageFormat::Rgb888);
    let results = detector.detect(&image)?;
    
    // 处理检测结果
    for obj in results.objects {
        println!("检测到: {:?}, 置信度: {}", obj.object_type, obj.confidence);
        println!("边界框: ({}, {}, {}, {})", 
                obj.bounding_box.left, obj.bounding_box.top,
                obj.bounding_box.right, obj.bounding_box.bottom);
    }
    
    Ok(())
}
```

## 模块详细说明

### KmboxAI 核心模块

提供 KmboxAI 硬件的核心控制功能：

- **系统控制**: `init()`, `run()`, `enable()`, `version()`
- **鼠标操作**: `mouse_move()`, `mouse_button()`, `mouse_wheel()`, `mouse_all()`
- **键盘操作**: `key_down()`, `key_up()`, `key_click()`, `key_init()`
- **输入监控**: `monitor_mouse_*()`, `monitor_keyboard()`, `get_mouse_state()`
- **输入掩码**: `mask_mouse_*()`, `mask_keyboard()`, `unmask_*()`
- **MiniUI控制**: `miniui_enable()`, `miniui_user_lock()`, `miniui_lcd_display()`
- **YOLO模型**: `yolo_load_model()`, `yolo_interface_model()`, `yolo_draw_*()`

### 键盘模块

提供完整的键盘操作功能：

- **按键枚举**: 支持所有标准键盘按键（A-Z, 0-9, 功能键等）
- **按键操作**: 按下、释放、点击、字符串输入
- **状态检测**: 检测按键是否按下
- **掩码控制**: 设置和检测按键掩码

### 图像处理模块

提供图像处理和分析功能：

- **屏幕捕获**: 实时屏幕截图
- **文件操作**: 加载和保存图像文件
- **格式转换**: 支持多种图像格式
- **图像处理**: 裁剪、缩放、格式转换

### YOLO检测模块

提供目标检测功能：

- **模型管理**: 加载和释放YOLO模型
- **目标检测**: 实时目标检测
- **结果处理**: 边界框、置信度、目标类型
- **参数配置**: 置信度阈值、NMS阈值

### RKNN推理模块

提供神经网络推理功能：

- **模型管理**: 加载和管理RKNN模型
- **张量操作**: 输入输出张量处理
- **推理执行**: 模型推理
- **性能分析**: 推理性能监控

## 错误处理

所有函数都返回 `KmboxResult<T>` 类型，提供统一的错误处理：

```rust
use kmbox_ai_rust::{KmboxError, KmboxResult};

fn example() -> KmboxResult<()> {
    let kmbox = KmboxAI::new()?;
    kmbox.init()?;
    kmbox.mouse_move(100, 200)?;
    Ok(())
}
```

错误类型包括：
- `InitializationError`: 初始化错误
- `DeviceError`: 设备连接错误
- `ParameterError`: 参数错误
- `MemoryError`: 内存分配错误
- `ModelError`: 模型加载错误
- `InferenceError`: 推理错误
- `KeyboardError`: 键盘操作错误
- `ImageError`: 图像处理错误
- `SystemError`: 系统调用错误
- `Unknown`: 未知错误

## 系统要求

- **操作系统**: Linux
- **硬件**: KmboxAI 硬件设备
- **依赖库**: 相应的动态库文件
- **Rust版本**: 1.70+

## 示例程序

项目包含多个示例程序：

```bash
# 运行核心功能示例
cargo run --example kmbox_ai_example

# 运行键盘操作示例
cargo run --example keyboard_example
```

## 测试

```bash
# 运行所有测试
cargo test

# 运行特定模块测试
cargo test keyboard
cargo test vision
cargo test yolo
cargo test kmbox_ai
```

## 文档

```bash
# 生成文档
cargo doc --open

# 生成文档（包含私有项）
cargo doc --document-private-items --open
```

## 许可证

本项目遵循相应的开源许可证。

## 贡献

欢迎提交 Issue 和 Pull Request！

## 更新日志

### v0.1.0
- ✅ 完整的 KmboxAI 核心功能封装
- ✅ 键盘操作模块
- ✅ 图像处理模块
- ✅ YOLO目标检测模块
- ✅ RKNN推理模块框架
- ✅ 统一的错误处理
- ✅ 完整的文档和示例
