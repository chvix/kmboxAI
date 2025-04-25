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
#define COLOR_YELLOW    0xFFFFFF00
#define COLOR_ORANGE    0xFFFF4500
#define COLOR_BLACK     0xFF000000
#define COLOR_WHITE     0xFFFFFFFF
/*
本demo中没有设置标签。如果需要标签请自行创建一个字符数组。根据class_id值与标签对应
*/
void showhelp()
{   printf("这是一个利用opencv读取视频。并将视频送入NPU推理识别的demo\n");
    printf("用法：./kmboxApp.exe 5 model.rknn video.mp4\n");
    printf("你需要给三个参数：\n");
    printf("\t第一个5表示yolov5版本。\n");
    printf("\t第二个model.rknn表示 yolov的模型注意模型与版本要对应。\n");
    printf("\t第三个test.mp4表示测试视频\n");
    printf("模型和mp4文件最好放在当前文件夹内\n");
    printf("本demo支持yolo V5/6/7/8/10/11等版本\n");
    printf("注意版本和模型请一一对应。不要给5的参数用6的模型\n\n");
}


//yolo v5 /v7中使用
const int anchor[3][6] = {{10, 13, 16, 30, 33, 23},
                          {30, 61, 62, 45, 59, 119},
                          {116, 90, 156, 198, 373, 326}};

int main(int argc, char **argv){
    int ret;
    TIMER duration;//计时器
    rknn_context_t ctx;//npu上下文    
    if(argc!=4){showhelp();return 0;}
    kmAI_Init();//初始化kmboxAI库
    //加载yolo模型
    duration.tik();
    ret=kmAI_YOLO_Loadmodel(&ctx,argv[2]);
    duration.tok();duration.print_time("加载模型耗时");
    if(ret<0){
        printf("加载模型：%s 失败\n",argv[2]);
        return 0;
    }

    //设置模型参数 
    ctx.yoloType=atoi(argv[1]);         //获取模型版本
    if(ctx.yoloType==5||ctx.yoloType==7){ //yolov5 和 yolo v7需要设置锚框
        memcpy(ctx.anchor,anchor,sizeof(anchor));
    }
    ctx.OBJ_CLASS_NUM=80; //80个类 ---你的模型有多少个类就填多少
    ctx.NMS_THRESH   =0.45;  //NMS阈值
    ctx.BOX_THRESH   =0.55;  //置信度
    printf("yolov%d 模型宽度=%d 高度=%d 通道数=%d\n",ctx.yoloType,ctx.model_width,ctx.model_height,ctx.model_channel);

    //打开mp4视频流
     cv::VideoCapture cap(argv[3]);
    if (!cap.isOpened()) {
        printf("无法打开%s文件\n",argv[3]);
        return 0;
    }

    cv::Mat frame;                            //推理的图像缓存 opencv格式
    object_detected_list od_results;          //推理结果列表
    cap >> frame;                             //读取视频到缓存
    while (!frame.empty())                    //读到了图像
    {   duration.tik();
        ret=kmAI_YOLO_InterfaceModel(&ctx,frame,&od_results);//将图像送入NPU进行推理
        duration.tok();duration.print_time("推理图像耗时");
        if(ret){
            printf("推理出错，错误代码=%d\n",ret);return -2;
        }
        //  推理完成，画框和概率
        char text[256];
        duration.tik();
        for (int i = 0; i < od_results.count; i++)
        {   object_detect_result *det_result = &(od_results.results[i]);
            printf("class%d @ (%d %d %d %d) %.3f\n", det_result->cls_id,det_result->box.left, det_result->box.top,det_result->box.right, det_result->box.bottom,det_result->prop);
            int x1 = det_result->box.left;
            int y1 = det_result->box.top;
            int x2 = det_result->box.right;
            int y2 = det_result->box.bottom; 
            kmAI_YOLO_DrawRectangle(frame, x1, y1, x2 - x1, y2 - y1, COLOR_RED, 3); //在原图中画红框。3像素厚度
            sprintf(text, "%d %.1f%%", det_result->cls_id, det_result->prop * 100); //显示类别和概率
            kmAI_YOLO_DrawText(frame, text, x1, y1 - 20, COLOR_RED, 10);            //在原图中画类别和概率      
        }
        duration.tok();duration.print_time("后处理耗时"); //此部分可以去掉画框。节省时间
        duration.tik();
        kmAI_MiniUI_LCDdisplay(frame);      //显示推理结果
        duration.tok();duration.print_time("显示一帧图像耗时"); //此部分可以屏蔽减少时间消耗
        cap >> frame;                       //读取下一贞视频
    }
    printf("视频播放完毕!销毁相关资源\n");
    kmAI_YOLO_Release(&ctx);
    return 0;
}

