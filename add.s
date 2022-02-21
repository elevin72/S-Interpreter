
# load X1 to Y and Z1
[A]: if X1 != 0 GOTO L1
Z3 <- Z3 + 1
if Z3 != 0 GOTO B
[L1]:
X1 <- X1 - 1
Y <- Y + 1
Z1 <- Z1 + 1
Z3 <- Z3 + 1
if Z3 != 0 GOTO A

# load X2 to Z2
[B]: if X2 != 0 GOTO L2
Z3 <- Z3 + 1
if Z3 != 0 GOTO D
[L2]:
X2 <- X2 - 1
Z2 <- Z2 + 1
Z3 <- Z3 + 1
if Z3 != 0 GOTO B
# add Z2 to Y while also resetting X2

[D]: if Z2 != 0 GOTO L3
Z3 <- Z3 + 1
if Z3 != 0 GOTO F
[L3]:
Y <- Y + 1
Z2 <- Z2 - 1
X2 <- X2 + 1
X3 <- X3 + 1
if Z3 != 0 GOTO D
# reset Z1

[F]: if Z1 != 0 GOTO L4
Z3 <- Z3 + 1
if Z3 != 0 GOTO E
[L4]:
X1 <- X1 + 1
Z1 <- Z1 - 1
X3 <- X3 + 1
if Z3 != 0 GOTO F









