n=int(input("Enter any number:"))
x,y=0,1
for i in range(n):
    print(x,end=" ")
    x,y=y,x+y