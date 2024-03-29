# Geometric Mean Market Maker
This will be all the background needed to understand the `GeometricMean` DFMM.

## Conceptual Overview
The `GeometricMean` DFMM gives the LP a portfolio that consists of a value-weighted ratio of the two assets based on the internal pricing mechanism.
If we pick the weight of the $X$-token to be $0.80$ and $0.20$ for the $Y$-token, then the LP will have a portfolio that is 80% in $X$ and 20% $Y$ by value.

## Core
We mark reserves as:
- $x \equiv \mathtt{rX}$
- $y \equiv \mathtt{rY}$

`GeometricMean` has two variable parameters:
- $w_X \equiv \mathtt{wX}$ 
- $w_Y \equiv \mathtt{wY}$ 
- These parameters must satisfy 
$$
w_x, w_y \geq 0 \\
w_x+w_y=1 
$$

The **trading function** is:
$$
\boxed{\varphi(x,y;w_X,w_Y) = \left(\frac{x}{L}\right)^{w_X} \left(\frac{y}{L}\right)^{w_Y} -1}
$$
where $L$ is the **liquidity** of the pool. 

## Price
The reported price of the pool given the reseres is:
$$
\begin{equation}
\boxed{P = \frac{w_X}{w_Y}\frac{y}{x}}
\end{equation}
$$

## Pool initialization
We need to initalize a pool from a given price $S_0$ and an amount of a token $x_0$ or $y_0$. 


### Given $x$ and price
Using the price formula above in Eq. (1), we can solve for $y_0$: 
$$
\boxed{y_0 = \frac{w_Y}{w_X}S_0 x_0}
$$

### Given $y$ and price
Again, using Eq. (1), we get:
$$
\boxed{x_0 = \frac{w_X}{w_Y}\frac{1}{S_0}y_0}
$$

## Swap 
We require that the trading function remain invariant when a swap is applied, that is:
$$
\left(\frac{x+\Delta_X}{L + \Delta_L}\right)^{w_X} \left(\frac{y+\Delta_Y}{L + \Delta_L}\right)^{w_Y}-1 = 0
$$
where either $\Delta_X$ or $\Delta_Y$ is given by user input and the $\Delta_L$ comes from fees.

In general, with a fee parameter $\gamma$, we have:
$$
\Delta_L = w_R(1-\gamma) L \frac{\Delta_R}{R}
$$
where $R$ represents either token $X$ or $Y$.

### Trade in $\Delta_X$ for $\Delta_Y$
If we want to trade in $\Delta_X$ for $\Delta_Y$, 
we use our invariant equation and solve for $\Delta_Y$ in terms of $\Delta_X$ to get:
$$
\boxed{\Delta_Y = \left(\frac{L + \Delta_L}{(x+\Delta_X)^{w_X}} \right)^{\frac{1}{w_Y}} - y}
$$

### Trade in $\Delta_Y$ for $\Delta_X$
If we want to trade in $\Delta_X$ for $\Delta_Y$, 
we use our invariant equation and solve for $\Delta_Y$ in terms of $\Delta_X$ to get:
$$
\boxed{\Delta_X = \left(\frac{L + \Delta_L}{(y+\Delta_Y)^{w_Y}} \right)^{\frac{1}{w_X}} - x}
$$


## Allocations and Deallocations
**Input $\delta_X$:** If a user wants to allocate a specific amount of $\delta_X$, then it must be that:
$$
\frac{x}{L} = \frac{x+\Delta_X}{L+\Delta_L}
$$
which yields:
$$
\Delta_L = L \frac{\Delta_X}{x}
$$
Then it must be that since the ratio of reserves cannot change.
$$
\Delta_Y = y\frac{\Delta_X}{x} 
$$

**Input $\Delta_Y$:** To allocate a specific amount of $\Delta_Y$, then it must be that:
$$
\frac{y}{\mu L} = \frac{y+\Delta_Y}{\mu(L+\Delta_L)}
$$
which yields:
$$
\Delta_L = L \frac{\Delta_Y}{y}
$$
and we likewise get
$$
\Delta_X = x\frac{\Delta_Y}{y}
$$


## Value Function via $L$ and $S$
Given that we treat $Y$ as the numeraire, we know that the portfolio value of a pool when $X$ is at price $S$ is:
$$
V = Sx(S) + y(S)
$$

We can solve for the following using the price and the trading function:
$$
x = \frac{L}{(\frac{w_y}{w_x}S)^{w_y}}\\
y = \frac{\left(\frac{w_x}{w_y}\frac{1}{S}\right)^{w_x}}{L}
$$
Plugging these into our value function, we get:
$$
\boxed{V(L,S)=LS^{w_X}\left(\left( \frac{w_X}{w_Y}\right)^{w_Y}+\left( \frac{w_Y}{w_X}\right)^{w_X}\right)}
$$

