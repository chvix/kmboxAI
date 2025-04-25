#include <stdio.h>
#include <unistd.h>
#include <thread>
#include "kmboxAI.h"
#include "lvgl.h"
// 这个值定义为1 或者2 
#define UI_MODE 2  //内部UI或者用户UI

static lv_obj_t   *label;
static int cnt=0;
//点下按键回调函数
static void btn_click_cb(lv_event_t * event)
{   lv_event_code_t code = lv_event_get_code(event);
    if(code==LV_EVENT_CLICKED)
    {   cnt++;
        lv_label_set_text_fmt(label,"功德+%d",cnt);           //标签名字叫“点我+1”     
        printf("按钮被点击，功德+1\n");
    }
}

//用户UI
void lv_example_user_ui(void)
{   kmAI_MiniUI_UserLock(); 
    lv_obj_t  *obj_1  =lv_img_create(lv_scr_act()); //创建一个画布obj_1
    lv_obj_t  *obj_btn=lv_button_create(obj_1);     //在画布上创建一个按钮obj_btn
    lv_obj_set_size(obj_btn,100,40);                //设置按钮大小80x30
    lv_obj_set_align(obj_1,LV_ALIGN_CENTER);        //按钮屏幕居中
    label = lv_label_create(obj_btn);               //在按钮上创建一个标签
    lv_label_set_text(label,"点我+1");              //标签名字叫“点我+1”
    LV_FONT_DECLARE(lv_font_chinese_16);            //使用中文字库
    static const lv_font_t  *font=&lv_font_chinese_16; 
    lv_obj_set_style_text_font(label, font, 0);     //标签使用中文字体
    lv_obj_set_align(label,LV_ALIGN_CENTER);        //文字按钮居中
    lv_obj_add_event_cb(obj_btn,btn_click_cb,LV_EVENT_CLICKED,NULL);//注册按键点击回调函数
    kmAI_MiniUI_UserUnLock(); 
}

int main(int argc, char **argv){   
    int ret=kmAI_Init();            //使能盒子
    kmAI_kmEnable(0);               //键鼠消息仅在盒子内部生效
#if UI_MODE==1                      //使用盒子内部UI
    kmAI_MiniUI_Enable(UI_MODE);    //用户自定义UI 如果给的参数是0 就不要设用LVGL相关库
#else if UI_MODE==2
    kmAI_MiniUI_Enable(UI_MODE);    //用户自定义UI 如果给的参数是0 就不要设用LVGL相关库
    lv_example_user_ui();
 #endif
    kmAI_Run();                     //开始运行 ,此函数不会退出
    return 0;
}


