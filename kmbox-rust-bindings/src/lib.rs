//! KmboxAI Rust 封装库
//!
//! 这是一个为 KmboxAI 硬件设备提供的 Rust 封装库，提供了以下功能：
//! - 键盘模拟和按键检测
//! - 图像处理和计算机视觉
//! - YOLO 目标检测
//! - RKNN 神经网络推理
//!
//! ## 功能特性
//!
//! ### 键盘模块
//! - 支持所有标准键盘按键
//! - 按键按下、释放、点击操作
//! - 按键状态检测
//! - 字符串输入功能
//! - 按键掩码设置
//!
//! ### 图像处理模块
//! - 屏幕截图捕获
//! - 图像文件加载和保存
//! - 图像格式转换
//! - 图像裁剪和缩放
//! - 多种图像格式支持
//!
//! ### YOLO目标检测模块
//! - 支持80+种目标类型
//! - 实时目标检测
//! - 置信度阈值设置
//! - NMS阈值配置
//! - 边界框检测结果
//!
//! ### RKNN神经网络推理模块
//! - 模型加载和管理
//! - 张量操作
//! - 推理执行
//! - 性能分析
//!
//! ## 使用示例
//!
//! ```rust
//! use kmbox_ai_rust::{
//!     keyboard::{Keyboard, Key},
//!     vision::{ImageProcessor, Image, ImageFormat},
//!     yolo::{YoloDetector, ObjectType, BoundingBox},
//!     rknn::{RknnContext, RknnModel},
//! };
//!
//! // 键盘操作
//! let keyboard = Keyboard::new()?;
//! keyboard.press_key(Key::A)?;
//! keyboard.release_key(Key::A)?;
//! keyboard.click_key(Key::Enter)?;
//! keyboard.type_string("Hello, World!")?;
//!
//! // 图像处理
//! let processor = ImageProcessor::new()?;
//! let image = processor.capture_screen()?;
//! let cropped = processor.crop(&image, rect)?;
//! processor.save_to_file(&image, "screenshot.png")?;
//!
//! // 目标检测
//! let detector = YoloDetector::new("model.rknn")?;
//! detector.set_confidence_threshold(0.5)?;
//! let results = detector.detect(&image)?;
//! for obj in results.objects {
//!     println!("检测到: {:?}, 置信度: {}", obj.object_type, obj.confidence);
//! }
//! ```
//!
//! ## 错误处理
//!
//! 所有函数都返回 `KmboxResult<T>` 类型，提供统一的错误处理：
//!
//! ```rust
//! use kmbox_ai_rust::{KmboxError, KmboxResult};
//!
//! fn example() -> KmboxResult<()> {
//!     let keyboard = Keyboard::new()?;
//!     keyboard.press_key(Key::A)?;
//!     Ok(())
//! }
//! ```
//!
//! ## 系统要求
//!
//! - Linux 系统
//! - KmboxAI 硬件设备
//! - 相应的动态库文件
//!
//! ## 许可证
//!
//! 本项目遵循相应的开源许可证。

pub mod error;
pub mod keyboard;
pub mod rknn;
pub mod vision;
pub mod yolo;

// 重新导出常用的类型和函数
pub use error::{KmboxError, KmboxResult};
pub use keyboard::{Key, KeyState, Keyboard};
pub use rknn::{RknnContext, RknnModel};
pub use vision::{DetectionResult, Image, ImageFormat, ImageProcessor, ImageRect};
pub use yolo::{BoundingBox, ObjectType, YoloDetector};
