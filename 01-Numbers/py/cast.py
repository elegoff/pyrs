if __name__ == "__main__":
    n = 32.1
    i = int(n)

    print(i,n)

    coord_x = 42.334
    coord_y = 42.32

    print(f"{coord_x} == {coord_y}", coord_x == coord_y)
    coord_x = 42.3200000000000000001
    coord_y = 42.32
    print(f"{coord_x} == {coord_y}", coord_x == coord_y)