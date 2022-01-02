   NB. See https://code.jsoftware.com/wiki/Vocabulary/percent#dyadic
   0 % 0
0
   10 % 5
2
   _10 % 5
_2
   NB. 3. Dividing zero by a nonzero produces positive or negative zero.
   0 % _3
_0
   0 % 34
0
   NB. Dividing 0 by infinity produces positive or negative 0.
   0 % _
0
   0 % __
_0
   NB. You can elementwise divide matrices
   10 20 30 % 1 2 3
10 10 10
   10 20 30 40 % 0.5
20 40 60 80
