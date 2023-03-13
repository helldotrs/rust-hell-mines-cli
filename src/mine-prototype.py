import random

class GridBit:
    def __init__(self, u, v):
        self.u = u
        self.v = v
        self.hidden = True
        self.mine = random.random() < 0.2
        self.nearby = 0
        
for v in range(5):
    for u in range(10):
        print(u,    end="")
        print(",",  end="")
        print(v,    end="")
        print(" ",  end="")
    print("")
        
        
"""
bit_u0v0 = GridBit(u=0, v=0)
bit_u0v1 = GridBit(u=0, v=0)
bit_u0v2 = GridBit(u=0, v=0)

bit_u1v0 = GridBit(u=0, v=0)
bit_u1v1 = GridBit(u=0, v=0)
bit_u1v2 = GridBit(u=0, v=0)

bit_u2v0 = GridBit(u=0, v=0)
bit_u2v1 = GridBit(u=0, v=0)
bit_u2v2 = GridBit(u=0, v=0)
"""

