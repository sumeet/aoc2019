from sympy import symbols
from sympy import Sum

m, x, b, d, n, k = symbols('m x b d n k')

#x = x
#for i in range(10):
    #print(f'{i}: {x.expand()}')
    #x = (m*x) + b

num_iterations = 2
num_cards = 10007
target = 499

expr = (Sum((b * m**k), (k, 0, n - 1)) + ((m ** n) * x))
result = expr.evalf(subs=dict(b=8036, m=7975, n=num_iterations, x=target))
print(result % num_cards)
