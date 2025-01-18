
#include <stdio.h>
#include <stdlib.h>
#include "kmboxAI.h"
/*
这是一个最简单调用demo
kmAI_Init()         :盒子初始化
kmAI_kmEnable()     :是否使能直通模式
kmAI_Run()          :等待子线程退出。（后续可以删除）
*/

int main(int argc, char **argv){   
    printf("\n\n当你看到此条打印，恭喜你已经成功入门kmboxAI编程版本号：%d\n",kmAI_Version());
    printf("kmAI_Init()返回值=%d (初始化盒子。必须调用一次)\n",kmAI_Init()); //初始化盒子   
    kmAI_kmEnable(1);       //使能键鼠直通到主机
    //kmAI_kmEnable(0);     //盒子上的鼠标只能操作盒子内部UI。
    printf("调用kmAI_kmEnable(1)后可以将盒子上的键鼠消息透传到主机口。\n"); 
    printf("此时你接盒子上的键盘、鼠标就能控制主机啦（接OTG口的电脑）。");
    kmAI_Run();//开始运行
    return 0;
}


