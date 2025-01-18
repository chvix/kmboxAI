
#include <stdio.h>
#include <unistd.h>
#include "kmboxAI.h"
#include "KeyboardTable.h" //键盘按键值请参考此文件
/*
此demo用来测试键盘相关函数功能：
kmAI_monitor_keyboard()  :监控键盘指定按键
kmAI_keydown()           :软件按下键盘指定按键
kmAI_keyup()             :软件松开键盘指定按键
kmAI_mask_keyboard()     :软件屏蔽物理键盘指定按键
kmAI_unmask_all()        :取消所有屏蔽按键
test by hw 20241212
*/

#define Delay100ms usleep(100*1000)   

int main(int argc, char **argv){   
    int ret=kmAI_Init();        //使能盒子
    kmAI_kmEnable(true);        //使能键鼠直通到主机
    kmAI_mask_keyboard(KEY_A);  //屏蔽键盘按键A
    kmAI_mask_keyboard(KEY_B);  //屏蔽键盘按键B
    kmAI_mask_keyboard(KEY_C);  //屏蔽键盘按键C
    kmAI_mask_keyboard(KEY_ESCAPE);//屏蔽键盘按键ESC键
    while(kmAI_monitor_keyboard(KEY_ESCAPE)==0) //按下键盘ESC键就退出。 
    {
       
        if(kmAI_monitor_keyboard(KEY_A)){
            printf("键盘A键按下\n");
            while(kmAI_monitor_keyboard(KEY_A)) Delay100ms;//等待松开
            printf("键盘A键松开\n");
            //此时主机没有A的消息。因为前面屏蔽了A
        }
        
        if(kmAI_monitor_keyboard(KEY_B)){
            printf("键盘B键按下\n");
            while(kmAI_monitor_keyboard(KEY_B)) Delay100ms;//等待松开
            printf("键盘A键松开\n");
            kmAI_keydown(KEY_B);//软件发送一次B键按下
            kmAI_keyup(KEY_B);  //软件发送一次B键松开
            printf("软件发送B键消息\n");
        }

        if(kmAI_monitor_keyboard(KEY_C)){
            printf("键盘C键按下\n");
            while(kmAI_monitor_keyboard(KEY_C)) Delay100ms;//等待松开
            printf("键盘C键松开\n");
            kmAI_keydown(KEY_C);//软件发送一次B键按下
            kmAI_keyup(KEY_C);  //软件发送一次B键松开
            printf("软件发送C键消息\n");
        }
        Delay100ms;
    }


    //按下ESC会到此处
    kmAI_unmask_keyboard(KEY_A);//解除A键屏蔽
    kmAI_unmask_keyboard(KEY_B);//解除B键屏蔽
    printf("解除A/B按键屏蔽。C依旧屏蔽中\n");
    while(kmAI_monitor_keyboard(KEY_C)==0) Delay100ms;//再次等待C键按下

    kmAI_unmask_all();//取消所有屏蔽
    printf("所有按键恢复正常\n");
    kmAI_Run();//开始运行 ,此函数不会退出
    return 0;
}


