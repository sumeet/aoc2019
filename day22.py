from sympy import Mod, Add, Symbol, Pow, Integer, Mul
from sympy import init_printing

OldMod = Mod
OldAdd = Add
OldPow = Pow
OldMul = Mul

def Mod(*args, **kwargs):
    return OldMod(*args, **kwargs, evaluate=False)
def Add(*args, **kwargs):
    return OldAdd(*args, **kwargs, evaluate=False)
def Pow(*args, **kwargs):
    return OldPow(*args, **kwargs, evaluate=False)
def Mul(*args, **kwargs):
    return OldMul(*args, **kwargs, evaluate=False)

init_printing()

e = Mod(
        Add(
            Mod(
                Mul(
                    Mod(Symbol('b'), Symbol('c')),
                    Mod(Pow(Add(Symbol('m'), Integer(-1)), Integer(-1)), Symbol('c')),
                    Mod(Add(Pow(Symbol('m'), Symbol('n')), Integer(-1)), Symbol('c')),
                ),
                Symbol('c')
            ),
            Mod(
                Mul(
                    Mod(Pow(Symbol('m'), Symbol('n')), Symbol('c')),
                    Mod(Symbol('x'), Symbol('c')),
                ),
                Symbol('c')
            ),
        ),
    Symbol('c'))
print(e)
print(e.evalf(subs=dict(b=8036, x=499, n=2, m=7975, c=10007)))
