from __future__ import print_function

def _inst(args):
    print(' '.join(map(str, args)))

def penup():
    _inst(['pu'])

pu = penup
up = penup

def pendown():
    _inst(['pd'])

pd = pendown
down = pendown

def forward(distance):
    _inst(['fd', distance])

fd = forward

def backward(distance):
    forward(-distance)

bk = backward
back = backward

def right(angle):
    _inst(['rt', angle])

rt = right

def left(angle):
    right(-angle)

lt = left

if __name__ == "__main__":
    print("Don't use me as entry script")
    exit(1)