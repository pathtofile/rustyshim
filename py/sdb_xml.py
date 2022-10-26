#!/usr/bin/env python3
# From https://gist.github.com/ariscop/f24ffc95a7a1767f8f83

import os
import tempfile
import argparse
import sys
from struct import unpack, unpack_from, iter_unpack
from collections import namedtuple, defaultdict
from uuid import UUID

import xml.etree.ElementTree as ET
import xml.dom.minidom as md

from names import *

def element(name, attrib={}, text=None):
    elem = ET.Element(name, attrib)
    if not text is None:
        elem.text = str(text)
    return elem

def to_hex(data):
    return ''.join('%02x' % x for x in data)


Header = namedtuple("SdbHeader", "unk1 unk2 magic")

stringtable_offset = None

def read_unknown(node, data, offset, end):
    return 0

def read_null(node, data, offset, end):
    return 2

def read_list(node, data, offset, end):
    tag,length = unpack_from("<HI", data, offset)
    node.set("length", str(length))
    read_tag(node, data, offset + 6, offset + 6 + length)
    return 6 + length

def read_stringtable(node, data, offset, end):
    global stringtable_offset
    stringtable_offset = offset
    return read_list(node, data, offset, end)

def read_binary(node, data, offset, end):
    tag,length = unpack_from("<HI", data, offset)
    node.set("length", str(length))
    node.text = ''.join(to_hex(data[offset+6:][:length]))
    with open(os.path.join(tempfile.gettempdir(), "TAG_%s@%s" % (names[tag], hex(offset))), "wb") as out:
        out.write(data[offset+6:][:length])
    return 6 + length

def read_uuid(node, data, offset, end):
    tag,length = unpack_from("<HI", data, offset)
    node.text = "{%s}" % str(UUID(bytes=bytes(data[offset+6:][:length])))
    return 6 + length

def read_index(node, data, offset, end):
    tag,length = unpack_from("<HI", data, offset)
    node.text = ''.join("%s, %s\n" % (key[::-1], offset) for key,offset in iter_unpack("<8sI", data[offset+6:][:length]))
    return 6 + length

def read_string(node, data, offset, end):
    tag,length = unpack_from("<HI", data, offset)
    node.text = bytes(data[offset+6:][:length]).decode('utf-16').strip('\x00')
    return 6 + length

def read_stringtable_item(node, data, offset, end):
    node.set("strid", hex(offset - stringtable_offset))
    return read_string(node, data, offset, end)

def read_word(node, data, offset, end):
    tag, value = unpack_from("<HH", data, offset)
    node.text = hex(value)
    return 4

def read_tagname(node, data, offset, end):
    tag, value = unpack_from("<HH", data, offset)
    node.text = names[value]
    return 4

def read_dword(node, data, offset, end):
    tag, value = unpack_from("<HI", data, offset)
    node.text = hex(value)
    return 6

def read_qword(node, data, offset, end):
    tag, value = unpack_from("<HQ", data, offset)
    node.text = hex(value)
    return 10

class parsers_dict(dict):
    def __missing__(self, key):
        if key & 0xF000 in self:
            return self[key & 0xF000]
        else:
            raise KeyError(key)

parsers = parsers_dict()
parsers.update({
    TAG_TYPE_NULL: read_null,
    TAG_TYPE_WORD: read_word,
    TAG_TYPE_DWORD: read_dword,
    TAG_TYPE_QWORD: read_qword,
    TAG_TYPE_STRINGREF: read_dword,
    TAG_TYPE_LIST: read_list,
    TAG_TYPE_STRING: read_string,
    TAG_TYPE_BINARY: read_binary,

    TAG_INDEX_TAG: read_tagname,
    TAG_INDEX_KEY: read_tagname,
    TAG_STRINGTABLE: read_stringtable,
    TAG_STRINGTABLE_ITEM: read_stringtable_item,
    TAG_FIX_ID: read_uuid,
    TAG_EXE_ID: read_uuid,
    TAG_APP_ID: read_uuid,
    TAG_CONTEXT_BRANCH_ID: read_uuid,
    TAG_CONTEXT_PLATFORM_ID: read_uuid,
    TAG_MSI_PACKAGE_ID: read_uuid,
    TAG_INDEX_BITS: read_index,
})

def read_tag(node, data, offset, end):
    while(offset < end):
        tag, = unpack_from("<H", data, offset)
        child = element(names[tag], attrib={"type": types[tag & 0xF000], "tagid": hex(offset), "tag": hex(tag)})
        size = parsers[tag](child, data, offset, end)
        node.append(child)
        if size == 0:
            return
        offset += size
        #offset is word aligned
        if offset % 2:
            offset += 1


def main():
    parser = argparse.ArgumentParser("SDB Parser")
    parser.add_argument("input", help="input file")
    parser.add_argument("--output", "-o", help="optional output file")
    args = parser.parse_args()
    with open(args.input, "rb") as fd:
        data = memoryview(fd.read())
    root = element("Sdbf")

    read_tag(root, data, 0xC, len(data)) #TAG_ROOT

    for node in root.findall('.//*[@type="STRINGREF"]'):
        item = root.find('./STRINGTABLE/STRINGTABLE_ITEM[@strid="%s"]' % node.text)
        node.text = item.text

    #pretty printing
    output = md.parseString(ET.tostring(root).decode("utf-8")).toprettyxml()
    if args.output is None:
        print(output)
    else:
        with open(args.output, "wb") as f:
            f.write(output.encode())
    #print(ET.tostring(root).decode())


if __name__ == "__main__":
    main()
