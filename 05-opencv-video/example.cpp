#include <iostream>             // for std::cerr
#include <opencv2/opencv.hpp>   // for cv::cvtColor
#include <fstream>              // for ofstream
#include <stdint.h> // for uint32_t
#include <sys/ioctl.h> // for ioctl
#include <linux/fb.h> // for fb_
#include <fcntl.h> // for O_RDWR
#include "kmboxAI.h"
#include "my_timer.h"

int main() {
    // 打开视频文件
    TIMER duration;
    cv::VideoCapture cap("test.mp4"); // 请将此替换为你的视频文件路径
    // 检查视频是否成功打开
    if (!cap.isOpened()) {
        std::cerr << "错误: 无法打开视频文件!" << std::endl;
        return -1;
    }
    cv::Mat frame;
    duration.tik();//开始计时
    while (true) {
        // 读取视频的一帧
        cap >> frame;
        // 如果读取帧失败，退出循环
        if (frame.empty()) {
            std::cout << "视频播放结束或者读取帧失败。" << std::endl;
            break;
        }
        kmAI_MiniUI_LCDdisplay(frame);
        // 显示当前帧
    }
    duration.tok();
    duration.print_time("视频播放完成，总共耗时");
    // 释放视频捕获对象并关闭所有窗口
    cap.release();
    return 0;
} 
