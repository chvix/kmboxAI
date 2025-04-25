
/****************************************************************************
 *
 *    Copyright (c) 2024 - 2025 by www.kmbox.top Corp.  All rights reserved.
 *
 *    The material in this file is confidential and contains trade secrets
 *    of kmbox Corporation. This is proprietary information owned by
 *     www.kmbox.top Corporation. No part of this work may be disclosed,
 *    reproduced, copied, transmitted, or used in any way for any purpose,
 *    without the express written permission of www.kmbox.top Corporation.
 *
 *****************************************************************************/

#include <opencv2/opencv.hpp>   // for cv::cvtColor
#include "kmboxYOLO.h"


#ifndef __KMBOX_AI__H_
#define __KMBOX_AI__H_
#ifdef __cplusplus
extern "C" {
#endif
//必须调用的函数
int kmAI_Init(); //初始化硬件 --ok
int kmAI_Run();  //启动系统  --tdb
int kmAI_kmEnable(int enable);//enable=1时盒子上的键鼠使能控制主机。0时只在盒子内部生效。-ok
int kmAI_Version();           //返回kmbox的软件版本号。
//鼠标类函数 ok
int kmAI_mouse_move(short x, short y);//鼠标相对移动控制ok--20241107
int kmAI_mouse_left(int isdown);//鼠标左键控制ok
int kmAI_mouse_right(int isdown);//鼠标右键控制ok
int kmAI_mouse_middle(int isdown);//鼠标中键控制ok
int kmAI_mouse_wheel(int wheel);//鼠标滚轮控制ok
int kmAI_mouse_side1(int isdown);//鼠标侧键1控制ok
int kmAI_mouse_side2(int isdown);//鼠标侧键2控制ok
int kmAI_mouse_all(int button, int x, int y, int wheel);//鼠标按键，坐标，滚轮一次性控制ok

//键盘函数 ok
int kmAI_keydown(int vkey);// ok
int kmAI_keyup(int vkey);  // ok
int kmAI_keyinit();         //ok 复位所有软件键盘按下消息
//int kmAI_keypress(int vk_key, int ms);	//ok

//物理鼠标状态获取--OK
int kmAI_monitor_mouse_left();		//查询物理鼠标左键状态 返回值0=松开，1=按下ok--20241107
int kmAI_monitor_mouse_middle();	//查询鼠标中键状态 返回值0=松开，1=按下ok--20241107
int kmAI_monitor_mouse_right();		//查询鼠标右键状态 返回值0=松开，1=按下ok--20241107
int kmAI_monitor_mouse_side1();		//查询鼠标侧键1状态 返回值0=松开，1=按下ok--20241107
int kmAI_monitor_mouse_side2();		//查询鼠标侧键2状态  返回值0=松开，1=按下ok--20241107
int kmAI_monitor_mouse_xy(int* x, int* y);//查询鼠标xy坐标值(最近一次移动的值) x,y是返回值。ok--20241107
int kmAI_monitor_mouse_wheel(int* wheel);//查询鼠标滚轮值(最近一次移动的值)    wheel是返回值。  ok--20241107
int kmAI_monitor_keyboard(short  vk_key);//查询键盘指定按键状态   返回值0=松开，1=按下  ok--20241107

//物理键鼠屏蔽系列--OK
int kmAI_mask_mouse_left(int enable);	//屏蔽鼠标左键 enable=1屏蔽  0解除屏蔽 ok--20241107
int kmAI_mask_mouse_right(int enable);	//屏蔽鼠标右键 enable=1屏蔽  0解除屏蔽 ok--20241107
int kmAI_mask_mouse_middle(int enable); //屏蔽鼠标中键 enable=1屏蔽  0解除屏蔽 ok--20241107
int kmAI_mask_mouse_side1(int enable);	//屏蔽鼠标侧键键1 enable=1屏蔽  0解除屏蔽 ok--20241107
int kmAI_mask_mouse_side2(int enable);	//屏蔽鼠标侧键键2 enable=1屏蔽  0解除屏蔽 ok--20241107
int kmAI_mask_mouse_x(int enable);		//单独屏蔽鼠标X轴坐标 enable=1屏蔽  0解除屏蔽 ok--20241107
int kmAI_mask_mouse_y(int enable);		//单独屏蔽鼠标y轴坐标 enable=1屏蔽  0解除屏蔽 ok--20241107
int kmAI_mask_mouse_xy(int enable);     //同时屏蔽鼠标xy轴坐标  enable=1屏蔽  0解除屏蔽 ok--20241107
int kmAI_mask_mouse_wheel(int enable);	//屏蔽鼠标滚轮   enable=1屏蔽  0解除屏蔽        ok--20241107
int kmAI_mask_keyboard(short vkey);	    //屏蔽键盘指定按键 vkey是要屏蔽的按键HID值。各个按键值参考hid键值表
int kmAI_unmask_keyboard(short vkey);	//解除键盘指定按键 vkey是要屏蔽的按键HID值。各个按键值参考hid键值表
int kmAI_unmask_all();					//解除屏蔽所有已经设置的物理屏蔽 ok--20241107

//图像处理类函数
int kmAI_MiniUI_Enable(int MODE);           //0默认UI界面 。UI 1：用户模式    2:关闭UI
int kmAI_MiniUI_UserLock();                 //用户调用UI界面时需要先
int kmAI_MiniUI_UserUnLock();               //当操作完用户代码后解除锁定。
int kmAI_MiniUI_LCDdisplay(cv::Mat img);    //直接将openCV的mat显示到整个屏幕上 

//加载模型
int kmAI_YOLO_Loadmodel(rknn_context_t *ctx,char *model_path);
//将图片img送入模型中推导,结果存放在out中。你需要自行处理out中的数据。
int kmAI_YOLO_InterfaceModel(rknn_context_t *ctx,cv::Mat img,object_detected_list *out);//将img图像放到
//在img中(rx,ry)中画一个宽高为（rw,rh）颜色为color厚度为thickness的矩形框。
void kmAI_YOLO_DrawRectangle(cv::Mat img, int rx, int ry, int rw, int rh, unsigned int color,int thickness);//画框
//在img中(rx,ry)处插入文本text,颜色为color字体大小为fontsize的文字
void kmAI_YOLO_DrawText(cv::Mat img, const char* text,int rx, int ry, unsigned int color,int fontsize);//绘制文本
//销毁相关资源
int kmAI_YOLO_Release(rknn_context_t *ctx);//



//
//配置类函数
int kmAI_reboot(void); //重启盒子


#ifdef __cplusplus
}
#endif


#endif
