# RSJ cheatsheet

## Evaluation

J is evaluated from right to left, with no concept of operator precedence,
except that parenthesis cause the subexpression to be evaluated first in the
usual way. Another way to think about this is that the right argument of any
verb is the entire expression to the right, until reaching a parenthesis. For
example,

```
   10*3+2
50
   2+10*3
32
   (10*3)+2
32
```

## Verbs

| Verb   | Name       | Meaning                                                                                                |
| ------ | ---------- | ------------------------------------------------------------------------------------------------------ |
| -. y   | not        | 1 if y=0; 0 if y=1; (1-y) if y is between 0 and 1 (the inverse probability); otherwise a domain error. |
| - y    | negate     |                                                                                                        |
| x - y  | minus      |                                                                                                        |
| x + y  | plus       |                                                                                                        |
| # y    | tally      | the number of items on the leading axis                                                                |
| $ y    | shape of   | a list: empty for an atom, otherwise giving the length of each axis of y                               |
| % y    | reciprocal | 1 % y                                                                                                  |
| x % y  | divide     | division; 0%0 = 0; division by nonzero gives signed infinity                                           |
| \* y   | signum     | 0 if y=0; \_1 if y<0; otherwise 1                                                                      |
| x \* y | times      | \_\*0 = 0                                                                                              |
| i. y   | integers   | a list of i integers starting from 0 if y is >=0; other cases are unimplemented                        |

## Number forms

| Form   | Meaning                      |
| ------ | ---------------------------- |
| `_`    | Positive infinity            |
| `__`   | Negative infinity            |
| `_3`   | Negative 3                   |
| `1e6`  | Scientific form; one million |
| `1e_3` | Negative exponent; 0.001     |

## Glossary

See <https://code.jsoftware.com/wiki/Vocabulary/Glossary>

- array
- atom
- axis
