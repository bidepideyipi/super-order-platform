from PIL import Image, ImageDraw
import os

icon_dir = "/Users/anthony/Documents/github/super-order/desktop/src-tauri/icons"

sizes = [32, 128, 256, 512]
for size in sizes:
    img = Image.new('RGBA', (size, size), (66, 133, 244, 255))
    draw = ImageDraw.Draw(img)
    
    padding = size // 10
    box = [padding, padding, size - padding, size - padding]
    draw.rectangle(box, outline=(255, 255, 255, 255), width=size // 10)
    
    draw.rectangle([size//4, size//3, size*3//4, size*2//3], fill=(255, 255, 255, 255))
    
    img.save(os.path.join(icon_dir, f"{size}x{size}.png"))

print("Icons created successfully")
