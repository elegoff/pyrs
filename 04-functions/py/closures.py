if __name__ == "__main__":
    my_lambda = lambda x: len(x)


    v = [1, 2, 3]
    s = my_lambda("lambda")
    n = my_lambda(v)
    print(s, n)

    is_same = lambda z: z == v

    print("Comparing to", v)
    v2 = [1, 2, 3]
    assert is_same(v2)

    things = range(1,42)
    doubling = list(map(lambda x: x * 2, things))
    total = sum(map(lambda x: x + 1, things))
    print(doubling, total)

    multi_args = lambda a,b,c: (a + b) * c
    print(multi_args(20,1,2))