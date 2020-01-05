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
print(expr.doit())
result = expr.evalf(subs=dict(b=8036, m=7975, n=num_iterations, x=target))

# a different approach (gotten from simplifying the guy)
alt = ((b * ((1 - m**n)/(1 - m))) + m**n*x).evalf(subs=(dict(b=8036, m=7975, n=num_iterations, x=target)))

print(result % num_cards)
print('alt:')
print(alt % num_cards)
exit()

# that was just the test, now for the real thing:

num_iterations = 101741582076661
num_cards = 119315717514047
target = 2020

alt = ((b * ((1 - m**n)/(1 - m))) + m**n*x).evalf(subs=(dict(b=8036, m=7975, n=num_iterations, x=target)))
print('real answer:')
print(alt % num_cards)
