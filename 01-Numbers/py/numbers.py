if __name__ == "__main__":
    # integers
    a = -42
    b = 56

    large_number = 123_456_789  # python >= 3.6

    # floats
    f = -1.2345
   
    # complex numbers
    complex_integer = complex(10, 20)
    complex_float = complex(10.0, 20.0000000000001)

    print(f"{complex_integer} == {complex_float}", complex_integer == complex_float)

    # binary literals
    b1 = 0b1010_1010
    b2 = 0b0101_0101
    assert b1 | b2 == 255
    assert b1 & b2 == 0 
    print(b1 | b2)
    print(b1 & b2)

    # hex literals
    hx = 0xCAFE_BABE
    print(hx)

    # boolean
    yes = True
    no = False