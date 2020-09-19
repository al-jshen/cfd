import matplotlib.pyplot as plt
from celluloid import Camera
import sys

fig = plt.figure()
camera = Camera(fig)

infile = sys.argv[1]

with open(infile, "r") as f:
    for line in f.readlines():
        plt.plot([float(i.strip()) for i in line.strip()[1:-1].split(",")], c="b")
        camera.snap()

animation = camera.animate()
plt.show()

if len(sys.argv) == 3:
    animation.save(sys.argv[2])
    print(f"animation saved to {sys.argv[2]}")
