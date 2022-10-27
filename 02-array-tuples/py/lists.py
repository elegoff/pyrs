if __name__ == "__main__":

    xs = [3 ,5, 23, 12]
    print("Using list like ", xs)

    print("appending to the list")
    xs.append(42)
    print("list becomes ", xs)
    print("length of the list", len(xs))
    print("first element", xs[0])
    print("last eleeent", xs[-1])
    try:
        print("goibg out of range ?", xs[42])
    except IndexError:
        print("out of range exception was raised")
    

    #iterating 1
    for elem in xs:
        print("element found :", elem)

    #interating 2
    for idx, elem in enumerate(xs):
        print(f"Found {elem} as index {idx}")

    # using for comprehensions loop to update in place
    xs = [elem * 2 for elem in xs]
    print("doubled list becomes ", xs)

    # builing a new list from a for comprehension
    sz = 10
    my_list = [i for i in range(0,sz) if i > 5]
   
    print(my_list.__class__)
   
    assert len(my_list) == 4
    for elem in my_list:
        print("element found :", elem)
    


