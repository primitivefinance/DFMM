# Geometric Mean Market Maker
This will be all the background needed to understand the `GeometricMean` DFMM.

## Conceptual Overview
The `GeometricMean` DFMM gives the LP a portfolio that consists of a value-weighted ratio of the two assets based on the internal pricing mechanism.
If we pick the weight of the $X$-token to be $0.80$ and $0.20$ for the $Y$-token, then the LP will have a portfolio that is 80% in $X$ and 20% $Y$ by value.

## Core
We mark the vector reserves as:
- $\boldsymbol{r} \in \R^n_+ \equiv \mathtt{reserves}$

`GeometricMean` also assigns weights to each of the reserves:
- $\boldsymbol{w} \in [0,1]^n \equiv \mathtt{weights}$ 
- These parameters must satisfy 
$$
\sum_{i=0}^{n-1} w_i = 1
$$

The **trading function** is:
$$
\boxed{\varphi(\mathbf{r}, L;\mathbf{w}) = \prod_{i=0}^{n-1}\left(\frac{r_i}{L}\right)^{w_i} -1}
$$
where $L$ is the **liquidity** of the pool. 

## Price
The reported price of Token $i$ with respect to Token $j$is:
$$
\begin{equation}
\boxed{P_{ij} = \frac{w_i}{w_j}\frac{r_j}{r_i}}
\end{equation}
$$
Note $P_{ij} \neq P_{ji}$ in general. 
Furthermore, assuming that token $n-1$ is the numeraire, then we can just write the price of Token $i$ as:
$$
\begin{equation}
P_i = \frac{w_i}{w_{n-1}}\frac{r_{n-1}}{r_i}
\end{equation}
$$


## Pool initialization
To initalize a pool, we must first specify the initial reserves and the weights.
Equivalently, and often more conveniently, we can specify the initial prices, the weights, and an amount of the numeraire token (Token $n-1$).

### Given Reserves and Weights
If we are given the reserves and the weights, then we can solve for the liquidity by:
$$
L = \prod_{i=0}^{n-1} r_i^{w_i}.
$$

### Given Prices and Weights
If we have $P_i$ for all $i$ (noting that $P_{n-1} = 1$ by definition), then we can solve for the amount of reserve $r_i$ by:
$$
r_i = \frac{w_i}{w_{n-1}}\frac{r_{n-1}}{P_i},
$$
since the user will also have to specify the amount of the numeraire token.
We then compute $L$ as before.

## Swap 
We require that the trading function remain invariant when a swap is applied, that is:
$$
\boxed{\varphi(\mathbf{r} + \mathbf{\Delta_r}, L + \Delta_L;\mathbf{w}) = \prod_{i=0}^{n-1}\left(\frac{r_i + \Delta_{r_i}}{L + \Delta_L}\right)^{w_i} -1}.
$$
In our case, we will deal with only single token input and single token output swaps.
Also, the $\Delta_L \geq 0$ and, in particular, is greater than zero when there is a nonzero swap fee.

In general, with a fee parameter $\gamma$, we have a change in $L$ when tendering $r_i$ given by:
$$
\Delta_L = w_i(1-\gamma) L \frac{\Delta_{r_i}}{r_i}.
$$


### Trade in $\Delta_{r_i}$ for $\Delta_{r_j}$
If we want to trade in $\Delta_X$ for $\Delta_Y$, 
we use our invariant equation and solve for $\Delta_Y$ in terms of $\Delta_X$ to get:
$$
\boxed{\Delta_{r_j} = \left(\frac{L + \Delta_L}{\displaystyle{\prod_{k \in \{0,\dots, n-1\} \setminus j}^{n-1} r_k^{w_k}}} \right)^{\frac{1}{w_X}} - r_j}
$$
This amount will be negative and the equation should be negated if you want a positive amount out.


## Allocations and Deallocations
**Input $\Delta_{r_i}$:** If a user wants to allocate a specific amount of Token $r_i$, then it must be that:
$$
\frac{r_i}{L} = \frac{r_i+\Delta_{r_i}}{L+\Delta_L}
$$
which yields:
$$
\Delta_L = L \frac{\Delta_X}{x}
$$
Then it must be that since the ratio of reserves cannot change, so the amount that the other reserves change by is:
$$
\Delta_{r_j} = r_j\frac{\Delta_{r_i}}{r_i} 
$$



## Value Function via $L$ and $S$
To look at a value function, we will consider a pool with just a token $X$ and a numeraire $Y$.
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

For pools with $n$-tokens, the value function for any pair $r_i$ and the numeraire $r_{n-1}$ is given as above.