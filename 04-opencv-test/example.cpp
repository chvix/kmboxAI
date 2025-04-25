#include <iostream>             // for std::cerr
#include <opencv2/opencv.hpp>   // for cv::cvtColor
#include <fstream>              // for ofstream
#include <stdint.h> // for uint32_t
#include <sys/ioctl.h> // for ioctl
#include <linux/fb.h> // for fb_
#include <fcntl.h> // for O_RDWR
#include "kmboxAI.h"
#include "my_timer.h"

void bgrToBgr565(const cv::Mat& bgrImage, unsigned short* bgr565Data);
static char help_message[]={
"----------opencv测试---------------\n\
将图片利用opencv库显示到lcd上\n\
图片名称为test.jpg\n\
第一种方式:纯CPU转换运算\n\
第二种方式:纯硬件转换\n\
\n\n\n"
};

int main(int, char **)
{
    printf("%s",help_message);
    TIMER duration;
    duration.tik();//开始计时
    cv::Mat imgs = cv::imread("test.jpg");      //读图片
    if(imgs.empty()){
        printf("图片test.jpg 不存在\n");//图像数据为空直接返回报错
        return 0;
    }
    duration.tok();duration.print_time("CPU读取图片耗时");


    printf("图片大小：%dx%d \n",imgs.cols,imgs.rows);
    printf("纯软件方法：\n");
    duration.tik();
    cv::Size lcd_size(320,240);                  //显示屏尺寸
    resize(imgs,imgs,lcd_size,cv::INTER_LINEAR); //将原始图像转换为lcd屏幕尺寸大小
    std::ofstream ofs("/dev/fb0",std::ios::binary);             //打开显示屏
    unsigned short* rgb565_data = new unsigned short[320 * 240];//显示buff
    bgrToBgr565(imgs,rgb565_data);                              //将BGR转换成BGR565
    ofs.write((char*)rgb565_data,320*240*2);                    //写入图像
    ofs.close();                                                //关闭
    delete[] rgb565_data;
    duration.tok();
    duration.print_time("纯软件转换图片格式并显示耗时[缩放+显示]");


    printf("纯硬件方法：\n");
    cv::Mat imgs_1 = cv::imread("test.jpg");      //读图片
    duration.tik();                 //开始计时
    kmAI_MiniUI_LCDdisplay(imgs_1);
    duration.tok();
    duration.print_time("纯硬件转换图片格式并显示耗时[缩放+显示]");
    return 0;
}

void bgrToBgr565(const cv::Mat& bgrImage, unsigned short* bgr565Data) {
    int width = bgrImage.cols;
    int height = bgrImage.rows;
    for (int y = 0; y < height; y++) {
        for (int x = 0; x < width; x++) {
            cv::Vec3b bgrPixel = bgrImage.at<cv::Vec3b>(y, x);
            // 提取蓝色通道，右移3位，因为BGR565中蓝色占5位
            unsigned short r = bgrPixel[0] >> 3;
            // 提取绿色通道，右移2位，因为BGR565中绿色占6位
            unsigned short g = bgrPixel[1] >> 2;
            // 提取红色通道，右移3位，因为BGR565中红色占5位
            unsigned short b = bgrPixel[2] >> 3;
            // 组合成BGR565格式，按照BGR565的位顺序（高位到低位）
            *bgr565Data++ = (r << 11) | (g << 5) | b;
        }
    }
}
