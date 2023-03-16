import numpy as np

def wiener_process(t):
    """generate a wiener process"""
    x = 0
    dt = t[1] - t[0]
    for i in range(1, len(t)):
        x += np.random.normal(0, np.sqrt(dt))
        t[i] = x
    return t

def main():
    """main function"""
    t = np.linspace(0, 10, 1000)
    x = wiener_process(t)
    print(x)

if __name__ == "__main__":
    main()
