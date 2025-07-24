import cv2
import numpy as np
import time
import argparse
from PIL import Image  # 用于GIF处理

# ASCII字符集，从最暗到最亮
ASCII_CHARS = "@%#*+=-:. "

def resize_image(image, new_width=100):
    # 计算新的高度，保持宽高比
    (old_height, old_width) = image.shape
    aspect_ratio = old_height / old_width
    new_height = int(aspect_ratio * new_width / 2)  # 除以2因为字符高度大约是宽度的两倍
    
    # 调整图像大小
    resized_image = cv2.resize(image, (new_width, new_height))
    
    return resized_image

def pixel_to_ascii(image):
    # 将像素值映射到ASCII字符
    pixels = image.flatten()
    ascii_str = ""
    
    for pixel in pixels:
        # 标准化到0-1范围
        normalized = pixel / 255
        # 映射到ASCII字符
        ascii_str += ASCII_CHARS[int(normalized * (len(ASCII_CHARS) - 1))]
    
    return ascii_str

def frame_to_ascii(frame, width=100):
    # 如果帧是PIL图像格式，先转换为OpenCV格式
    if isinstance(frame, Image.Image):
        frame = cv2.cvtColor(np.array(frame), cv2.COLOR_RGB2BGR)
    
    # 转换为灰度图
    gray_frame = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)
    
    # 调整大小
    resized_frame = resize_image(gray_frame, width)
    
    # 转换为ASCII
    ascii_str = pixel_to_ascii(resized_frame)
    
    # 将字符串分割成行
    ascii_str_len = len(ascii_str)
    cols = resized_frame.shape[1]
    ascii_img = ""
    
    for i in range(0, ascii_str_len, cols):
        ascii_img += ascii_str[i:i+cols] + "\n"
    
    return ascii_img

def process_gif(gif_path, output_width=100, fps_limit=30):
    try:
        gif = Image.open(gif_path)
    except:
        print("无法打开GIF文件")
        return
    
    # 获取GIF的帧延迟时间(毫秒)
    frame_delay = gif.info.get('duration', 100) / 1000  # 转换为秒
    
    # 如果设置了FPS限制，调整帧延迟
    if fps_limit > 0:
        frame_delay = max(frame_delay, 1/fps_limit)
    
    while True:
        start_time = time.time()
        
        try:
            # 获取下一帧
            gif.seek(gif.tell() + 1)
            frame = gif.copy()
        except EOFError:
            # 循环播放
            gif.seek(0)
            continue
        
        # 转换为ASCII
        ascii_art = frame_to_ascii(frame, output_width)
        
        # 清屏并打印ASCII艺术
        print("\033[H\033[J")  # 清屏
        print(ascii_art)
        
        # 控制帧率
        elapsed_time = time.time() - start_time
        sleep_time = max(0, frame_delay - elapsed_time)
        time.sleep(sleep_time)

def process_video(video_path, output_width=100, fps_limit=30):
    # 打开视频文件
    cap = cv2.VideoCapture(video_path)
    
    if not cap.isOpened():
        print("无法打开视频文件")
        return
    
    # 获取视频的原始FPS
    original_fps = cap.get(cv2.CAP_PROP_FPS)
    frame_delay = 1 / original_fps if original_fps > 0 else 1/30
    
    # 如果设置了FPS限制，调整帧延迟
    if fps_limit > 0:
        frame_delay = max(frame_delay, 1/fps_limit)
    
    while True:
        start_time = time.time()
        
        # 读取帧
        ret, frame = cap.read()
        
        if not ret:
            # 循环播放
            cap.set(cv2.CAP_PROP_POS_FRAMES, 0)
            continue
        
        # 转换为ASCII
        ascii_art = frame_to_ascii(frame, output_width)
        
        # 清屏并打印ASCII艺术
        print("\033[H\033[J")  # 清屏
        print(ascii_art)
        
        # 控制帧率
        elapsed_time = time.time() - start_time
        sleep_time = max(0, frame_delay - elapsed_time)
        time.sleep(sleep_time)
    
    cap.release()

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='将视频/GIF转换为ASCII动画')
    parser.add_argument('input_file', type=str, help='输入文件路径(视频或GIF)')
    parser.add_argument('--width', type=int, default=100, help='输出宽度（字符数）')
    parser.add_argument('--fps', type=int, default=30, help='最大FPS限制')
    
    args = parser.parse_args()
    
    # 根据文件扩展名选择处理方式
    if args.input_file.lower().endswith(('.gif')):
        process_gif(args.input_file, args.width, args.fps)
    else:
        process_video(args.input_file, args.width, args.fps)
