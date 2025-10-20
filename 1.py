import math as m
def mean_point_distance(point_count):
    point_list=[]
    for i in range(0,point_count):
        x=int(input("Enter x:"))
        y=int(input("Enter y:"))
        point_list.append((x,y))
    dist_list=[]
    for (x,y) in point_list:
        dist_list.append(m.sqrt((x)**2+(y)**2))
    sum=0
    for d in dist_list:
        sum=sum+d
    return (sum/point_count)

print(mean_point_distance(5))