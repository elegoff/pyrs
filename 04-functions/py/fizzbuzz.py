if __name__ == "__main__":
    
    def fizzbuzz(x):
        if x % 15 == 0:
            print("FizzBuzz")
        elif x % 3 == 0:
            print("Fizz")
        elif x % 5 == 0:
            print("Buzz")
        else:
            print(x)

    def fizzbuzz2(x):
        res =""
        if x % 15 == 0:
            res ="FizzBuzz"
        elif x % 3 == 0:
            res = "Fizz"
        elif x % 5 == 0:
            res = "Buzz"
        else:
            res =x
        return res      

    for i in range(1, 42):
        fizzbuzz(i)

    print("===")
    print(list(map(fizzbuzz, range(1, 42))))
    print(list(map(fizzbuzz2, range(1, 42))))
