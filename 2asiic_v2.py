import cv2
import numpy as np
import time
import argparse
from PIL import Image

# 优化的ASCII字符集（89级灰度）
ASCII_CHARS = (
    "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,\"^`'. "
)

# 增强的ASCII字符集（带颜色支持）
ENHANCED_CHARS = [
    ("@", (0, 0, 0)),        # 纯黑
    ("#", (70, 70, 70)),     # 深灰
    ("8", (135, 135, 135)),  # 中灰
    ("o", (200, 200, 200)),  # 浅灰
    (" ", (255, 255, 255))   # 纯白
]

def get_enhanced_char(pixel):
    """获取带颜色映射的ASCII字符"""
    brightness = sum(pixel) / 3
    for char, color in ENHANCED_CHARS:
        if brightness <= sum(color)/3:
            return char
    return " "

def resize_with_aa(image, new_width=100):
    """带抗锯齿的调整大小"""
    (old_height, old_width) = image.shape
    aspect_ratio = old_height / old_width
    new_height = int(aspect_ratio * new_width * 0.55)  # 更精确的宽高比
    
    # 使用高质量的抗锯齿缩放
    resized_image = cv2.resize(
        image, 
        (new_width, new_height),
        interpolation=cv2.INTER_AREA
    )
    
    # 应用高斯模糊进行柔化
    resized_image = cv2.GaussianBlur(resized_image, (3, 3), 0)
    return resized_image

def gamma_correction(image, gamma=1.8):
    """Gamma校正增强对比度"""
    inv_gamma = 1.0 / gamma
    table = np.array([
        ((i / 255.0) ** inv_gamma) * 255
        for i in np.arange(0, 256)
    ]).astype("uint8")
    return cv2.LUT(image, table)

def frame_to_high_quality_ascii(frame, width=100, color=False):
    """高精度ASCII转换"""
    # 转换为灰度图
    gray_frame = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)
    
    # 图像增强处理
    gray_frame = gamma_correction(gray_frame)
    gray_frame = cv2.equalizeHist(gray_frame)
    
    # 带抗锯齿的调整大小
    resized_frame = resize_with_aa(gray_frame, width)
    
    # 获取图像尺寸
    rows, cols = resized_frame.shape
    
    # 转换为ASCII
    ascii_img = ""
    for i in range(rows):
        for j in range(cols):
            pixel = resized_frame[i, j]
            if color:
                # 获取原始帧对应位置的颜色
                orig_row = int(i * frame.shape[0] / rows)
                orig_col = int(j * frame.shape[1] / cols)
                orig_color = frame[orig_row, orig_col]
                # 使用ANSI颜色代码
                r, g, b = orig_color
                char = get_enhanced_char(resized_frame[i, j])
                ascii_img += f"\033[38;2;{r};{g};{b}m{char}\033[0m"
            else:
                # 高精度灰度映射
                normalized = pixel / 255
                index = int(normalized * (len(ASCII_CHARS) - 1))
                ascii_img += ASCII_CHARS[index]
        ascii_img += "\n"
    
    return ascii_img

def process_media(input_file, width=100, fps_limit=30, color=False):
    """处理媒体文件（视频或GIF）"""
    if input_file.lower().endswith(('.gif')):
        # GIF处理
        try:
            gif = Image.open(input_file)
            frame_delay = gif.info.get('duration', 100) / 1000
            
            if fps_limit > 0:
                frame_delay = max(frame_delay, 1/fps_limit)
                
            while True:
                start_time = time.time()
                try:
                    gif.seek(gif.tell() + 1)
                    frame = cv2.cvtColor(np.array(gif.convert('RGB')), cv2.COLOR_RGB2BGR)
                except EOFError:
                    gif.seek(0)
                    continue
                    
                ascii_art = frame_to_high_quality_ascii(frame, width, color)
                print("\033[H\033[J" + ascii_art)
                
                elapsed_time = time.time() - start_time
                time.sleep(max(0, frame_delay - elapsed_time))
        except Exception as e:
            print(f"处理GIF时出错: {e}")
    else:
        # 视频处理
        cap = cv2.VideoCapture(input_file)
        if not cap.isOpened():
            print("无法打开视频文件")
            return
            
        original_fps = cap.get(cv2.CAP_PROP_FPS)
        frame_delay = 1 / original_fps if original_fps > 0 else 1/30
        
        if fps_limit > 0:
            frame_delay = max(frame_delay, 1/fps_limit)
            
        while True:
            start_time = time.time()
            ret, frame = cap.read()
            if not ret:
                cap.set(cv2.CAP_PROP_POS_FRAMES, 0)
                continue
                
            ascii_art = frame_to_high_quality_ascii(frame, width, color)
            print("\033[H\033[J" + ascii_art)
            
            elapsed_time = time.time() - start_time
            time.sleep(max(0, frame_delay - elapsed_time))
        cap.release()

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='高精度视频/GIF转ASCII动画工具')
    parser.add_argument('input_file', help='输入文件路径')
    parser.add_argument('--width', type=int, default=120, help='输出宽度（字符数）')
    parser.add_argument('--fps', type=int, default=30, help='最大FPS限制')
    parser.add_argument('--color', action='store_true', help='启用彩色输出')
    
    args = parser.parse_args()
    
    try:
        process_media(args.input_file, args.width, args.fps, args.color)
    except KeyboardInterrupt:
        print("\n转换已停止")
