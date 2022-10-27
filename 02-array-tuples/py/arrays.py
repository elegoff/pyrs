from array import array
if __name__ == "__main__":

    #typecode (1st arg) must be b, B, u, h, H, i, I, l, L, q, Q, f or d)
    arr = array('i', [1, 2, 3, 4, 5])

    # better use `numpy.zeros((500,), dtype=numpy.uint64)` 
    big_array = array('Q', [0] * 500)

    print("first element of the array:", arr[0])
    print("last element of the array:", arr[-1])

    print("small size:", len(arr))
    print("big size:", len(big_array))

    print("a section of the array as a slice", arr[1:3])

    # Going beyond the limits , i.e "out of range"
    print(arr[5])