
#include <iostream>
#include <unistd.h>
#include <chrono>
#include <thread>
#include "kmboxAI.h"
#include "KeyboardTable.h" //键盘按键值请参考此文件
using namespace std;

/*
鼠标类函数测试：
kmAI_monitor_mouse_xxx      :监控物理鼠标对应xxx按键状态
kmAI_mask_mouse_xxx         :屏蔽物理鼠标对应xxx按键
kmAI_unmask_all             :取消所有物理键鼠屏蔽
kmAI_mouse_move             :鼠标移动测试
test by hw@20241212
*/

#define Delay100ms usleep(100*1000)   

//要测试哪个功能将宏定义改为1 ，其他的改成0
#define TEST_MOUSE_MONITOR 0 //测试鼠标监控类函数
#define TEST_MOUSE_MASK    0 //物理鼠标屏蔽类函数测试
#define TEST_MOUSE_CTR     1 //鼠标控制类函数测试--单线程测试
#define TEST_MOUSE_THREAD  0 //多线程方式测试鼠标发送速度


#if  TEST_MOUSE_THREAD
// 定义线程函数 1
void threadFunction1() {
    int i=80000;
    while(i)
    {    if(kmAI_mouse_move(10,10)) {cout<<"线程1执行错误 error:"<<endl;}
         i--;
    }
}
// 定义线程函数 1
void threadFunction2() {
    int i=80000;
    while(i)
    {    if(kmAI_mouse_move(-10,-10)) {cout<<"线程2执行错误 error:"<<endl;}
         i--;
    }
}
#endif 


int main(int argc, char **argv){   
 
    cout <<"kmboxAI初始化返回值="<<kmAI_Init()<<endl;    //正常返回0 ，错误值参考错误代码
    kmAI_kmEnable(true);                                //使能键鼠直通到主机
    while(kmAI_monitor_keyboard(KEY_A)==0) //按下键盘A键就退出。 按键键值表可参考附录 HID键值表文档。
    {
       
#if TEST_MOUSE_MONITOR  || TEST_MOUSE_MASK  //鼠标按键状态检测
        if(kmAI_monitor_mouse_left()==1)    {cout<<"鼠标左键按下"<<endl;    while(kmAI_monitor_mouse_left()==1)     {Delay100ms;};cout<<"鼠标左键松开"<<endl;};
        if(kmAI_monitor_mouse_middle()==1)  {cout<<"鼠标中键按下"<<endl;    while(kmAI_monitor_mouse_middle()==1)   {Delay100ms;};cout<<"鼠标中键松开"<<endl;};
        if(kmAI_monitor_mouse_right()==1)   {cout<<"鼠标右键按下"<<endl;    while(kmAI_monitor_mouse_right()==1)    {Delay100ms;};cout<<"鼠标右键松开"<<endl;};
        if(kmAI_monitor_mouse_side1()==1)   {cout<<"鼠标侧键1按下"<<endl;   while(kmAI_monitor_mouse_side1()==1)    {Delay100ms;};cout<<"鼠标侧键1松开"<<endl;};
        if(kmAI_monitor_mouse_side2()==1)   {cout<<"鼠标侧键2按下"<<endl;   while(kmAI_monitor_mouse_side2()==1)    {Delay100ms;};cout<<"鼠标侧键2松开"<<endl;};
        static int x,y,w; //记录上一次的XYW值
        int xx,yy,ww;
        kmAI_monitor_mouse_xy(&xx,&yy);
        kmAI_monitor_mouse_wheel(&ww);
        if(xx!=x || yy!=y || ww!=w) //做一个简单的过滤，不然打印数据太多了。只有数据与上一次的不一样才打印
        {   cout<<"鼠标:x="<<x<<" y="<<y<<" wheel="<<w<<endl;
            x=xx;y=yy;w=ww;
        }
#endif 

#if TEST_MOUSE_MASK  //物理鼠标屏蔽类函数测试
        kmAI_mask_mouse_left(1);     //屏蔽物理鼠标左键
        kmAI_mask_mouse_middle(1);   //屏蔽物理鼠标中键
        kmAI_mask_mouse_right(1);
        kmAI_mask_mouse_side1(1);
        kmAI_mask_mouse_side2(1);
        kmAI_mask_mouse_x(1);
        kmAI_mask_mouse_y(1);
        kmAI_mask_mouse_xy(1);
        kmAI_mask_mouse_wheel(1);
#endif 

#if TEST_MOUSE_CTR  //鼠标移动效率测试
        std::cout << "鼠标移动速度测试--单线程" << std::endl;
        auto start = std::chrono::high_resolution_clock::now();
        for(int i=0;i<80000;i++)
        {   int ret=kmAI_mouse_move(10,10);
                ret=kmAI_mouse_move(-10,-10);
            if(ret!=0)
            {   cout<<"执行错误 error:"<<ret<<endl;
                break;
            }
        }
        auto end = std::chrono::high_resolution_clock::now();
        auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
        std::cout << "执行16万次移动耗时" << duration.count() << " ms" << std::endl; 
        //16万次移动耗时是40001ms.平均1次移动耗时0.25ms=250us（4KHZ回报率）.正常应该是125us.为什么少了一倍？
        break;
#endif 


#if TEST_MOUSE_THREAD  //鼠标移动效率测试
        std::cout << "鼠标移动速度测试---多线程" << std::endl;
        auto start = std::chrono::high_resolution_clock::now();
        std::thread t1(threadFunction1);
        std::thread t2(threadFunction2);   
        t1.join();       // 等待线程完成
        t2.join();
        auto end = std::chrono::high_resolution_clock::now();
        auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
        std::cout << "执行16万次移动耗时" << duration.count() << " ms" << std::endl; 
        //16万次移动耗时是40005ms.平均1次移动耗时0.25ms=250us（4KHZ回报率）.正常应该是125us.为什么少了一倍？
        break;
#endif 

    }
    cout<<"退出主循环 解除所有鼠标屏蔽，鼠标恢复正常\n"<<endl;
    kmAI_unmask_all();
    kmAI_Run();//开始运行 ,此函数不会退出
    return 0;
}


