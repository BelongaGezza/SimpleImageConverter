#!/usr/bin/env python3
"""
2D Image Converter - Proof of Concept
Converts between PNG, JPG, and BMP formats
Usage: python img-convert.py <source_file> <destination_extension>
Example: python img-convert.py input.png jpg
"""

import sys
import os
from pathlib import Path
from PIL import Image

# Supported formats for PoC
SUPPORTED_FORMATS = {
    'png': 'PNG',
    'jpg': 'JPEG',
    'jpeg': 'JPEG',
    'bmp': 'BMP'
}

def convert_image(source_path, dest_extension):
    """
    Convert an image file to the specified format
    
    Args:
        source_path: Path to source image file
        dest_extension: Desired output format (png, jpg, bmp)
    
    Returns:
        Path to converted file
    """
    # Normalize extension
    dest_extension = dest_extension.lower().lstrip('.')
    
    # Validate destination format
    if dest_extension not in SUPPORTED_FORMATS:
        raise ValueError(f"Unsupported destination format: {dest_extension}. Supported: {', '.join(SUPPORTED_FORMATS.keys())}")
    
    # Validate source file exists
    source_file = Path(source_path)
    if not source_file.exists():
        raise FileNotFoundError(f"Source file not found: {source_path}")
    
    # Validate source format
    source_ext = source_file.suffix.lower().lstrip('.')
    if source_ext not in SUPPORTED_FORMATS:
        raise ValueError(f"Unsupported source format: {source_ext}. Supported: {', '.join(SUPPORTED_FORMATS.keys())}")
    
    # Generate output filename
    output_path = source_file.with_suffix(f'.{dest_extension}')
    
    print(f"Converting: {source_file} → {output_path}")
    print(f"Format: {SUPPORTED_FORMATS[source_ext]} → {SUPPORTED_FORMATS[dest_extension]}")
    
    # Open and convert
    try:
        with Image.open(source_file) as img:
            # Display image info
            print(f"Source info: {img.format} {img.mode} {img.size[0]}x{img.size[1]}")
            
            # Handle transparency for formats that don't support it
            if dest_extension in ['jpg', 'jpeg'] and img.mode in ('RGBA', 'LA', 'P'):
                print("Note: Converting transparent image to JPG (transparency will be lost)")
                # Convert RGBA to RGB with white background
                if img.mode == 'RGBA':
                    background = Image.new('RGB', img.size, (255, 255, 255))
                    background.paste(img, mask=img.split()[3])  # Use alpha channel as mask
                    img = background
                elif img.mode == 'P':
                    img = img.convert('RGB')
                else:
                    img = img.convert('RGB')
            
            # Save with quality settings
            save_kwargs = {}
            if dest_extension in ['jpg', 'jpeg']:
                save_kwargs['quality'] = 95  # High quality for PoC
                save_kwargs['optimize'] = True
            elif dest_extension == 'png':
                save_kwargs['optimize'] = True
            
            img.save(output_path, SUPPORTED_FORMATS[dest_extension], **save_kwargs)
            
            print(f"✓ Conversion successful!")
            print(f"Output: {output_path}")
            print(f"Size: {output_path.stat().st_size:,} bytes")
            
            return str(output_path)
            
    except Exception as e:
        raise RuntimeError(f"Conversion failed: {str(e)}")

def main():
    if len(sys.argv) != 3:
        print("Usage: python img-convert.py <source_file> <destination_extension>")
        print("Example: python img-convert.py input.png jpg")
        print(f"Supported formats: {', '.join(SUPPORTED_FORMATS.keys())}")
        sys.exit(1)
    
    source_file = sys.argv[1]
    dest_extension = sys.argv[2]
    
    try:
        convert_image(source_file, dest_extension)
        sys.exit(0)
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()
