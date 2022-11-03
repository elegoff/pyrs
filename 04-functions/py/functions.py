def outer(x):
    def inner(a):
        return a + 40

    return inner(x)


if __name__ == '__main__':
    print(outer(2))