b=8036
m=7975
x=499
n=2
c=10007

# the real values
#c = 119315717514047
#n = 101741582076661
#x = 2020


# the original one (https://www.wolframalpha.com/input/?i=%28%28sum+b+*+m**k%2C+k+%3D+0+to+n+-+1%29+%2B+%28m**n*x%29%29+mod+c)
f = ((((b * (m**n - 1))/(m - 1)) % c) + (((x % c)  * (m ** n % c )) % c)) % c
print(f)

## hw branch
#f = ((((b * (pow(m, n) - 1)) * (1 / (m - 1))) % c) + \
#        (x * pow(m, n)) % c) % c
#print(f)        # works
#
#
#f = ((((b * (pow(m, n) - 1)) * (1 / (m - 1))) % c) + \
#        ((x % c) * pow(m, n, c)) % c) % c       # works
#print(f)
#
#f = (
#        ((((b * (pow(m, n) - 1)) % c) * ((1 / (m - 1)) % c)) % c)
#
#        +
#
#        ((x % c) * pow(m, n, c)) % c
#    ) % c
#print(f)
#
#exit()

f = (
    (((b * (m**n - 1))/(m - 1)) % c)
    +
    (((x % c)  * (m ** n % c )) % c)
) % c
print(f)

# still working!!!
f = (
    (
        (
            (b * (m**n - 1)) * (1/(m - 1))
        )
        % c
    )
    +
    (((x % c)  * (m ** n % c)) % c)
) % c
print(f)

f = (
    (
        (
            (b * (m**n - 1) * (1/(m - 1)))
        )
        % c
    )
    +
    (((x % c)  * (m ** n % c)) % c)
) % c
print(f)

def extended_gcd(aa, bb):
    lastremainder, remainder = abs(aa), abs(bb)
    x, lastx, y, lasty = 0, 1, 1, 0
    while remainder:
        lastremainder, (quotient, remainder) = remainder, divmod(lastremainder, remainder)
        x, lastx = lastx - quotient*x, x
        y, lasty = lasty - quotient*y, y
    return lastremainder, lastx * (-1 if aa < 0 else 1), lasty * (-1 if bb < 0 else 1)

def modinv(a, m):
    g, x, y = extended_gcd(a, m)
    if g != 1:
            raise ValueError
    return x % m


# this is where it goes wrong. sub in %c into the above and it pharts
f = (
    (
        (
            (b % c)
            *
            ((m**n - 1) % c)
            *
            modinv(m - 1, c)
        )
        % c
    )
    +
    (((x % c)  * (m ** n % c)) % c)
) % c
print(f)

# works:
f = (
    (
        (
            (b % c)
            *
            ((pow(m, n, c) - 1) % c)
            *
            modinv(m - 1, c)
        )
        % c
    )
    +
    (((x % c)  * (m ** n % c)) % c)
) % c
print(f)

b=8036
m=7975
x=2020
n=101741582076661
c=119315717514047
#x = 877
#n = 8999
#c = 10007

# not sure if this works v
f = (
    (
        (
            (b % c)
            *
            ((pow(m, n, c) - 1) % c)
            *
            pow(m - 1, c - 2, c)
        )
        % c
    )
    +
    (((x % c)  * pow(m, n, c)) % c)
) % c
print(f)
