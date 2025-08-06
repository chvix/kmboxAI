//! 图像处理模块
//!
//! 提供图像捕获、处理和格式转换功能
//! 完整包装了common.h.rs中的所有图像相关结构体和函数

use crate::error::{KmboxError, KmboxResult};

// 导入图像处理相关的C函数
unsafe extern "C" {
    // 图像捕获函数
    #[link_name = "\u{1}_Z15capture_screen_v"]
    fn capture_screen_v() -> *mut ImageBufferT;

    // 图像加载和保存函数
    #[link_name = "\u{1}_Z12load_image_vPKc"]
    fn load_image_v(path: *const std::os::raw::c_char) -> *mut ImageBufferT;

    #[link_name = "\u{1}_Z12save_image_vPKcP15image_buffer_t"]
    fn save_image_v(
        path: *const std::os::raw::c_char,
        buffer: *const ImageBufferT,
    ) -> std::os::raw::c_int;

    // 图像格式转换函数
    #[link_name = "\u{1}_Z18convert_image_formatP15image_buffer_tS0_"]
    fn convert_image_format(
        src: *const ImageBufferT,
        dst: *mut ImageBufferT,
    ) -> std::os::raw::c_int;

    // 图像裁剪和缩放函数
    #[link_name = "\u{1}_Z12crop_image_vP15image_buffer_tPK11image_rect_t"]
    fn crop_image_v(src: *const ImageBufferT, rect: *const ImageRectT) -> *mut ImageBufferT;

    #[link_name = "\u{1}_Z13resize_image_vP15image_buffer_tii"]
    fn resize_image_v(
        src: *const ImageBufferT,
        new_width: std::os::raw::c_int,
        new_height: std::os::raw::c_int,
    ) -> *mut ImageBufferT;

    // 内存管理函数
    #[link_name = "\u{1}_Z15release_image_vP15image_buffer_t"]
    fn release_image_v(buffer: *mut ImageBufferT);
}

// 导入C结构体定义
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImageBufferT {
    pub width: std::os::raw::c_int,
    pub height: std::os::raw::c_int,
    pub width_stride: std::os::raw::c_int,
    pub height_stride: std::os::raw::c_int,
    pub format: ImageFormatT,
    pub virt_addr: *mut std::os::raw::c_uchar,
    pub size: std::os::raw::c_int,
    pub fd: std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImageRectT {
    pub left: std::os::raw::c_int,
    pub top: std::os::raw::c_int,
    pub right: std::os::raw::c_int,
    pub bottom: std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImageObbBoxT {
    pub x: std::os::raw::c_int,
    pub y: std::os::raw::c_int,
    pub w: std::os::raw::c_int,
    pub h: std::os::raw::c_int,
    pub angle: f32,
}

pub type ImageFormatT = std::os::raw::c_uint;

/// 图像格式枚举 - 与C定义一致
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    /// 8位灰度图像
    Gray8 = 0,
    /// RGB888格式
    Rgb888 = 1,
    /// RGBA8888格式
    Rgba8888 = 2,
    /// YUV420SP NV21格式
    Yuv420spNv21 = 3,
    /// YUV420SP NV12格式
    Yuv420spNv12 = 4,
    /// BGR888格式
    Bgr888 = 5,
}

// 图像格式转换实现
impl ImageFormat {
    /// 获取格式的数值表示
    pub fn as_u32(&self) -> u32 {
        match self {
            ImageFormat::Gray8 => 0,
            ImageFormat::Rgb888 => 1,
            ImageFormat::Rgba8888 => 2,
            ImageFormat::Yuv420spNv21 => 3,
            ImageFormat::Yuv420spNv12 => 4,
            ImageFormat::Bgr888 => 5,
        }
    }
}

/// 图像矩形区域
#[derive(Debug, Clone, Copy)]
pub struct ImageRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl ImageRect {
    /// 创建新的图像矩形
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    /// 获取矩形宽度
    pub fn width(&self) -> i32 {
        self.right - self.left
    }

    /// 获取矩形高度
    pub fn height(&self) -> i32 {
        self.bottom - self.top
    }
}

/// 图像数据结构
pub struct Image {
    pub width: i32,
    pub height: i32,
    pub format: ImageFormat,
    pub data: Vec<u8>,
}

impl Image {
    /// 创建新的图像
    pub fn new(width: i32, height: i32, format: ImageFormat) -> Self {
        let size = Self::calculate_size(width, height, format);
        Self {
            width,
            height,
            format,
            data: vec![0; size],
        }
    }

    /// 计算图像数据大小
    fn calculate_size(width: i32, height: i32, format: ImageFormat) -> usize {
        let pixel_size = match format {
            ImageFormat::Gray8 => 1,
            ImageFormat::Rgb888 | ImageFormat::Bgr888 => 3,
            ImageFormat::Rgba8888 => 4,
            ImageFormat::Yuv420spNv21 | ImageFormat::Yuv420spNv12 => {
                // YUV420格式：Y分量 + UV分量
                (width * height + width * height / 2) as usize
            }
        };
        ((width * height) as usize) * pixel_size
    }

    /// 获取图像数据
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// 获取图像数据的可变引用
    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// 获取图像尺寸
    pub fn size(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    /// 获取图像格式
    pub fn format(&self) -> ImageFormat {
        self.format
    }

    /// 从C缓冲区创建图像
    pub fn from_c_buffer(buffer: &ImageBufferT) -> Self {
        let format = match buffer.format {
            0 => ImageFormat::Gray8,
            1 => ImageFormat::Rgb888,
            2 => ImageFormat::Rgba8888,
            3 => ImageFormat::Yuv420spNv21,
            4 => ImageFormat::Yuv420spNv12,
            5 => ImageFormat::Bgr888,
            _ => ImageFormat::Rgb888, // 默认格式
        };

        let mut data = Vec::new();
        if !buffer.virt_addr.is_null() {
            unsafe {
                let data_slice = std::slice::from_raw_parts(buffer.virt_addr, buffer.size as usize);
                data.extend_from_slice(data_slice);
            }
        }

        Self {
            width: buffer.width,
            height: buffer.height,
            format,
            data,
        }
    }

    /// 转换为C缓冲区
    pub fn to_c_buffer(&self) -> ImageBufferT {
        // 注意：这里只是创建结构体，实际的内存管理需要外部处理
        ImageBufferT {
            width: self.width,
            height: self.height,
            width_stride: self.width,
            height_stride: self.height,
            format: self.format.as_u32(),
            virt_addr: std::ptr::null_mut(),
            size: self.data.len() as i32,
            fd: -1,
        }
    }
}

/// 检测结果
#[derive(Debug, Clone)]
pub struct DetectionResult {
    pub class_id: i32,
    pub confidence: f32,
    pub bounding_box: ImageRect,
}

/// 图像处理器
pub struct ImageProcessor {
    initialized: bool,
}

impl ImageProcessor {
    /// 创建新的图像处理器
    pub fn new() -> KmboxResult<Self> {
        Ok(Self { initialized: true })
    }

    /// 捕获屏幕图像
    pub fn capture_screen(&self) -> KmboxResult<Image> {
        if !self.initialized {
            return Err(KmboxError::InitializationError(
                "图像处理器未初始化".to_string(),
            ));
        }

        unsafe {
            let buffer_ptr = capture_screen_v();
            if buffer_ptr.is_null() {
                return Err(KmboxError::ImageError("屏幕捕获失败".to_string()));
            }

            let buffer = *buffer_ptr;
            let image = Image::from_c_buffer(&buffer);

            // 释放C缓冲区
            release_image_v(buffer_ptr);

            Ok(image)
        }
    }

    /// 从文件加载图像
    pub fn load_from_file(&self, path: &str) -> KmboxResult<Image> {
        if !self.initialized {
            return Err(KmboxError::InitializationError(
                "图像处理器未初始化".to_string(),
            ));
        }

        let path_cstr = std::ffi::CString::new(path)
            .map_err(|e| KmboxError::ParameterError(format!("无效的文件路径: {}", e)))?;

        unsafe {
            let buffer_ptr = load_image_v(path_cstr.as_ptr());
            if buffer_ptr.is_null() {
                return Err(KmboxError::ImageError(format!(
                    "无法加载图像文件: {}",
                    path
                )));
            }

            let buffer = *buffer_ptr;
            let image = Image::from_c_buffer(&buffer);

            // 释放C缓冲区
            release_image_v(buffer_ptr);

            Ok(image)
        }
    }

    /// 保存图像到文件
    pub fn save_to_file(&self, image: &Image, path: &str) -> KmboxResult<()> {
        if !self.initialized {
            return Err(KmboxError::InitializationError(
                "图像处理器未初始化".to_string(),
            ));
        }

        let path_cstr = std::ffi::CString::new(path)
            .map_err(|e| KmboxError::ParameterError(format!("无效的文件路径: {}", e)))?;

        let buffer = image.to_c_buffer();

        unsafe {
            let result = save_image_v(path_cstr.as_ptr(), &buffer);
            if result != 0 {
                return Err(KmboxError::ImageError(format!(
                    "保存图像文件失败: {}",
                    path
                )));
            }
        }

        Ok(())
    }

    /// 转换图像格式
    pub fn convert_format(&self, image: &Image, target_format: ImageFormat) -> KmboxResult<Image> {
        if !self.initialized {
            return Err(KmboxError::InitializationError(
                "图像处理器未初始化".to_string(),
            ));
        }

        let src_buffer = image.to_c_buffer();
        let mut dst_buffer = ImageBufferT {
            width: image.width,
            height: image.height,
            width_stride: image.width,
            height_stride: image.height,
            format: target_format.as_u32(),
            virt_addr: std::ptr::null_mut(),
            size: 0,
            fd: -1,
        };

        unsafe {
            let result = convert_image_format(&src_buffer, &mut dst_buffer);
            if result != 0 {
                return Err(KmboxError::ImageError("图像格式转换失败".to_string()));
            }

            let converted_image = Image::from_c_buffer(&dst_buffer);
            Ok(converted_image)
        }
    }

    /// 裁剪图像
    pub fn crop(&self, image: &Image, rect: ImageRect) -> KmboxResult<Image> {
        if !self.initialized {
            return Err(KmboxError::InitializationError(
                "图像处理器未初始化".to_string(),
            ));
        }

        let src_buffer = image.to_c_buffer();
        let c_rect = ImageRectT {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        };

        unsafe {
            let buffer_ptr = crop_image_v(&src_buffer, &c_rect);
            if buffer_ptr.is_null() {
                return Err(KmboxError::ImageError("图像裁剪失败".to_string()));
            }

            let buffer = *buffer_ptr;
            let cropped_image = Image::from_c_buffer(&buffer);

            // 释放C缓冲区
            release_image_v(buffer_ptr);

            Ok(cropped_image)
        }
    }

    /// 调整图像大小
    pub fn resize(&self, image: &Image, new_width: i32, new_height: i32) -> KmboxResult<Image> {
        if !self.initialized {
            return Err(KmboxError::InitializationError(
                "图像处理器未初始化".to_string(),
            ));
        }

        let src_buffer = image.to_c_buffer();

        unsafe {
            let buffer_ptr = resize_image_v(&src_buffer, new_width, new_height);
            if buffer_ptr.is_null() {
                return Err(KmboxError::ImageError("图像缩放失败".to_string()));
            }

            let buffer = *buffer_ptr;
            let resized_image = Image::from_c_buffer(&buffer);

            // 释放C缓冲区
            release_image_v(buffer_ptr);

            Ok(resized_image)
        }
    }
}

impl Drop for ImageProcessor {
    fn drop(&mut self) {
        // 清理资源（如果需要）
    }
}
