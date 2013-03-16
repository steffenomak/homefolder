import os

class conf:
    prefix=""
    location=""

class _pair:
    first=""
    second=""

def _split(st):
    p = _pair()
    first = True
    for c in st:
        if c == '\n':
            continue
        elif c == ' ':
            continue
        elif c == '=':
            first = False
        else:
            if first:
                p.first += c
            else:
                p.second += c
    return p

def _unravel_command(command):
    if command == 'HOME':
        return os.getenv('HOME')
    else:
        print(command)
        return ""

def _location(st):
    tmp = ""
    command = ""
    in_command = False
    for c in st:
        if c == '%':
            if in_command:
                tmp += _unravel_command(command)
                in_command = False
            else:
                in_command = True
        elif in_command:
            command += c
        else:
            tmp += c

    return tmp

def conf_file(stream):
    c = conf()
    for line in stream:
        p = _split(line)
        if p.first == 'prefix':
            c.prefix = p.second
        elif p.first == 'location':
            c.location = _location(p.second)
    return c
