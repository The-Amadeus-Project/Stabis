x = 1
print(x)
y = 2
for _ in range(100):
    print(y)
    temp = y
    y += x
    x = temp

print(y)
