import sys

s = "Simpson"
mem_size = sys.getsizeof(s)
print(f"'{s}': length: {len(s)} chars: {len(s)} mem_size: {mem_size}")

# make bytes
print(s.encode('utf-8'))
print("after i:", s[2:])

#
print("===")

unicode = "Thanks 😊"
mem_size = sys.getsizeof(unicode)
print(f"'{unicode}': length: {len(unicode)} chars: {len(unicode)} mem_size: {mem_size}")

# make bytes
#
#
print(unicode.encode('utf-8'))
print("after h:", unicode[2:])