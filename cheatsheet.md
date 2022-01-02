# RSJ cheatsheet

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

## Number forms

| Form | Meaning           |
| ---- | ----------------- |
| `_`  | Positive infinity |
| `__` | Negative infinity |
| `_3` | Negative 3        |

## Glossary

See <https://code.jsoftware.com/wiki/Vocabulary/Glossary>

- array
- atom
- axis
