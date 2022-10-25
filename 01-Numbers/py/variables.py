if __name__ =="__main__":
    a = 42
    print(a)
    #shadowing the value
    a = "forty two"
    print(a)

    # declare, init
    opt = None
    a = 15
    print("x, opt", a, opt)
    opt = a * 5
    print("x, opt", a, opt)

    #python variabkles are mutable
    b = 0
    print(f"before b == {b}")
    b = (b + 3) * opt
    print(f"after b == {b}")

    FOO = 42  # only const by convention
    a *= FOO