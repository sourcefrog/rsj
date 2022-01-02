# Negative and Minus

In J, minus is not associative, and is evaluated from right to left.

So this does what you might expect:

       10 - 1
    9

But this may be surprising:

       100 - 10 - 1
    91
       100 - 10
    90

Negative numbers are represented with a leading underscore, to disambiguate
them from a monadic negation operator:

       100 - 200
    _100
       -1
    _1
       -_1
    1
       ----9
    9

*negative* applied to a matrix works element-at-a-time to negate the matrix:

       - 10 20 30 _40 0
    _10 _20 _30 40 0
