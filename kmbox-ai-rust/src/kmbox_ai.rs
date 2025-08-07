//! KmboxAI 核心模块
//!
//! 提供KmboxAI的核心功能，包括：
//! - 系统初始化和控制
//! - 鼠标操作和监控
//! - 键盘操作和监控
//! - 输入掩码控制
//! - MiniUI界面控制
//! - YOLO模型管理
//! - 系统重启
//!
//! ## 功能特性
//!
//! - 完整的鼠标控制（移动、点击、滚轮）
//! - 键盘按键操作
//! - 输入监控和掩码
//! - MiniUI界面控制
//! - YOLO模型加载和管理
//! - 系统级操作
//!
//! ## 使用示例
//!
//! ```rust
//! use kmbox_ai_rust::kmbox_ai::{KmboxAI, MouseButton, MouseAction};
//!
//! let kmbox = KmboxAI::new()?;
//! kmbox.init()?;
//! kmbox.mouse_move(100, 200)?;
//! kmbox.mouse_click(MouseButton::Left)?;
//! ```

use crate::error::{check_result, KmboxError, KmboxResult};
use std::ffi::CString;

// 导入所有kmAI函数
unsafe extern "C" {
    // 系统初始化和控制
    #[link_name = "\u{1}_Z7kmAI_Initv"]
    fn kmAI_Init() -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z7kmAI_Runv"]
    fn kmAI_Run() -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z11kmAI_kmEnablei"]
    fn kmAI_kmEnable(enable: std::os::raw::c_int) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z10kmAI_Versionv"]
    fn kmAI_Version() -> std::os::raw::c_int;

    // 鼠标操作
    #[link_name = "\u{1}_Z12kmAI_mouse_moveii"]
    fn kmAI_mouse_move(x: std::os::raw::c_int, y: std::os::raw::c_int) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z13kmAI_mouse_lefti"]
    fn kmAI_mouse_left(isdown: std::os::raw::c_int) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z14kmAI_mouse_righti"]
    fn kmAI_mouse_right(isdown: std::os::raw::c_int) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z15kmAI_mouse_middlei"]
    fn kmAI_mouse_middle(isdown: std::os::raw::c_int) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z14kmAI_mouse_wheeli"]
    fn kmAI_mouse_wheel(wheel: std::os::raw::c_int) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z15kmAI_mouse_side1i"]
    fn kmAI_mouse_side1(isdown: std::os::raw::c_int) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z15kmAI_mouse_side2i"]
    fn kmAI_mouse_side2(isdown: std::os::raw::c_int) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z13kmAI_mouse_alliiiiii"]
    fn kmAI_mouse_all(
        x: std::os::raw::c_int,
        y: std::os::raw::c_int,
        left: std::os::raw::c_int,
        right: std::os::raw::c_int,
        middle: std::os::raw::c_int,
        wheel: std::os::raw::c_int,
    ) -> std::os::raw::c_int;

    // 键盘操作
    #[link_name = "\u{1}_Z11kmAI_keydowni"]
    fn kmAI_keydown(vkey: std::os::raw::c_int) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z9kmAI_keyupi"]
    fn kmAI_keyup(vkey: std::os::raw::c_int) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z11kmAI_keyinitv"]
    fn kmAI_keyinit() -> std::os::raw::c_int;

    // 鼠标监控
    #[link_name = "\u{1}_Z22kmAI_monitor_mouse_leftv"]
    fn kmAI_monitor_mouse_left() -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z24kmAI_monitor_mouse_middlev"]
    fn kmAI_monitor_mouse_middle() -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z23kmAI_monitor_mouse_rightv"]
    fn kmAI_monitor_mouse_right() -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z24kmAI_monitor_mouse_side1v"]
    fn kmAI_monitor_mouse_side1() -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z24kmAI_monitor_mouse_side2v"]
    fn kmAI_monitor_mouse_side2() -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z20kmAI_monitor_mouse_xyPiS_"]
    fn kmAI_monitor_mouse_xy(
        x: *mut std::os::raw::c_int,
        y: *mut std::os::raw::c_int,
    ) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z24kmAI_monitor_mouse_wheelPi"]
    fn kmAI_monitor_mouse_wheel(wheel: *mut std::os::raw::c_int) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z21kmAI_monitor_keyboards"]
    fn kmAI_monitor_keyboard(vk_key: std::os::raw::c_short) -> std::os::raw::c_int;

    // 鼠标掩码
    #[link_name = "\u{1}_Z20kmAI_mask_mouse_lefti"]
    fn kmAI_mask_mouse_left(enable: std::os::raw::c_int) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z21kmAI_mask_mouse_righti"]
    fn kmAI_mask_mouse_right(enable: std::os::raw::c_int) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z22kmAI_mask_mouse_middlei"]
    fn kmAI_mask_mouse_middle(enable: std::os::raw::c_int) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z22kmAI_mask_mouse_side1i"]
    fn kmAI_mask_mouse_side1(enable: std::os::raw::c_int) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z22kmAI_mask_mouse_side2i"]
    fn kmAI_mask_mouse_side2(enable: std::os::raw::c_int) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z18kmAI_mask_mouse_xi"]
    fn kmAI_mask_mouse_x(enable: std::os::raw::c_int) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z18kmAI_mask_mouse_yi"]
    fn kmAI_mask_mouse_y(enable: std::os::raw::c_int) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z19kmAI_mask_mouse_xyi"]
    fn kmAI_mask_mouse_xy(enable: std::os::raw::c_int) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z21kmAI_mask_mouse_wheeli"]
    fn kmAI_mask_mouse_wheel(enable: std::os::raw::c_int) -> std::os::raw::c_int;

    // 键盘掩码
    #[link_name = "\u{1}_Z19kmAI_mask_keyboards"]
    fn kmAI_mask_keyboard(vkey: std::os::raw::c_short) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z21kmAI_unmask_keyboards"]
    fn kmAI_unmask_keyboard(vkey: std::os::raw::c_short) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z15kmAI_unmask_allv"]
    fn kmAI_unmask_all() -> std::os::raw::c_int;

    // MiniUI控制
    #[link_name = "\u{1}_Z18kmAI_MiniUI_Enablei"]
    fn kmAI_MiniUI_Enable(MODE: std::os::raw::c_int) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z20kmAI_MiniUI_UserLockv"]
    fn kmAI_MiniUI_UserLock() -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z22kmAI_MiniUI_UserUnLockv"]
    fn kmAI_MiniUI_UserUnLock() -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z21kmAI_MiniUI_LCDdisplayN2cv3MatE"]
    fn kmAI_MiniUI_LCDdisplay(img: *mut std::os::raw::c_void) -> std::os::raw::c_int;

    // YOLO模型管理
    #[link_name = "\u{1}_Z18kmAI_YOLO_LoadmodelPKc"]
    fn kmAI_YOLO_Loadmodel(model_path: *const std::os::raw::c_char) -> *mut std::os::raw::c_void;

    #[link_name = "\u{1}_Z23kmAI_YOLO_InterfaceModelPv"]
    fn kmAI_YOLO_InterfaceModel(ctx: *mut std::os::raw::c_void) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z22kmAI_YOLO_DrawRectangleiiiiii"]
    fn kmAI_YOLO_DrawRectangle(
        x: std::os::raw::c_int,
        y: std::os::raw::c_int,
        w: std::os::raw::c_int,
        h: std::os::raw::c_int,
        color: std::os::raw::c_int,
        thickness: std::os::raw::c_int,
    ) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z16kmAI_YOLO_DrawTextPKciiiii"]
    fn kmAI_YOLO_DrawText(
        text: *const std::os::raw::c_char,
        x: std::os::raw::c_int,
        y: std::os::raw::c_int,
        color: std::os::raw::c_int,
        scale: std::os::raw::c_int,
        thickness: std::os::raw::c_int,
    ) -> std::os::raw::c_int;

    #[link_name = "\u{1}_Z18kmAI_YOLO_ReleasePv"]
    fn kmAI_YOLO_Release(ctx: *mut std::os::raw::c_void) -> std::os::raw::c_int;

    // 系统操作
    #[link_name = "\u{1}_Z10kmAI_rebootv"]
    fn kmAI_reboot() -> std::os::raw::c_int;
}

/// 鼠标按钮枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    /// 左键
    Left,
    /// 右键
    Right,
    /// 中键
    Middle,
    /// 侧键1
    Side1,
    /// 侧键2
    Side2,
}

/// 鼠标动作枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseAction {
    /// 按下
    Press,
    /// 释放
    Release,
    /// 点击
    Click,
}

/// 鼠标位置结构体
#[derive(Debug, Clone, Copy)]
pub struct MousePosition {
    pub x: i32,
    pub y: i32,
}

/// 鼠标状态结构体
#[derive(Debug, Clone)]
pub struct MouseState {
    pub position: MousePosition,
    pub left: bool,
    pub right: bool,
    pub middle: bool,
    pub side1: bool,
    pub side2: bool,
    pub wheel: i32,
}

/// MiniUI模式枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MiniUIMode {
    /// 禁用
    Disabled = 0,
    /// 启用
    Enabled = 1,
}

/// KmboxAI核心控制器
pub struct KmboxAI {
    initialized: bool,
    yolo_context: Option<*mut std::os::raw::c_void>,
}

impl KmboxAI {
    /// 创建新的KmboxAI实例
    pub fn new() -> KmboxResult<Self> {
        Ok(Self {
            initialized: false,
            yolo_context: None,
        })
    }

    /// 初始化KmboxAI系统
    pub fn init(&mut self) -> KmboxResult<()> {
        let result = unsafe { kmAI_Init() };
        check_result(result, "初始化KmboxAI系统")?;
        self.initialized = true;
        Ok(())
    }

    /// 运行KmboxAI系统
    pub fn run(&self) -> KmboxResult<()> {
        let result = unsafe { kmAI_Run() };
        check_result(result, "运行KmboxAI系统")
    }

    /// 启用/禁用KmboxAI
    pub fn enable(&self, enable: bool) -> KmboxResult<()> {
        let result = unsafe { kmAI_kmEnable(if enable { 1 } else { 0 }) };
        check_result(result, "设置KmboxAI启用状态")
    }

    /// 获取版本信息
    pub fn version(&self) -> KmboxResult<i32> {
        let result = unsafe { kmAI_Version() };
        Ok(result)
    }

    /// 移动鼠标到指定位置
    pub fn mouse_move(&self, x: i32, y: i32) -> KmboxResult<()> {
        let result = unsafe { kmAI_mouse_move(x, y) };
        check_result(result, "移动鼠标")
    }

    /// 鼠标按键操作
    #[allow(clippy::only_used_in_recursion)]
    pub fn mouse_button(&self, button: MouseButton, action: MouseAction) -> KmboxResult<()> {
        let isdown = match action {
            MouseAction::Press => 1,
            MouseAction::Release => 0,
            MouseAction::Click => {
                // 先按下再释放
                self.mouse_button(button, MouseAction::Press)?;
                std::thread::sleep(std::time::Duration::from_millis(50));
                return self.mouse_button(button, MouseAction::Release);
            }
        };

        let result = unsafe {
            match button {
                MouseButton::Left => kmAI_mouse_left(isdown),
                MouseButton::Right => kmAI_mouse_right(isdown),
                MouseButton::Middle => kmAI_mouse_middle(isdown),
                MouseButton::Side1 => kmAI_mouse_side1(isdown),
                MouseButton::Side2 => kmAI_mouse_side2(isdown),
            }
        };
        check_result(result, "鼠标按键操作")
    }

    /// 鼠标滚轮操作
    pub fn mouse_wheel(&self, wheel: i32) -> KmboxResult<()> {
        let result = unsafe { kmAI_mouse_wheel(wheel) };
        check_result(result, "鼠标滚轮操作")
    }

    /// 综合鼠标操作
    pub fn mouse_all(
        &self,
        x: i32,
        y: i32,
        left: bool,
        right: bool,
        middle: bool,
        wheel: i32,
    ) -> KmboxResult<()> {
        let result = unsafe {
            kmAI_mouse_all(
                x,
                y,
                if left { 1 } else { 0 },
                if right { 1 } else { 0 },
                if middle { 1 } else { 0 },
                wheel,
            )
        };
        check_result(result, "综合鼠标操作")
    }

    /// 键盘按键按下
    pub fn key_down(&self, vkey: i32) -> KmboxResult<()> {
        let result = unsafe { kmAI_keydown(vkey) };
        check_result(result, "键盘按键按下")
    }

    /// 键盘按键释放
    pub fn key_up(&self, vkey: i32) -> KmboxResult<()> {
        let result = unsafe { kmAI_keyup(vkey) };
        check_result(result, "键盘按键释放")
    }

    /// 键盘按键点击
    pub fn key_click(&self, vkey: i32) -> KmboxResult<()> {
        self.key_down(vkey)?;
        std::thread::sleep(std::time::Duration::from_millis(50));
        self.key_up(vkey)
    }

    /// 初始化键盘
    pub fn key_init(&self) -> KmboxResult<()> {
        let result = unsafe { kmAI_keyinit() };
        check_result(result, "初始化键盘")
    }

    /// 监控鼠标左键状态
    pub fn monitor_mouse_left(&self) -> KmboxResult<bool> {
        let result = unsafe { kmAI_monitor_mouse_left() };
        Ok(result != 0)
    }

    /// 监控鼠标中键状态
    pub fn monitor_mouse_middle(&self) -> KmboxResult<bool> {
        let result = unsafe { kmAI_monitor_mouse_middle() };
        Ok(result != 0)
    }

    /// 监控鼠标右键状态
    pub fn monitor_mouse_right(&self) -> KmboxResult<bool> {
        let result = unsafe { kmAI_monitor_mouse_right() };
        Ok(result != 0)
    }

    /// 监控鼠标侧键1状态
    pub fn monitor_mouse_side1(&self) -> KmboxResult<bool> {
        let result = unsafe { kmAI_monitor_mouse_side1() };
        Ok(result != 0)
    }

    /// 监控鼠标侧键2状态
    pub fn monitor_mouse_side2(&self) -> KmboxResult<bool> {
        let result = unsafe { kmAI_monitor_mouse_side2() };
        Ok(result != 0)
    }

    /// 监控鼠标位置
    pub fn monitor_mouse_position(&self) -> KmboxResult<MousePosition> {
        let mut x = 0i32;
        let mut y = 0i32;
        let result = unsafe { kmAI_monitor_mouse_xy(&mut x, &mut y) };
        check_result(result, "监控鼠标位置")?;
        Ok(MousePosition { x, y })
    }

    /// 监控鼠标滚轮
    pub fn monitor_mouse_wheel(&self) -> KmboxResult<i32> {
        let mut wheel = 0i32;
        let result = unsafe { kmAI_monitor_mouse_wheel(&mut wheel) };
        check_result(result, "监控鼠标滚轮")?;
        Ok(wheel)
    }

    /// 监控键盘按键
    pub fn monitor_keyboard(&self, vk_key: i16) -> KmboxResult<bool> {
        let result = unsafe { kmAI_monitor_keyboard(vk_key) };
        Ok(result != 0)
    }

    /// 获取完整鼠标状态
    pub fn get_mouse_state(&self) -> KmboxResult<MouseState> {
        let position = self.monitor_mouse_position()?;
        let left = self.monitor_mouse_left()?;
        let right = self.monitor_mouse_right()?;
        let middle = self.monitor_mouse_middle()?;
        let side1 = self.monitor_mouse_side1()?;
        let side2 = self.monitor_mouse_side2()?;
        let wheel = self.monitor_mouse_wheel()?;

        Ok(MouseState {
            position,
            left,
            right,
            middle,
            side1,
            side2,
            wheel,
        })
    }

    /// 掩码鼠标左键
    pub fn mask_mouse_left(&self, enable: bool) -> KmboxResult<()> {
        let result = unsafe { kmAI_mask_mouse_left(if enable { 1 } else { 0 }) };
        check_result(result, "掩码鼠标左键")
    }

    /// 掩码鼠标右键
    pub fn mask_mouse_right(&self, enable: bool) -> KmboxResult<()> {
        let result = unsafe { kmAI_mask_mouse_right(if enable { 1 } else { 0 }) };
        check_result(result, "掩码鼠标右键")
    }

    /// 掩码鼠标中键
    pub fn mask_mouse_middle(&self, enable: bool) -> KmboxResult<()> {
        let result = unsafe { kmAI_mask_mouse_middle(if enable { 1 } else { 0 }) };
        check_result(result, "掩码鼠标中键")
    }

    /// 掩码鼠标侧键1
    pub fn mask_mouse_side1(&self, enable: bool) -> KmboxResult<()> {
        let result = unsafe { kmAI_mask_mouse_side1(if enable { 1 } else { 0 }) };
        check_result(result, "掩码鼠标侧键1")
    }

    /// 掩码鼠标侧键2
    pub fn mask_mouse_side2(&self, enable: bool) -> KmboxResult<()> {
        let result = unsafe { kmAI_mask_mouse_side2(if enable { 1 } else { 0 }) };
        check_result(result, "掩码鼠标侧键2")
    }

    /// 掩码鼠标X轴移动
    pub fn mask_mouse_x(&self, enable: bool) -> KmboxResult<()> {
        let result = unsafe { kmAI_mask_mouse_x(if enable { 1 } else { 0 }) };
        check_result(result, "掩码鼠标X轴移动")
    }

    /// 掩码鼠标Y轴移动
    pub fn mask_mouse_y(&self, enable: bool) -> KmboxResult<()> {
        let result = unsafe { kmAI_mask_mouse_y(if enable { 1 } else { 0 }) };
        check_result(result, "掩码鼠标Y轴移动")
    }

    /// 掩码鼠标XY轴移动
    pub fn mask_mouse_xy(&self, enable: bool) -> KmboxResult<()> {
        let result = unsafe { kmAI_mask_mouse_xy(if enable { 1 } else { 0 }) };
        check_result(result, "掩码鼠标XY轴移动")
    }

    /// 掩码鼠标滚轮
    pub fn mask_mouse_wheel(&self, enable: bool) -> KmboxResult<()> {
        let result = unsafe { kmAI_mask_mouse_wheel(if enable { 1 } else { 0 }) };
        check_result(result, "掩码鼠标滚轮")
    }

    /// 掩码键盘按键
    pub fn mask_keyboard(&self, vkey: i16) -> KmboxResult<()> {
        let result = unsafe { kmAI_mask_keyboard(vkey) };
        check_result(result, "掩码键盘按键")
    }

    /// 取消掩码键盘按键
    pub fn unmask_keyboard(&self, vkey: i16) -> KmboxResult<()> {
        let result = unsafe { kmAI_unmask_keyboard(vkey) };
        check_result(result, "取消掩码键盘按键")
    }

    /// 取消所有掩码
    pub fn unmask_all(&self) -> KmboxResult<()> {
        let result = unsafe { kmAI_unmask_all() };
        check_result(result, "取消所有掩码")
    }

    /// 启用/禁用MiniUI
    pub fn miniui_enable(&self, mode: MiniUIMode) -> KmboxResult<()> {
        let result = unsafe { kmAI_MiniUI_Enable(mode as i32) };
        check_result(result, "设置MiniUI状态")
    }

    /// 锁定MiniUI用户界面
    pub fn miniui_user_lock(&self) -> KmboxResult<()> {
        let result = unsafe { kmAI_MiniUI_UserLock() };
        check_result(result, "锁定MiniUI用户界面")
    }

    /// 解锁MiniUI用户界面
    pub fn miniui_user_unlock(&self) -> KmboxResult<()> {
        let result = unsafe { kmAI_MiniUI_UserUnLock() };
        check_result(result, "解锁MiniUI用户界面")
    }

    /// 在LCD上显示图像
    ///
    /// # Safety
    ///
    /// 调用者必须确保 `img` 是一个有效的图像指针，且指向的内存在使用期间保持有效。
    pub unsafe fn miniui_lcd_display(&self, img: *mut std::os::raw::c_void) -> KmboxResult<()> {
        let result = kmAI_MiniUI_LCDdisplay(img);
        check_result(result, "LCD显示图像")
    }

    /// 加载YOLO模型
    pub fn yolo_load_model(&mut self, model_path: &str) -> KmboxResult<()> {
        let path_cstr = CString::new(model_path)
            .map_err(|e| KmboxError::ParameterError(format!("无效的模型路径: {}", e)))?;

        let ctx = unsafe { kmAI_YOLO_Loadmodel(path_cstr.as_ptr()) };
        if ctx.is_null() {
            return Err(KmboxError::ModelError("加载YOLO模型失败".to_string()));
        }

        self.yolo_context = Some(ctx);
        Ok(())
    }

    /// 接口YOLO模型
    pub fn yolo_interface_model(&self) -> KmboxResult<()> {
        if let Some(ctx) = self.yolo_context {
            let result = unsafe { kmAI_YOLO_InterfaceModel(ctx) };
            check_result(result, "接口YOLO模型")
        } else {
            Err(KmboxError::ModelError("YOLO模型未加载".to_string()))
        }
    }

    /// 绘制矩形
    pub fn yolo_draw_rectangle(
        &self,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        color: i32,
        thickness: i32,
    ) -> KmboxResult<()> {
        let result = unsafe { kmAI_YOLO_DrawRectangle(x, y, w, h, color, thickness) };
        check_result(result, "绘制矩形")
    }

    /// 绘制文本
    pub fn yolo_draw_text(
        &self,
        text: &str,
        x: i32,
        y: i32,
        color: i32,
        scale: i32,
        thickness: i32,
    ) -> KmboxResult<()> {
        let text_cstr = CString::new(text)
            .map_err(|e| KmboxError::ParameterError(format!("无效的文本: {}", e)))?;

        let result =
            unsafe { kmAI_YOLO_DrawText(text_cstr.as_ptr(), x, y, color, scale, thickness) };
        check_result(result, "绘制文本")
    }

    /// 重启系统
    pub fn reboot(&self) -> KmboxResult<()> {
        let result = unsafe { kmAI_reboot() };
        check_result(result, "重启系统")
    }
}

impl Drop for KmboxAI {
    fn drop(&mut self) {
        // 释放YOLO上下文
        if let Some(ctx) = self.yolo_context {
            unsafe {
                let _ = kmAI_YOLO_Release(ctx);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kmbox_ai_creation() {
        let kmbox = KmboxAI::new();
        assert!(kmbox.is_ok());
    }

    #[test]
    fn test_mouse_button_enum() {
        assert_eq!(MouseButton::Left, MouseButton::Left);
        assert_ne!(MouseButton::Left, MouseButton::Right);
    }

    #[test]
    fn test_mouse_action_enum() {
        assert_eq!(MouseAction::Press, MouseAction::Press);
        assert_ne!(MouseAction::Press, MouseAction::Release);
    }

    #[test]
    fn test_miniui_mode_enum() {
        assert_eq!(MiniUIMode::Disabled as i32, 0);
        assert_eq!(MiniUIMode::Enabled as i32, 1);
    }

    #[test]
    fn test_mouse_position() {
        let pos = MousePosition { x: 100, y: 200 };
        assert_eq!(pos.x, 100);
        assert_eq!(pos.y, 200);
    }

    #[test]
    fn test_mouse_state() {
        let state = MouseState {
            position: MousePosition { x: 100, y: 200 },
            left: true,
            right: false,
            middle: false,
            side1: false,
            side2: false,
            wheel: 0,
        };
        assert_eq!(state.position.x, 100);
        assert_eq!(state.position.y, 200);
        assert!(state.left);
        assert!(!state.right);
    }
}
