import struct
import numpy as np

def load_map(buffer: bytes, start_offset: int = 0) -> tuple[dict[str, str], int]:
    translations = {}
    offset = start_offset

    # Number of entries (u64 little-endian)
    (num_entries,) = struct.unpack_from('<Q', buffer, offset)
    offset += 8

    for _ in range(num_entries):
        (key_len,) = struct.unpack_from('<Q', buffer, offset)
        offset += 8
        key = buffer[offset:offset + key_len].decode('utf-8')
        offset += key_len

        (value_len,) = struct.unpack_from('<Q', buffer, offset)
        offset += 8
        value = buffer[offset:offset + value_len].decode('utf-8')
        offset += value_len

        translations[key] = value

    return translations, offset - start_offset

class RustTextBlock:
    def __init__(self):
        self.font_size = 0
        self.angle = 0.0
        self.prob = 0.0
        self.skip_translate = False
        self.fg_color = None
        self.bg_color = None
        self.text = ""
        self.lines = None  # numpy array of shape (num_lines, 4, 2), dtype=int64
        self.translations = {}

def load_textblock(data: bytes) -> RustTextBlock:
    tb = RustTextBlock()
    offset = 0

    tb.font_size, = struct.unpack_from('<Q', data, offset)
    offset += 8

    tb.angle, = struct.unpack_from('<d', data, offset)
    offset += 8

    tb.prob, = struct.unpack_from('<d', data, offset)
    offset += 8

    tb.skip_translate = bool(data[offset])
    offset += 1

    if data[offset]:
        tb.fg_color = tuple(data[offset+1:offset+4])
        offset += 4
    else:
        offset += 1

    if data[offset]:
        tb.bg_color = tuple(data[offset+1:offset+4])
        offset += 4
    else:
        offset += 1

    text_len, = struct.unpack_from('<Q', data, offset)
    offset += 8
    tb.text = data[offset:offset+text_len].decode('utf-8')
    offset += text_len

    num_lines, = struct.unpack_from('<Q', data, offset)
    offset += 8

    lines = []
    for _ in range(num_lines):
        line = []
        for _ in range(4):
            x, = struct.unpack_from('<q', data, offset)  # i64
            offset += 8
            y, = struct.unpack_from('<q', data, offset)
            offset += 8
            line.append([x, y])
        lines.append(line)

    tb.lines = np.array(lines, dtype=np.int64)
    trans, new_offset = load_map(data[offset:])
    tb.translations = trans
    offset += new_offset
    return tb, offset


class Image:
    def __init__(self):
        self.width = 0
        self.height = 0
        self.raw = False
        self.data = None

def load_image(data: bytes) -> Image:
    img = Image()
    offset = 0

    img.width, = struct.unpack_from('<H', data, offset)
    offset += 2
    img.height, = struct.unpack_from('<H', data, offset)
    offset += 2

    img.raw = bool(data[offset])
    offset += 1

    data_len, = struct.unpack_from('<Q', data, offset)
    offset += 8

    raw_data = data[offset:offset+data_len]

    if img.raw:
        img.data = np.frombuffer(raw_data, dtype=np.uint8).reshape((img.height, img.width))
    else:
        img.data = raw_data
    offset += data_len
    return img, offset


class Patch:
    def __init__(self):
        self.pos = (0, 0)
        self.bg = None  # Image
        self.info = None  # RustTextBlock

def load_patch(data: bytes) -> Patch:
    patch = Patch()
    offset = 0

    x, = struct.unpack_from('<Q', data, offset)
    offset += 8
    y, = struct.unpack_from('<Q', data, offset)
    offset += 8
    patch.pos = (x, y)

    patch.bg, total_bg_bytes = load_image(data[offset:])
    offset += total_bg_bytes

    patch.info, new_offset = load_textblock(data[offset:])
    offset += new_offset
    return patch, offset

class Export:
    def __init__(self):
        self.img = None  # Image
        self.patches = []

def load_export(data: bytes) -> Export:
    export = Export()

    export.img, offset = load_image(data)

    num_patches, = struct.unpack_from('<Q', data, offset)
    offset += 8

    patches = []
    for _ in range(num_patches):
        patch, new_offset = load_patch(data[offset:])
        offset += new_offset
        patches.append(patch)
    export.patches = patches

    return export
