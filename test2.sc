definitions:
   a = point(12,1);
   b = point(1,1);
   d = point(9,4);
   y = square(d,6);
   x = circle(a,2)
   z = circle(b, 5);
operations:
   print(x);
   contained(x,y);
   print(y);
   intersects(y,x);
   print(z);
   contained(x,z)
end.