import math
import decimal as dc

def is_sq(x):
    sq = (x ** (1 / 2))
    if round(sq) ** 2 == x:
        return True
    return False

def compute(N):
    m, i = 10**8, None
    dc.getcontext().prec = 50
    pi = dc.Decimal('3.1415926535897932384626433832795028841971693993751') #First 50 digits of pi
    
    for n in range(N + 1):
        if is_sq(n) == False:
            x = pi * dc.Decimal(n).sqrt()
            v = (dc.Decimal(x).exp() + dc.Decimal(-x).exp()) / dc.Decimal(2)
            z = math.cos(math.pi*math.sqrt(n))
            
            t1 = min(math.ceil(v) - v, v - math.floor(v))
            t2 = min(math.ceil(z) - z, z - math.floor(z))
            if t1 < m:
                m, i = t1, -n
            if t2 < m:
                m, i = t2, n
    return i

# Find the result for n between 1 and 1000
result = compute(1000)

# Display the result
print(f"Best n: {result}")

# Calculate and display the actual value
if result > 0:
    value = math.cos(math.pi * math.sqrt(result))
    print(f"cos(π√{result}) = {value}")
    print(f"Nearest integer: {round(value)}")
    print(f"Distance to nearest integer: {abs(value - round(value)):.10f}")
else:
    # For negative n, use cosh
    n_abs = abs(result)
    dc.getcontext().prec = 50
    pi = dc.Decimal('3.1415926535897932384626433832795028841971693993751')
    x = pi * dc.Decimal(n_abs).sqrt()
    value = (dc.Decimal(x).exp() + dc.Decimal(-x).exp()) / dc.Decimal(2)
    print(f"cosh(π√{n_abs}) = {value}")
    print(f"Nearest integer: {round(float(value))}")
    print(f"Distance to nearest integer: {float(abs(value - round(float(value)))):.10f}")
