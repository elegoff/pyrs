if __name__ == "__main__":
    multi_types = (1, "nce", "upon",'a', True, 3.1416)

    print("first value:", multi_types[0])
    print("last value:", multi_types[-1])

    nested_tuple = ((4,2,6), (-1,3,2), (9,3,5))
    print("nested_tuple: ", nested_tuple)


    assert len(nested_tuple) == 3


    #destructuring / unpacking
    a,b,c,d,e,f = multi_types

    print(a)
    print(b)
    print(c)
    print(d)
    print(e)
    print(f)

    nested2 = ((4, (5, 'Gfg')), (7, (8, 6)))
  
    # printing original list
    print("The original tuple is : " + str(nested2))
  
    # Unpacking nested tuples
    # using list comprehension
    res = [(x, y, z) for x, (y, z) in nested2]
  
    # printing result
    print("The unpacked nested tuple  listis : " + str(res))


   