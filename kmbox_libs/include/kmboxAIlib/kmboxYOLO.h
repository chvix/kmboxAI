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
#ifndef _KMBOX_YOLO_H_
#define _KMBOX_YOLO_H_
#include "kmbox_rknn_api.h"
#include "common.h"
#include <stdbool.h>

#define OBJ_NUMB_MAX_SIZE 128  //最多检测128个目标

typedef struct {
    rknn_context rknn_ctx;          //    
    rknn_input_output_num io_num;   
    rknn_tensor_attr* input_attrs; //输入张量
    rknn_tensor_attr* output_attrs;//输出张量
    int model_channel;  //模型通道数
    int model_width;    //模型宽度
    int model_height;   //模型高度
    bool is_quant;      //是否量化
    float NMS_THRESH;   //NMS阈值
    float BOX_THRESH;   //置信度阈值
    int OBJ_CLASS_NUM;  //有多少个class
    int yoloType;       //这个模型是YOLO几的模型？ 其中yolov5/7需要设置锚框参数
    int anchor[3][6];   //在yolov5 yolov7中需要设置锚框
} rknn_context_t; 


typedef struct {
    image_rect_t box;   //位置信息
    float prop;         //概率信息
    int cls_id;         //识别类编号
} object_detect_result;

typedef struct {
    int id;         //id 
    int count;      //总数
    object_detect_result results[OBJ_NUMB_MAX_SIZE];//最大检测128个目标
} object_detected_list;

#endif 