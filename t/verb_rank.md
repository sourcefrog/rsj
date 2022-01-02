# Verb rank

At this time RSJ only supports rank 0 verbs that apply to elements. Using `-` as
an example: in monadic form it can negate a single number, or every element of
an array:

```
   - 123
_123
   - 1 2 3
_1 _2 _3
```

In dyadic form, you can subtract an array from another array:

```
   10 20 30 - 1 2 3
9 18 27
```

However the arrays must have the same shape:

```
   10 20 - 1 2 3
error: Length
```

You can also subtract a number from an array or vice versa.

```
   1 - 1 0 1 0
0 1 0 1
```

```
   10 11 12 13 14 - 7
3 4 5 6 7
```
