#
set(DEMOS kmboxApp.exe)
project(${DEMOS})
# 设置 CMake 最低版本要求
cmake_minimum_required(VERSION 3.10)
# 指定目标系统名称
set(CMAKE_SYSTEM_NAME Linux)
# 指定目标系统的处理器架构，根据实际情况修改
set(CMAKE_SYSTEM_PROCESSOR aarch64)
# 设置交叉编译工具链的路径，将 /path/to/toolchain  替换为实际路径
set(TOOLCHAIN_DIR /opt/toolchain/bin)
# 设置 C 编译器
set(CMAKE_C_COMPILER ${TOOLCHAIN_DIR}/aarch64-linux-gcc)
# 设置 C++ 编译器
set(CMAKE_CXX_COMPILER ${TOOLCHAIN_DIR}/aarch64-linux-g++)
# 设置链接器
set(CMAKE_LINKER ${TOOLCHAIN_DIR}/aarch64-linux-ld)
# 设置 AR 工具
set(CMAKE_AR ${TOOLCHAIN_DIR}/aarch64-linux-ar)
# 设置 RANLIB 工具
set(CMAKE_RANLIB ${TOOLCHAIN_DIR}/aarch64-linux-ranlib)
# 设置 FIND_ROOT_PATH 以指定在交叉编译环境中查找库和头文件的路径
set(CMAKE_FIND_ROOT_PATH ${TOOLCHAIN_DIR}/../aarch64-buildroot-linux-gnu/sysroot)
# 设置 CMake 搜索模式
set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM NEVER)
set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_PACKAGE ONLY)


#设置编译器标准
set(CMAKE_C_STANDARD 99) # LVGL officially supports C99 and above
set(CMAKE_CXX_STANDARD 17) #C17
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(EXECUTABLE_OUTPUT_PATH ${PROJECT_SOURCE_DIR}/output)


set(KM_INCLUDE ${PROJECT_SOURCE_DIR}/../kmbox_libs/include/kmboxAIlib)
include_directories(${KM_INCLUDE})
include_directories(.)


set(LIBS ${PROJECT_SOURCE_DIR}/../kmbox_libs/lib)
link_directories(${LIBS})


aux_source_directory(. SRC_LIST)
add_executable(${DEMOS} ${SRC_LIST})


target_link_libraries(${DEMOS} m pthread input udev 
    kmboxAI
    opencv_core
    opencv_imgcodecs
    opencv_imgproc
    opencv_video
    opencv_videostab
    opencv_videoio
    rknnrt
    rga
    )
