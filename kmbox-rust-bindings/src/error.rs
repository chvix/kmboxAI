//! 错误处理模块
//!
//! 定义了 KmboxAI 库中使用的错误类型和结果类型

use std::error::Error as StdError;
use std::fmt;

/// KmboxAI 库的错误类型
#[derive(Debug)]
pub enum KmboxError {
    /// 初始化错误
    InitializationError(String),
    /// 设备连接错误
    DeviceError(String),
    /// 参数错误
    ParameterError(String),
    /// 内存分配错误
    MemoryError(String),
    /// 模型加载错误
    ModelError(String),
    /// 推理错误
    InferenceError(String),
    /// 键盘操作错误
    KeyboardError(String),
    /// 图像处理错误
    ImageError(String),
    /// 系统调用错误
    SystemError(String),
    /// 未知错误
    Unknown(String),
}

impl fmt::Display for KmboxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KmboxError::InitializationError(msg) => write!(f, "初始化错误: {}", msg),
            KmboxError::DeviceError(msg) => write!(f, "设备错误: {}", msg),
            KmboxError::ParameterError(msg) => write!(f, "参数错误: {}", msg),
            KmboxError::MemoryError(msg) => write!(f, "内存错误: {}", msg),
            KmboxError::ModelError(msg) => write!(f, "模型错误: {}", msg),
            KmboxError::InferenceError(msg) => write!(f, "推理错误: {}", msg),
            KmboxError::KeyboardError(msg) => write!(f, "键盘错误: {}", msg),
            KmboxError::ImageError(msg) => write!(f, "图像错误: {}", msg),
            KmboxError::SystemError(msg) => write!(f, "系统错误: {}", msg),
            KmboxError::Unknown(msg) => write!(f, "未知错误: {}", msg),
        }
    }
}

impl StdError for KmboxError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

impl From<std::io::Error> for KmboxError {
    fn from(err: std::io::Error) -> Self {
        KmboxError::SystemError(err.to_string())
    }
}

impl From<std::ffi::NulError> for KmboxError {
    fn from(err: std::ffi::NulError) -> Self {
        KmboxError::ParameterError(err.to_string())
    }
}

impl From<std::string::FromUtf8Error> for KmboxError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        KmboxError::ParameterError(err.to_string())
    }
}

/// KmboxAI 库的结果类型
pub type KmboxResult<T> = Result<T, KmboxError>;

/// 检查 C 函数返回值的辅助函数
pub(crate) fn check_result(result: i32, operation: &str) -> KmboxResult<()> {
    if result == 0 {
        Ok(())
    } else {
        Err(KmboxError::SystemError(format!(
            "{} 失败，错误码: {}",
            operation, result
        )))
    }
}

/// 检查指针是否为空的辅助函数
#[allow(dead_code)]
pub(crate) fn check_ptr<T>(ptr: *const T, operation: &str) -> KmboxResult<()> {
    if ptr.is_null() {
        Err(KmboxError::SystemError(format!(
            "{} 失败：空指针",
            operation
        )))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kmbox_error_display() {
        let error = KmboxError::InitializationError("测试错误".to_string());
        assert!(error.to_string().contains("初始化错误"));
        assert!(error.to_string().contains("测试错误"));
    }

    #[test]
    fn test_kmbox_error_variants() {
        let errors = vec![
            KmboxError::InitializationError("init".to_string()),
            KmboxError::DeviceError("device".to_string()),
            KmboxError::ParameterError("param".to_string()),
            KmboxError::MemoryError("memory".to_string()),
            KmboxError::ModelError("model".to_string()),
            KmboxError::InferenceError("inference".to_string()),
            KmboxError::KeyboardError("keyboard".to_string()),
            KmboxError::ImageError("image".to_string()),
            KmboxError::SystemError("system".to_string()),
            KmboxError::Unknown("unknown".to_string()),
        ];

        for error in errors {
            assert!(!error.to_string().is_empty());
        }
    }

    #[test]
    fn test_check_result() {
        assert!(check_result(0, "测试操作").is_ok());
        assert!(check_result(1, "测试操作").is_err());
    }

    #[test]
    fn test_check_ptr() {
        let ptr: *const i32 = std::ptr::null();
        assert!(check_ptr(ptr, "空指针测试").is_err());

        let value = 42;
        let ptr: *const i32 = &value;
        assert!(check_ptr(ptr, "有效指针测试").is_ok());
    }

    #[test]
    fn test_error_conversions() {
        // 测试 From 实现
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "文件未找到");
        let kmbox_error: KmboxError = io_error.into();
        assert!(matches!(kmbox_error, KmboxError::SystemError(_)));
    }
}
