#!/usr/bin/env python3
# From https://gist.github.com/ariscop/f24ffc95a7a1767f8f83

import os
import tempfile
import argparse
import json
from struct import unpack_from, iter_unpack
from collections import namedtuple
from uuid import UUID
import base64

from names import *

stringtable_offset = None
stringtable = {}


def read_unknown(data, offset, end):
    return None, 0


def read_null(data, offset, end):
    return None, 2


def read_list(data, offset, end):
    tag, length = unpack_from("<HI", data, offset)
    data = read_tag(data, offset + 6, offset + 6 + length)
    return data, 6 + length


def read_stringtable(data, offset, end):
    global stringtable_offset
    stringtable_offset = offset
    return read_list(data, offset, end)


def read_binary(data, offset, end):
    tag, length = unpack_from("<HI", data, offset)
    text = base64.b64encode(data[offset + 6 :][:length]).decode()
    with open(
        os.path.join(tempfile.gettempdir(), "TAG_%s@%s" % (names[tag], hex(offset))),
        "wb",
    ) as out:
        out.write(data[offset + 6 :][:length])
    return text, 6 + length


def read_uuid(data, offset, end):
    tag, length = unpack_from("<HI", data, offset)
    text = "{%s}" % str(UUID(bytes=bytes(data[offset + 6 :][:length])))
    return text, 6 + length


def read_index(data, offset, end):
    tag, length = unpack_from("<HI", data, offset)
    text = "".join(
        "%s, %s\n" % (key[::-1], offset)
        for key, offset in iter_unpack("<8sI", data[offset + 6 :][:length])
    )
    return text, 6 + length


def read_string(data, offset, end):
    tag, length = unpack_from("<HI", data, offset)
    text = bytes(data[offset + 6 :][:length]).decode("utf-16").strip("\x00")
    return text, 6 + length


def read_stringtable_item(data, offset, end):
    text, length = read_string(data, offset, end)
    stringtable[offset - stringtable_offset] = text
    return None, length


def read_word(data, offset, end):
    tag, value = unpack_from("<HH", data, offset)
    return value, 4


def read_tagname(data, offset, end):
    tag, value = unpack_from("<HH", data, offset)
    return names[value], 4


def read_dword(data, offset, end):
    tag, value = unpack_from("<HI", data, offset)
    return value, 6


def read_qword(data, offset, end):
    tag, value = unpack_from("<HQ", data, offset)
    return value, 10


class parsers_dict(dict):
    def __missing__(self, key):
        if key & 0xF000 in self:
            return self[key & 0xF000]
        else:
            raise KeyError(key)


parsers = parsers_dict()
parsers.update(
    {
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
        TAG_DATABASE_ID: read_uuid,
        TAG_APP_ID: read_uuid,
        TAG_CONTEXT_BRANCH_ID: read_uuid,
        TAG_CONTEXT_PLATFORM_ID: read_uuid,
        TAG_MSI_PACKAGE_ID: read_uuid,
        TAG_INDEX_BITS: read_index,
    }
)


def read_tag(data, offset, end):
    result = []
    while offset < end:
        (tag,) = unpack_from("<H", data, offset)
        child = {"tag": f"TAG_{names[tag]}", "type": types[tag & 0xF000]}
        if names[tag].endswith("_ID"):
            child["type"] = "GUID"

        value, size = parsers[tag](data, offset, end)
        if value is not None:
            child["value"] = value

        if names[tag] != "STRINGTABLE":
            result.append(child)
        if size == 0:
            return result
        offset += size
        # offset is word aligned
        if offset % 2:
            offset += 1
    return result

def fix_strings(root):
    for item in root:
        if item["type"] == "LIST":
            fix_strings(item["value"])
        elif item["type"] == "STRINGREF":
            item["type"] = "STRING"
            item["value"] = stringtable[item["value"]]

def main():
    parser = argparse.ArgumentParser("SDB Parser")
    parser.add_argument("input", help="input file")
    parser.add_argument("--output", "-o", help="optional output file")
    args = parser.parse_args()
    with open(args.input, "rb") as fd:
        data = memoryview(fd.read())
    root = {"name": args.input}

    root["data"] = read_tag(data, 0xC, len(data))
    fix_strings(root["data"])

    # Print
    print(json.dumps(root, indent=2))
    if args.output is not None:
        with open(args.output, "w") as f:
            json.dump(root, f, indent=2)


if __name__ == "__main__":
    main()
