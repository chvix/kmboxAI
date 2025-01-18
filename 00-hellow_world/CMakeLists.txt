cmake_minimum_required(VERSION 3.10)
set(DEMOS 00-hellow_kmboxAI)
project(${DEMOS})


set(CMAKE_SYSTEM_NAME Linux)
set(CMAKE_SYSTEM_PROCESSOR aarch64)
set(tools "/home/rk3566/sdk_my/buildroot/output/rockchip_rk3566/host/bin")
set(CMAKE_C_COMPILER ${tools}/aarch64-linux-gcc)
set(CMAKE_CXX_COMPILER ${tools}/aarch64-linux-g++)
set(CMAKE_C_STANDARD 99) # LVGL officially supports C99 and above
set(CMAKE_CXX_STANDARD 17) #C17
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(EXECUTABLE_OUTPUT_PATH ${PROJECT_SOURCE_DIR}/output)


set(KM_INCLUDE ${PROJECT_SOURCE_DIR}/../kmbox_libs/install/include)
include_directories(${KM_INCLUDE})
include_directories(${KM_INCLUDE}/3rdparty/opencv/opencv-linux-aarch64/include)
include_directories(.)


set(LIBS ${PROJECT_SOURCE_DIR}/../kmbox_libs/install/lib)
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
    )



