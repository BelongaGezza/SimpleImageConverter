#!/usr/bin/env python3
"""
Convert JPG images to SVG using vtracer for vectorization.
Requires: vtracer (pip install vtracer)
"""

import vtracer
import sys

def convert_jpg_to_svg(input_path, output_path, colormode='color', 
                       hierarchical='stacked', mode='spline', 
                       filter_speckle=4, color_precision=6, 
                       layer_difference=16, corner_threshold=60,
                       length_threshold=4.0, splice_threshold=45):
    """
    Convert JPG to SVG using vtracer.
    
    Args:
        input_path: Path to input JPG file
        output_path: Path to output SVG file
        colormode: 'color' or 'binary'
        hierarchical: 'stacked' or 'cutout'
        mode: 'spline', 'polygon', or 'none'
        filter_speckle: Filter speckle of this size (default: 4)
        color_precision: Number of color bits (default: 6)
        layer_difference: Layer difference threshold (default: 16)
        corner_threshold: Corner threshold (default: 60)
        length_threshold: Length threshold (default: 4.0)
        splice_threshold: Splice threshold (default: 45)
    """
    try:
        vtracer.convert_image_to_svg_py(
            input_path,
            output_path,
            colormode=colormode,
            hierarchical=hierarchical,
            mode=mode,
            filter_speckle=filter_speckle,
            color_precision=color_precision,
            layer_difference=layer_difference,
            corner_threshold=corner_threshold,
            length_threshold=length_threshold,
            splice_threshold=splice_threshold,
        )
        print(f"✓ Successfully converted {input_path} to {output_path}")
        return True
    except Exception as e:
        print(f"✗ Error converting image: {e}")
        return False

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python jpg_to_svg.py input.jpg [output.svg]")
        print("\nExample: python jpg_to_svg.py photo.jpg photo.svg")
        sys.exit(1)
    
    input_file = sys.argv[1]
    output_file = sys.argv[2] if len(sys.argv) > 2 else input_file.rsplit('.', 1)[0] + '.svg'
    
    print(f"Converting {input_file} to {output_file}...")
    convert_jpg_to_svg(input_file, output_file)
