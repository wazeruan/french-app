#!/usr/bin/env python3
"""Build AppIcon.icns from a source image. Usage: make-icns.py <src> <out.icns>"""
import struct, sys, io
from PIL import Image

TYPES = [(16, b'icp4'), (32, b'icp5'), (64, b'icp6'), (128, b'ic07'),
         (256, b'ic08'), (512, b'ic09'), (1024, b'ic10')]

src = Image.open(sys.argv[1]).convert('RGBA')
entries = b''
for size, otype in TYPES:
    img = src.resize((size, size), Image.LANCZOS)
    buf = io.BytesIO()
    img.save(buf, 'PNG')
    data = buf.getvalue()
    entries += otype + struct.pack('>I', 8 + len(data)) + data

out = b'icns' + struct.pack('>I', 8 + len(entries)) + entries
open(sys.argv[2], 'wb').write(out)
print(f'wrote {sys.argv[2]} ({len(out)} bytes, {len(TYPES)} sizes)')
