#include <iostream>             
#include <opencv2/opencv.hpp>   
#include <fstream>             
#include <stdint.h>
#include <sys/ioctl.h> 
#include <linux/fb.h>
#include <fcntl.h> 
#include "kmboxAI.h"
#include "my_timer.h"

// Color Format ARGB8888
#define COLOR_GREEN     0xFF00FF00
#define COLOR_BLUE      0xFF0000FF
#define COLOR_RED       0xFFFF0000
/*
本demo中没有设置标签。如果需要标签请自行创建一个字符数组。根据class_id值与标签对应
*/
void showhelp()
{   printf("\n\n自己训练模型：检测安全帽\n");
}

int main(int argc, char **argv){
    int ret;
    TIMER duration;     //计时器
    rknn_context_t ctx;//npu上下文    
    kmAI_Init();         //初始化kmboxAI库
    duration.tik();      //加载yolo模型
    ret=kmAI_YOLO_Loadmodel(&ctx,"yolov6.rknn");
    duration.tok();duration.print_time("加载模型耗时");

    if(ret<0){printf("加载模型：yolov6.rknn 失败\n");return 0;}
    //设置模型参数 
    ctx.yoloType=6;      //获取模型版本
    ctx.OBJ_CLASS_NUM=2; //你的模型有多少个类就填多少
    ctx.NMS_THRESH=0.45;  //NMS阈值
    ctx.BOX_THRESH=0.55;  //置信度
    printf("yolov%d 模型宽度=%d 高度=%d 通道数=%d\n",ctx.yoloType,ctx.model_width,ctx.model_height,ctx.model_channel);
    cv::Mat img = cv::imread("test.jpg", cv::IMREAD_COLOR); //读取待检测图片
    object_detected_list od_results;                        //推理结果列表

    duration.tik();
    ret=kmAI_YOLO_InterfaceModel(&ctx,img,&od_results);     //将图像送入NPU进行推理
    duration.tok();duration.print_time("推理耗时"); //此部分可以去掉画框。节省时间
    if(ret){ printf("推理出错，错误代码=%d\n",ret);return -2;}

    
    //推理完成，画框和概率
    char text[256];
    //duration.tik();
    for (int i = 0; i < od_results.count; i++)
    {   object_detect_result *det_result = &(od_results.results[i]);
        printf("class%d @ (%d %d %d %d) %.3f\n", det_result->cls_id,det_result->box.left, det_result->box.top,det_result->box.right, det_result->box.bottom,det_result->prop);
        int x1 = det_result->box.left;
        int y1 = det_result->box.top;
        int x2 = det_result->box.right;
        int y2 = det_result->box.bottom; 
        kmAI_YOLO_DrawRectangle(img, x1, y1, x2 - x1, y2 - y1, COLOR_RED, 3); //在原图中画红框。3像素厚度
        sprintf(text, "%d %.1f%%", det_result->cls_id, det_result->prop * 100); //显示类别和概率
        kmAI_YOLO_DrawText(img, text, x1, y1 - 20, COLOR_RED, 10);            //在原图中画类别和概率      
    }
    //duration.tok();duration.print_time("后处理耗时"); //此部分可以去掉画框。节省时间
    //duration.tik();
    kmAI_MiniUI_LCDdisplay(img);      //显示推理结果
    kmAI_YOLO_Release(&ctx);
    return 0;
}

