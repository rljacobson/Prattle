# Binding Power's Relationship to Operator Properties

| Affix    | Associativity | NBP     | LBP  | RBP     |
|:---------|:--------------|:--------|:-----|:--------|
| Infix    | Left          | LBP     | Prec | LBP + 1 |
|          | Right         | LBP     | Prec | LBP     |
|          | Non           | LBP - 1 | Prec | LBP + 1 |
|          | Full          | LBP - 1 | Prec | LBP - 1 |
| Prefix   | N/A           | Prec    | -1   | -1      |
| Matchfix | N/A           | 0       | -1   | N/A     |
| Postfix  | N/A           | -1      | Prec | -1      |

Infix Full has to look ahead to see if it coalesces.
