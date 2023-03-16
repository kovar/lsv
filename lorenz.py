"""Lorenz Chaotic Model Implementation."""

import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D

def lorenz(x, y, z, s=10, r=28, b=8/3):
    """Lorenz Chaotic Model."""
    x_dot = s * (y - x)
    y_dot = r * x - y - x * z
    z_dot = x * y - b * z
    return x_dot, y_dot, z_dot

def main():
    """main function."""
    dt = 0.01
    step_cnt = 10000

    # Need one more for the initial values
    xs = np.empty((step_cnt + 1,))
    ys = np.empty((step_cnt + 1,))
    zs = np.empty((step_cnt + 1,))

    # Setting initial values
    xs[0], ys[0], zs[0] = (0., 1., 1.05)

    # Stepping through "time".
    for i in range(step_cnt):
        # Derivatives of the X, Y, Z state
        x_dot, y_dot, z_dot = lorenz(xs[i], ys[i], zs[i])
        xs[i + 1] = xs[i] + (x_dot * dt)
        ys[i + 1] = ys[i] + (y_dot * dt)
        zs[i + 1] = zs[i] + (z_dot * dt)

    fig = plt.figure()
    ax = fig.add_subplot(projection='3d')

    ax.plot(xs, ys, zs, lw=0.5)
    ax.set_xlabel("X Axis")
    ax.set_ylabel("Y Axis")
    ax.set_zlabel("Z Axis")
    ax.set_title("Lorenz Attractor")
    plt.show()

if __name__ == "__main__":
    main()
