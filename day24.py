"""Day 24 part 2.

Start with `SWAPS` empty. Run until it errors.
Use the error output + the graphvis output to find the swap that needs to be
made, and add it to `SWAPS`. Repeat until the script prints the answer.
"""
from dataclasses import dataclass
from enum import Enum
import re

class Op(Enum):
    AND = "AND"
    OR = "OR"
    XOR = "XOR"

@dataclass
class Gate:
    lhs: str
    rhs: str
    output: str
    op: Op

op_re = re.compile("([a-z0-9]{3}) (AND|OR|XOR) ([a-z0-9]{3})")

gates = []

with open("./inputs/day24_gates.txt") as f:
    for line in f.read().split("\n"):
        if "->" not in line:
            continue
        opside, output = line.strip().split(" -> ")
        lhs, op, rhs = op_re.match(opside).groups()
        match op:
            case "AND":
                op_enum = Op.AND
            case "OR":
                op_enum = Op.OR
            case "XOR":
                op_enum = Op.XOR
            case _:
                raise ValueError(f"Invalid op: {op}")

        gates.append(Gate(lhs, rhs, output, op_enum))

SWAPS = [
    ("kmb", "z10"),
    ("tvp", "z15"),
    ("dpg", "z25"),
    ("mmf", "vdk"),
]

def do_swaps(swaps):
    for (s0, s1) in swaps:
        first = next(g for g in gates if g.output == s0)
        second = next(g for g in gates if g.output == s1)
        first.output = s1
        second.output = s0

do_swaps(SWAPS)

# We're looking for a relation of the form (I found this by staring at the
# graphvis output from day24.rs very hard.):
# X[N] ^ Y[N] -> B[N]
# X[N] & Y[N] -> A[N]
# CARRY[N-1] & B[N] -> I[N]
# I[N] | A[N] -> CARRY[N]
# CARRY[N-1] ^ B[N] -> Z[N]
#
# Initial conditions:
# CARRY[0] = X[0] & Y[0]
# Z[0] = X[0] ^ Y[0]

Cs = {}
Bs = {}
Is = {}
As = {}


for g in gates:
    if g.lhs in ("x00", "y00") and g.rhs in ("x00", "y00"):
        if g.op == Op.AND:
            Cs[0] = g.output
        else:
            assert g.op == Op.XOR
            Bs[0] = g.output


for i in range(1, 45):
    # Find B
    expected_inputs = (f"x{i:02d}", f"y{i:02d}")
    for g in gates:
        if g.lhs in expected_inputs and g.rhs in expected_inputs and g.op == Op.XOR:
            Bs[i] = g.output
            break
    if Bs.get(i) is None:
        raise RuntimeError(f"Missing B[{i}] with inputs {expected_inputs}")

    # Find A
    expected_inputs = (f"x{i:02d}", f"y{i:02d}")
    for g in gates:
        if g.lhs in expected_inputs and g.rhs in expected_inputs and g.op == Op.AND:
            As[i] = g.output
            break
    if As.get(i) is None:
        raise RuntimeError(f"Missing A[{i}] with inputs {expected_inputs}")

    # Find I
    expected_inputs = (Cs[i-1], Bs[i])
    for g in gates:
        if g.lhs in expected_inputs and g.rhs in expected_inputs and g.op == Op.AND:
            Is[i] = g.output
            break
    if Is.get(i) is None:
        print(f"I[{i-1}] = {Is[i-1]}")
        print(f"B[{i}] = {Bs[i]}")
        print(f"C[{i-1}] = {Cs[i-1]}")
        raise RuntimeError(f"Missing I[{i}] with inputs {expected_inputs}")

    # Find Z
    found_z = False
    expected_inputs = (Cs[i-1], Bs[i])
    for g in gates:
        if g.lhs in expected_inputs and g.rhs in expected_inputs and g.op == Op.XOR:
            found_z = True
            assert g.output == f"z{i:02d}", f"Output was {g.output} instead of z{i:02d}"
            break
    if not found_z:
        raise RuntimeError(f"Missing Z[{i}] with inputs {expected_inputs}")

    # Find C
    expected_inputs = (Is[i], As[i])
    for g in gates:
        if g.lhs in expected_inputs and g.rhs in expected_inputs and g.op == Op.OR:
            Cs[i] = g.output
            break
    if Cs.get(i) is None:
        raise RuntimeError(f"Missing C[{i}] with inputs {expected_inputs}")


result = [f for s in SWAPS for f in s]
result.sort()
print("Part 2:", ",".join(result))