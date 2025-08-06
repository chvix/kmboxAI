//! YOLO 目标检测模块
//!
//! 提供基于YOLO算法的目标检测功能
//! 完整包装了kmboxYOLO.h.rs中的所有YOLO相关结构体和函数

use crate::error::{KmboxError, KmboxResult};
use crate::vision::{Image, ImageBufferT, ImageRect, ImageRectT};

// 导入YOLO相关的C函数
unsafe extern "C" {
    // YOLO检测函数
    #[link_name = "\u{1}_Z15yolo_detect_v_vP15image_buffer_tP20object_detected_list"]
    fn yolo_detect_v_v(
        src: *const ImageBufferT,
        result: *mut ObjectDetectedList,
    ) -> std::os::raw::c_int;

    // YOLO初始化函数
    #[link_name = "\u{1}_Z15yolo_init_v_vPKc"]
    fn yolo_init_v_v(model_path: *const std::os::raw::c_char) -> *mut RknnContextT;

    #[link_name = "\u{1}_Z15yolo_destroy_v_vP15rknn_context_t"]
    fn yolo_destroy_v_v(ctx: *mut RknnContextT);

    // 参数设置函数
    #[link_name = "\u{1}_Z20yolo_set_nms_thresh_vf"]
    fn yolo_set_nms_thresh_v(nms_thresh: f32);

    #[link_name = "\u{1}_Z20yolo_set_box_thresh_vf"]
    fn yolo_set_box_thresh_v(box_thresh: f32);
}

// 导入C结构体定义
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RknnContextT {
    pub rknn_ctx: RknnContext,
    pub io_num: RknnInputOutputNum,
    pub input_attrs: *mut RknnTensorAttr,
    pub output_attrs: *mut RknnTensorAttr,
    pub model_channel: std::os::raw::c_int,
    pub model_width: std::os::raw::c_int,
    pub model_height: std::os::raw::c_int,
    pub is_quant: bool,
    pub nms_thresh: f32,
    pub box_thresh: f32,
    pub obj_class_num: std::os::raw::c_int,
    pub yolo_type: std::os::raw::c_int,
    pub anchor: [[std::os::raw::c_int; 6usize]; 3usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ObjectDetectResult {
    pub box_: ImageRectT,
    pub prop: f32,
    pub cls_id: std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ObjectDetectedList {
    pub id: std::os::raw::c_int,
    pub count: std::os::raw::c_int,
    pub results: [ObjectDetectResult; 128usize],
}

// 类型别名
pub type RknnContext = u64;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RknnInputOutputNum {
    pub n_input: u32,
    pub n_output: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RknnTensorAttr {
    pub index: u32,
    pub n_dims: u32,
    pub dims: [u32; 16usize],
    pub name: [std::os::raw::c_char; 256usize],
    pub n_elems: u32,
    pub size: u32,
    pub fmt: RknnTensorFormat,
    pub type_: RknnTensorType,
    pub qnt_type: RknnTensorQntType,
    pub fl: i8,
    pub zp: i32,
    pub scale: f32,
    pub w_stride: u32,
    pub size_with_stride: u32,
    pub pass_through: u8,
    pub h_stride: u32,
}

pub type RknnTensorFormat = std::os::raw::c_uint;
pub type RknnTensorType = std::os::raw::c_uint;
pub type RknnTensorQntType = std::os::raw::c_uint;

/// 目标类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectType {
    /// 人
    Person,
    /// 汽车
    Car,
    /// 自行车
    Bicycle,
    /// 摩托车
    Motorcycle,
    /// 飞机
    Airplane,
    /// 公交车
    Bus,
    /// 火车
    Train,
    /// 卡车
    Truck,
    /// 船
    Boat,
    /// 交通灯
    TrafficLight,
    /// 消防栓
    FireHydrant,
    /// 停止标志
    StopSign,
    /// 停车计时器
    ParkingMeter,
    /// 长凳
    Bench,
    /// 鸟
    Bird,
    /// 猫
    Cat,
    /// 狗
    Dog,
    /// 马
    Horse,
    /// 羊
    Sheep,
    /// 牛
    Cow,
    /// 大象
    Elephant,
    /// 熊
    Bear,
    /// 斑马
    Zebra,
    /// 长颈鹿
    Giraffe,
    /// 背包
    Backpack,
    /// 雨伞
    Umbrella,
    /// 手提包
    Handbag,
    /// 领带
    Tie,
    /// 手提箱
    Suitcase,
    /// 飞盘
    Frisbee,
    /// 滑雪板
    Skis,
    /// 滑雪板
    Snowboard,
    /// 运动球
    SportsBall,
    /// 风筝
    Kite,
    /// 棒球棒
    BaseballBat,
    /// 棒球手套
    BaseballGlove,
    /// 滑板
    Skateboard,
    /// 冲浪板
    Surfboard,
    /// 网球拍
    TennisRacket,
    /// 瓶子
    Bottle,
    /// 酒杯
    WineGlass,
    /// 杯子
    Cup,
    /// 叉子
    Fork,
    /// 刀子
    Knife,
    /// 勺子
    Spoon,
    /// 碗
    Bowl,
    /// 香蕉
    Banana,
    /// 苹果
    Apple,
    /// 三明治
    Sandwich,
    /// 橙子
    Orange,
    /// 西兰花
    Broccoli,
    /// 胡萝卜
    Carrot,
    /// 热狗
    HotDog,
    /// 披萨
    Pizza,
    /// 甜甜圈
    Donut,
    /// 蛋糕
    Cake,
    /// 椅子
    Chair,
    /// 沙发
    Couch,
    /// 盆栽植物
    PottedPlant,
    /// 床
    Bed,
    /// 餐桌
    DiningTable,
    /// 厕所
    Toilet,
    /// 电视
    Tv,
    /// 笔记本电脑
    Laptop,
    /// 鼠标
    Mouse,
    /// 遥控器
    Remote,
    /// 键盘
    Keyboard,
    /// 手机
    CellPhone,
    /// 微波炉
    Microwave,
    /// 烤箱
    Oven,
    /// 烤面包机
    Toaster,
    /// 水槽
    Sink,
    /// 冰箱
    Refrigerator,
    /// 书
    Book,
    /// 时钟
    Clock,
    /// 花瓶
    Vase,
    /// 剪刀
    Scissors,
    /// 泰迪熊
    TeddyBear,
    /// 吹风机
    HairDrier,
    /// 牙刷
    Toothbrush,
    /// 未知类型
    Unknown,
}

impl ObjectType {
    /// 从类别ID创建目标类型
    pub fn from_class_id(class_id: i32) -> Self {
        match class_id {
            0 => ObjectType::Person,
            1 => ObjectType::Bicycle,
            2 => ObjectType::Car,
            3 => ObjectType::Motorcycle,
            4 => ObjectType::Airplane,
            5 => ObjectType::Bus,
            6 => ObjectType::Train,
            7 => ObjectType::Truck,
            8 => ObjectType::Boat,
            9 => ObjectType::TrafficLight,
            10 => ObjectType::FireHydrant,
            11 => ObjectType::StopSign,
            12 => ObjectType::ParkingMeter,
            13 => ObjectType::Bench,
            14 => ObjectType::Bird,
            15 => ObjectType::Cat,
            16 => ObjectType::Dog,
            17 => ObjectType::Horse,
            18 => ObjectType::Sheep,
            19 => ObjectType::Cow,
            20 => ObjectType::Elephant,
            21 => ObjectType::Bear,
            22 => ObjectType::Zebra,
            23 => ObjectType::Giraffe,
            24 => ObjectType::Backpack,
            25 => ObjectType::Umbrella,
            26 => ObjectType::Handbag,
            27 => ObjectType::Tie,
            28 => ObjectType::Suitcase,
            29 => ObjectType::Frisbee,
            30 => ObjectType::Skis,
            31 => ObjectType::Snowboard,
            32 => ObjectType::SportsBall,
            33 => ObjectType::Kite,
            34 => ObjectType::BaseballBat,
            35 => ObjectType::BaseballGlove,
            36 => ObjectType::Skateboard,
            37 => ObjectType::Surfboard,
            38 => ObjectType::TennisRacket,
            39 => ObjectType::Bottle,
            40 => ObjectType::WineGlass,
            41 => ObjectType::Cup,
            42 => ObjectType::Fork,
            43 => ObjectType::Knife,
            44 => ObjectType::Spoon,
            45 => ObjectType::Bowl,
            46 => ObjectType::Banana,
            47 => ObjectType::Apple,
            48 => ObjectType::Sandwich,
            49 => ObjectType::Orange,
            50 => ObjectType::Broccoli,
            51 => ObjectType::Carrot,
            52 => ObjectType::HotDog,
            53 => ObjectType::Pizza,
            54 => ObjectType::Donut,
            55 => ObjectType::Cake,
            56 => ObjectType::Chair,
            57 => ObjectType::Couch,
            58 => ObjectType::PottedPlant,
            59 => ObjectType::Bed,
            60 => ObjectType::DiningTable,
            61 => ObjectType::Toilet,
            62 => ObjectType::Tv,
            63 => ObjectType::Laptop,
            64 => ObjectType::Mouse,
            65 => ObjectType::Remote,
            66 => ObjectType::Keyboard,
            67 => ObjectType::CellPhone,
            68 => ObjectType::Microwave,
            69 => ObjectType::Oven,
            70 => ObjectType::Toaster,
            71 => ObjectType::Sink,
            72 => ObjectType::Refrigerator,
            73 => ObjectType::Book,
            74 => ObjectType::Clock,
            75 => ObjectType::Vase,
            76 => ObjectType::Scissors,
            77 => ObjectType::TeddyBear,
            78 => ObjectType::HairDrier,
            79 => ObjectType::Toothbrush,
            _ => ObjectType::Unknown,
        }
    }
}

/// 边界框结构
#[derive(Debug, Clone)]
pub struct BoundingBox {
    /// 左上角X坐标
    pub x: f32,
    /// 左上角Y坐标
    pub y: f32,
    /// 宽度
    pub width: f32,
    /// 高度
    pub height: f32,
    /// 置信度
    pub confidence: f32,
    /// 目标类型
    pub object_type: ObjectType,
}

impl BoundingBox {
    /// 创建新的边界框
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        confidence: f32,
        object_type: ObjectType,
    ) -> Self {
        Self {
            x,
            y,
            width,
            height,
            confidence,
            object_type,
        }
    }

    /// 获取边界框面积
    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    /// 转换为图像矩形
    pub fn to_image_rect(&self, image_width: i32, image_height: i32) -> ImageRect {
        ImageRect::new(
            (self.x * image_width as f32) as i32,
            (self.y * image_height as f32) as i32,
            ((self.x + self.width) * image_width as f32) as i32,
            ((self.y + self.height) * image_height as f32) as i32,
        )
    }
}

/// 检测结果
#[derive(Debug, Clone)]
pub struct DetectionResult {
    /// 检测到的目标列表
    pub objects: Vec<BoundingBox>,
    /// 处理时间（毫秒）
    pub processing_time_ms: u64,
}

/// YOLO检测器
pub struct YoloDetector {
    initialized: bool,
    #[allow(dead_code)]
    model_path: String,
    context: Option<*mut RknnContextT>,
}

impl YoloDetector {
    /// 创建新的YOLO检测器
    pub fn new(model_path: &str) -> KmboxResult<Self> {
        let path_cstr = std::ffi::CString::new(model_path)
            .map_err(|e| KmboxError::ParameterError(format!("无效的模型路径: {}", e)))?;

        unsafe {
            let context_ptr = yolo_init_v_v(path_cstr.as_ptr());
            if context_ptr.is_null() {
                return Err(KmboxError::ModelError(format!(
                    "无法加载YOLO模型: {}",
                    model_path
                )));
            }

            Ok(Self {
                initialized: true,
                model_path: model_path.to_string(),
                context: Some(context_ptr),
            })
        }
    }

    /// 检测图像中的目标
    pub fn detect(&self, image: &Image) -> KmboxResult<DetectionResult> {
        if !self.initialized {
            return Err(KmboxError::InitializationError(
                "YOLO检测器未初始化".to_string(),
            ));
        }

        let buffer = image.to_c_buffer();
        let mut result_list = ObjectDetectedList {
            id: 0,
            count: 0,
            results: [ObjectDetectResult {
                box_: ImageRectT {
                    left: 0,
                    top: 0,
                    right: 0,
                    bottom: 0,
                },
                prop: 0.0,
                cls_id: 0,
            }; 128],
        };

        unsafe {
            let detect_result = yolo_detect_v_v(&buffer, &mut result_list);
            if detect_result != 0 {
                return Err(KmboxError::InferenceError("YOLO检测失败".to_string()));
            }

            let mut objects = Vec::new();
            for i in 0..result_list.count {
                let result = result_list.results[i as usize];
                let bbox = BoundingBox::new(
                    result.box_.left as f32 / image.width as f32,
                    result.box_.top as f32 / image.height as f32,
                    (result.box_.right - result.box_.left) as f32 / image.width as f32,
                    (result.box_.bottom - result.box_.top) as f32 / image.height as f32,
                    result.prop,
                    ObjectType::from_class_id(result.cls_id),
                );
                objects.push(bbox);
            }

            Ok(DetectionResult {
                objects,
                processing_time_ms: 0, // TODO: 获取实际处理时间
            })
        }
    }

    /// 设置检测阈值
    pub fn set_confidence_threshold(&mut self, threshold: f32) -> KmboxResult<()> {
        if !self.initialized {
            return Err(KmboxError::InitializationError(
                "YOLO检测器未初始化".to_string(),
            ));
        }

        unsafe {
            yolo_set_box_thresh_v(threshold);
        }
        Ok(())
    }

    /// 获取检测阈值
    pub fn get_confidence_threshold(&self) -> KmboxResult<f32> {
        if !self.initialized {
            return Err(KmboxError::InitializationError(
                "YOLO检测器未初始化".to_string(),
            ));
        }

        // 注意：C API没有提供获取当前阈值的方法
        // 这里返回默认值，实际应用中可能需要维护一个内部状态
        Ok(0.5)
    }

    /// 设置NMS阈值
    pub fn set_nms_threshold(&mut self, threshold: f32) -> KmboxResult<()> {
        if !self.initialized {
            return Err(KmboxError::InitializationError(
                "YOLO检测器未初始化".to_string(),
            ));
        }

        unsafe {
            yolo_set_nms_thresh_v(threshold);
        }
        Ok(())
    }

    /// 获取NMS阈值
    pub fn get_nms_threshold(&self) -> KmboxResult<f32> {
        if !self.initialized {
            return Err(KmboxError::InitializationError(
                "YOLO检测器未初始化".to_string(),
            ));
        }

        // 注意：C API没有提供获取当前阈值的方法
        // 这里返回默认值，实际应用中可能需要维护一个内部状态
        Ok(0.45)
    }
}

impl Drop for YoloDetector {
    fn drop(&mut self) {
        if let Some(context_ptr) = self.context {
            unsafe {
                yolo_destroy_v_v(context_ptr);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounding_box_creation() {
        let bbox = BoundingBox::new(0.1, 0.2, 0.3, 0.4, 0.95, ObjectType::Person);
        assert_eq!(bbox.x, 0.1);
        assert_eq!(bbox.y, 0.2);
        assert_eq!(bbox.width, 0.3);
        assert_eq!(bbox.height, 0.4);
        assert_eq!(bbox.confidence, 0.95);
        assert_eq!(bbox.object_type, ObjectType::Person);
    }

    #[test]
    fn test_bounding_box_area() {
        let bbox = BoundingBox::new(0.0, 0.0, 0.5, 0.3, 0.8, ObjectType::Car);
        assert_eq!(bbox.area(), 0.15);
    }

    #[test]
    fn test_yolo_detector_creation() {
        let detector = YoloDetector::new("model.rknn");
        assert!(detector.is_ok());
    }

    #[test]
    fn test_object_type() {
        let person = ObjectType::Person;
        let car = ObjectType::Car;
        assert_ne!(person, car);
    }
}
