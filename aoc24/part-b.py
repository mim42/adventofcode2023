from z3 import *

rocks = []

for line in  open("inputs/input.txt"):
    a,b = line.split("@")
    x,y,z = a.split(",")
    x,y,z = x.strip(),y.strip(),z.strip()
    vx,vy,vz = b.split(",")
    vx,vy,vz = vx.strip(),vy.strip(),vz.strip()
    rocks.append((x,y,z,vx,vy,vz))

s = Solver()

all_times = []
x, y, z = BitVecs("x y z", 64)
vx, vy, vz = BitVecs("vx vy vz", 64)

for i, rock in enumerate(rocks[:20]):
    t = BitVec(str(i)+"_t", 64)
    s.add(t > 0)
    s.add(x + vx * t == rock[0] + rock[3] * t)
    s.add(y + vy * t == rock[1] + rock[4] * t)
    s.add(z + vz * t == rock[2] + rock[5] * t)
    all_times.append(t)

s.add(Distinct(all_times))
s.check()

x,y,z = s.model()[x].as_long() ,s.model()[y].as_long(),s.model()[z].as_long()

print(x+y+z)
