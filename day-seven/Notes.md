# Problem Notes

Formulas evaluated as:

190 = (10 _ 19)
3267 = ((81 _ 40) _ 27)

Since it is evaluated left-to-right, we can always compare and produce a tree.

if 3267 % 27 == 0:
- evaluate  3267/27, (81 _ 40)
- evaluate 3267-27, (81 _ 40)

fn returns Option<Vec<Operator>>


## Update

get_operators -> Vec<Vec<Operators>>

for example: '3267: 81 40 27' returns:
- [[+,*],[*,+]]

## Part 2: Concatenation

left and right

result = left * 10^floor(log10(right)) + right


A,B,C,D,E,F

6 + 5 * 6 + $ * 5

result: 102
right: 2
 
result - right = 98
100 / 10^1

10

## Collation Split Target

7290 -> 3 Operators. 3 potential splits.

Concat -> 5 * indexes

0 || 7290
7 || 290 == A || (B O C) O D
72 || 90 == (A O D) || (C O D)
729 || 0 == (A O D) O C || C
7290 || 0


Check from the right -> left (if you check || at the back and you find nothing... you can check at the front)

There is N potential values that the AST can be to the right for a target, where N = Log10(Target)

Target 686, Right = 6

Target - Right = 680
/ 10^Digits Right
 