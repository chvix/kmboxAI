#!/bin/bash

# 查找当前目录下的所有项目文件夹
for project_dir in */; do
    # 检查是否为目录
    if [ -d "$project_dir" ]; then
        echo "处理项目: ${project_dir%/}"
        
        # 清理build文件夹
        build_dir="${project_dir}build/"
        if [ -d "$build_dir" ]; then
            echo "  清理build目录..."
            find "$build_dir" -mindepth 1 -delete
        else
            echo "  build目录不存在,跳过"
        fi
        
        # 清理output文件夹中的kmboxApp.exe
        output_dir="${project_dir}output/"
        exe_file="${output_dir}kmboxApp.exe"
        if [ -f "$exe_file" ]; then
            echo "  删除kmboxApp.exe"
            rm "$exe_file"
        else
            echo "  kmboxApp.exe不存在,跳过"
        fi
        
        echo "------------------------"
    fi
done

echo "所有项目清理完成！"    