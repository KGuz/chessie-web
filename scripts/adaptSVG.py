from sys import argv
from os import path
from xml.dom.minidom import parse, parseString
import xml.etree.ElementTree as ET

def main(filename: str):
    assert path.isfile(filename) and path.splitext(filename)[-1].lower() == ".svg"   
    adapt_svg(ET.parse(filename).getroot())

def adapt_svg(svg: ET.Element):
    for path in svg:
        for (key, val) in path.attrib.items():
            print(key)


if __name__ == "__main__":
    assert len(argv) == 2
    main(argv[1])
