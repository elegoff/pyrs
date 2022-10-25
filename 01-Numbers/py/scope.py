glob_v = 42


def my_function():
    global glob_v
    loc_v = 43
    def nested_func():
        print("local", loc_v)
        print("global", glob_v)
        enclosed_variable = 3
        
        #uncommenting below would fail ("referenced before assiugnment")
        #loc_v = 5
    nested_func()
    #below print woudl fail : varibale went ouf of scope
    #print("enclosed", enclosed_variable)
    
    glob_v = 149
    loc_v = 12
    print("global", glob_v)
    print("local", loc_v)

    # example use of built-in scope
    foo = int("42")


if __name__ == '__main__':
    my_function()