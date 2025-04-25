#include <iostream>             // for std::cerr
#include <opencv2/opencv.hpp>   // for cv::cvtColor
#include <fstream>              // for ofstream
#include "kmboxAI.h"
#include <unistd.h>              // 引入 usleep 所需的头文件
#include "my_timer.h"

#define Video_width 1280
#define Video_heigh 720
#define Video_fps   60



int main(int, char **)
{
    TIMER duration;
    cv::VideoCapture cap(9); // 9固定值，对应盒子采集卡编号
    if (!cap.isOpened()) {std::cerr << "错误: 无法打开视频文件!" << std::endl;return -1;}
    // 设置分辨率为 1920x1080
    if (!cap.set(cv::CAP_PROP_FRAME_WIDTH, Video_width) ||
        !cap.set(cv::CAP_PROP_FRAME_HEIGHT, Video_heigh)) {
        std::cerr << "错误: 无法设置分辨率为" <<Video_width<<"x"<<Video_heigh<< std::endl;
    }
    // 设置帧率为 60Hz
    if (!cap.set(cv::CAP_PROP_FPS, Video_fps)) {
        std::cerr << "错误: 无法设置帧率为" <<Video_fps<<std::endl;
    }
    // 获取实际设置后的图像的宽度、高度和帧率
    double width = cap.get(cv::CAP_PROP_FRAME_WIDTH);
    double height = cap.get(cv::CAP_PROP_FRAME_HEIGHT);
    double fps = cap.get(cv::CAP_PROP_FPS);
    std::cout << "实际图像分辨率: " << width << "x" << height << std::endl;
    std::cout << "实际帧率: " << fps << " Hz" << std::endl;
    cv::Mat frame;

    //你要找什么图？
    cv::Mat templateImage = cv::imread("findwhat.png");  //你要找的图像
    printf("查找图片大小：%dx%d\n",templateImage.cols,templateImage.rows);

    while (true)  {
        cap >> frame;              // 读取视频的一帧
        if (frame.empty()) {       // 如果读取帧失败，说明还未准备好。
            usleep(5000);          //休眠5ms 
            continue;              //继续读取
        }

        // 创建输出结果矩阵
            cv::Mat result;
            duration.tik();
            cv::matchTemplate(frame, templateImage, result, cv::TM_CCOEFF_NORMED);
            duration.tok(); duration.print_time("找图耗时");



            duration.tik();
            double minVal, maxVal;
            cv::Point minLoc, maxLoc;
            cv::minMaxLoc(result, &minVal, &maxVal, &minLoc, &maxLoc);            // 确定最佳匹配位置
            cv::Point matchLoc;            // 对于 TM_CCOEFF_NORMED 方法，最佳匹配位置是最大值的位置
            matchLoc = maxLoc;             // 在输入图像上绘制矩形框出匹配区域
            cv::rectangle(frame, matchLoc, cv::Point(matchLoc.x + templateImage.cols, matchLoc.y + templateImage.rows), cv::Scalar(0, 255, 0), 2);
            duration.tok(); duration.print_time("后处理耗时");


        kmAI_MiniUI_LCDdisplay(frame); //读到数据就显示到屏幕上
    };
    return 0;
}
