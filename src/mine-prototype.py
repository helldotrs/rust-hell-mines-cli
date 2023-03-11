import random

class GridBit:
    def __init__(self, u, v):
        self.u = u
        self.v = v
        self.hidden = True
        self.mine = random.random() < 0.2
        self.nearby = 0

bit_u0v0 = GridBit(u=0, v=0)
