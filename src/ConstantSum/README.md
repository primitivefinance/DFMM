# Constant Sum Market Maker
This will be all the background needed to understand the `GeometricMean` DFMM.

## Conceptual Overview
The `ConstantSum` DFMM gives the LP a portfolio that will allow exchange of a pair of tokens at a single price.
We can allow this price to be dynamically chosen.

## Core
We mark reserves as:
- $x \equiv \mathtt{rX}$
- $y \equiv \mathtt{rY}$

`ConstantSum` has one variable parameter:
- $P \equiv \mathtt{price}$ 

The **trading function** is:
$$
\boxed{\varphi(x,y,L;P) = \frac{x}{L} + \frac{y}{LP} -1}
$$
where $L$ is the **liquidity** of the pool. 

## Price
The reported price of the pool given the reseres is $P$.

## Pool initialization
The `ConstantSum` pool can be initialized with any given price and any given value of reserves. 
A user may supply $(x_0,y_0,P)$, then we find that:
$$
L_0 = x_0 + \frac{y_0}{P}
$$

## Swap 
We require that the trading function remain invariant when a swap is applied, that is:
$$
\frac{x+\Delta_X}{L + \Delta_L} + \frac{y+\Delta_Y}{P(L + \Delta_L)}-1 = 0
$$
where either $\Delta_X$ or $\Delta_Y$ is given by user input and the $\Delta_L$ comes from fees.

### Trade in $\Delta_X$ for $\Delta_Y$
If we want to trade in $\Delta_X$ for $\Delta_Y$, 
we first accumulate fees by taking 
$$
\Delta_L = (1-\gamma) \Delta_X.
$$
Then we can use our invariant equation and solve for $\Delta_Y$ in terms of $\Delta_X$ to get:
$$
\boxed{\Delta_Y = \gamma P \Delta_X}
$$

### Trade in $\Delta_Y$ for $\Delta_X$
If we want to trade in $\Delta_X$ for $\Delta_Y$, 
we first accumulate fees by taking 
$$
\Delta_L = \frac{1-\gamma}{P}\Delta_Y.
$$
Then we can use our invariant equation and solve for $\Delta_X$ in terms of $\Delta_Y$ to get:
$$
\boxed{\Delta_X = \frac{\gamma}{P} \Delta_Y}
$$

## Allocations and Deallocations
Allocations and deallocations should not change the price of a pool and since this pool only quotes a single price, any amount of reserves can be allocated at any time.
We need only compute the new $L$.
Specifically:
$$
L + \Delta_L = x+\Delta_X + \frac{y+\Delta_Y}{P}
$$


## Value Function via $L$ and $S$
Given that we treat $Y$ as the numeraire, we know that the portfolio value of a pool when $X$ is at price $S$ is:
$$
V = Sx(S) + y(S)
$$

In this case, the value function is that of a limit order and follows:
$$
V(L,S) = \begin{cases}
LS & \text{if } S \leq P \\
LP & \text{if } S \geq P
\end{cases}
$$

