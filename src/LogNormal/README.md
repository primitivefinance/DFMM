# Log Normal Market Maker
This will be all the background needed to understand the `LogNormal` DFMM.

## Conceptual Overview
The `LogNormal` DFMM provides the LP with a a log-normal shaped liquidity distribution centered around a price $\mu$ with a width given by $\sigma$.

Note that this strategy can be made time-dependent by an additional $\tau$ parameter that is the time til the pool will "expire".
In this case, the LN trading function provides the LP with a payoff that is equivalent to a Black-Scholes covered call option with strike $K = \mu$, implied volatility $\sigma$, and time to expiration $\tau$. 
We do not cover this explicitly here.

## Core
We mark reserves as:
- $x \equiv \mathtt{rX}$
- $y \equiv \mathtt{rY}$

`LogNormal` has two variable parameters:
- $\mu \equiv \mathtt{mean}$
- $\sigma \equiv \mathtt{width}$
- These parameters must satisfy:
$$\mu > 0\\
\sigma > 0$$

The trading function for this DFMM is given by
$$\begin{equation}
\boxed{\varphi(x,y,L;\mu,\sigma) = \Phi^{-1}\left(\frac{x}{L}\right)+\Phi^{-1}\left(\frac{y}{\mu L}\right)+\sigma}
\end{equation}$$
where $L$ is the **liquidity** of the pool.

Given the domain of $\Phi^{-1}$ ([inverse Gaussian CDF](https://en.wikipedia.org/wiki/Normal_distribution)) we can see that $x\in [0,L]$ and $y\in [0,\mu L]$.
As the pool's liquidity increases, the maximal amount of each reserve increases and both are scaled by the same factor, which is also how we decide how to compute fees.

## Useful Notation
We will use the following notation:
$$\begin{equation}
d_1(S;\mu,\sigma) = \frac{\ln\frac{S}{\mu}+\frac{1}{2}\sigma^2 }{\sigma}
\end{equation}
$$
$$
\begin{equation}
d_2(S;\mu,\sigma) = \frac{\ln\frac{S}{\mu}-\frac{1}{2}\sigma^2 }{\sigma}
\end{equation}
$$

## Price
We can provide the price of the pool given either of the reserves:
$$\begin{equation}
\boxed{P_X(x, L; \mu, \sigma) = \mu \exp\left(\Phi^{-1} \left(1 - \frac{x}{L}\right) \sigma  - \frac{1}{2} \sigma^2 \right)} 
\end{equation}$$

$$\begin{equation}
\boxed{P_Y(y, L; \mu, \sigma) = \mu \exp\left(\Phi^{-1} \left(\frac{y}{\mu L}\right) \sigma + \frac{1}{2} \sigma^2 \right)}
\end{equation}$$

Note that other DFMMs such as the `GeometricMean` have a price that can be determined from both reserves at once, so we typically do not write $P_X$ and $P_Y$.

## Pool initialization
When the pool is initialized, we need to determine the value of $L$ and the other reserve.
The user will provide a price $S_0$ and an amount $x_0$ or an amount of $y_0$ that they wish to tender and we can get the other reserve and $L$ from the trading function.

We can recall that get that:
$$\begin{equation}
\frac{x}{L} = 1-\Phi((d_1(S;\mu,\sigma))
\end{equation}$$
and
$$\begin{equation}
\frac{y}{\mu L} = \Phi(d_2(S;\mu,\sigma))
\end{equation}$$

### Given $x$ and price
Suppose that the user specifies the amount $x_0$ they wish to allocate and they also choose a price $S_0$.
We first get $L_0$ using (6):
$$\begin{equation}
\boxed{L_0 = \frac{x}{1-\Phi(d_1(S;\mu,\sigma))}}
\end{equation}$$
From this, we can get the amount $y_0$ 
$$
\boxed{y_0 = \mu L_0 \Phi(d_2(S;\mu,\sigma, \tau))}
$$


### Given $y$ and price
The work here is basically a mirrored image of the above.
We get $L_0$:
$$\begin{equation}
\boxed{L_0 = \frac{y}{\mu\Phi(d_2(S;\mu,\sigma))}}
\end{equation}$$
Suppose that the user specifies the amount $y$ they wish to allocate and they also choose a price $S$.
Now we need to get $x$:
$$\boxed{x_0 = L_0 \left(1-\Phi\left(d_1(S;\mu,\sigma)\right)\right)}$$

## Allocations and Deallocations
Allocations and deallocations should not change the price of a pool, and hence the ratio of reserves cannot change while increasing liquidity the correct amount.

**Input $\Delta_X$:** If a user wants to allocate a specific amount of $\Delta_X$, then it must be that:
$$
\frac{x}{L} = \frac{x+\Delta_X}{L+\Delta_L}
$$
which yields:
$$
\boxed{\Delta_L = L \frac{\Delta_X}{x}}
$$
Then it must be that
$$
\boxed{\Delta_Y = y\frac{\Delta_X}{x}}
$$

**Input $\Delta_Y$:** To allocate a specific amount of $\Delta_Y$, then it must be that:
$$
\frac{y}{\mu L} = \frac{y+\Delta_Y}{\mu(L+\Delta_L)}
$$
which yields:
$$
\boxed{\Delta_L = L \frac{\Delta_Y}{y}}
$$
and we likewise get
$$
\boxed{\Delta_X = x\frac{\Delta_Y}{y}}
$$

## Swaps
We require that the trading function remain invariant when a swap is applied, that is:
$$\Phi^{-1}\left(\frac{x+\Delta_X}{L + \Delta_L}\right)+\Phi^{-1}\left(\frac{y}{\mu (L + \Delta_L)}\right)+\sigma = 0$$
where either $\Delta_X$ or $\Delta_Y$ is given by user input and the $\Delta_L$ comes from fees.

### Trade in $\Delta_X$ for $\Delta_Y$
If we want to trade in $\Delta_X$ for $\Delta_Y$, 
we first accumulate fees by taking 
$$
\textrm{Fees} = (1-\gamma) \Delta_X.
$$
Then, we treat these fees as an allocation, therefore:
$$
\boxed{\Delta_L = \frac{P}{Px +y}L\frac{(1-\gamma)\Delta_X}{x}}
$$
where $P$ is the price of token $X$ quoted by the pool itself (i.e., using $P_X$ or $P_Y$ in Eq. (4) or (5) above).
Then we can use our invariant equation and solve for $\Delta_Y$ in terms of $\Delta_X$ to get:
$$\boxed{\Delta_Y = \mu (L+\Delta_L)\cdot\Phi\left(-\sigma-\Phi^{-1}\left(\frac{x+\Delta_X}{L+\Delta_L}\right)\right)-y}$$

### Trade in $\Delta_Y$ for $\Delta_X$
If we want to trade in $\Delta_X$ for $\Delta_Y$, 
we first accumulate fees by taking 
$$
\boxed{\Delta_L = L\frac{(1-\gamma)\Delta_X}{Px +y}}
$$
Then we can use our invariant equation and solve for $\Delta_X$ in terms of $\Delta_Y$ to get:
$$
\boxed{\Delta_X = (L+\Delta_L)\cdot\Phi\left(-\sigma-\Phi^{-1}\left(\frac{y+\Delta_Y}{\mu(L+\Delta_L)}\right)\right)-x}
$$

## Value Function on $L(S)$
Relate to value on $V(L,S)$ and $V(x,y)$. 
Then we can use this to tokenize. We have $L_X(x, S)$ and $L_Y(y, S)$.
We know that:
$$V = Sx + y$$
We can get the following from the trading function:
$$
x = LS\cdot\left(1-\Phi\left(\frac{\ln\frac{S}{\mu}+\frac{1}{2}\sigma^2}{\sigma}\right)\right)\\
y = \mu\cdot L\cdot \Phi\left(\frac{\ln\frac{S}{\mu}-\frac{1}{2}\sigma^2}{\sigma}\right)
$$
Therefore:
$$
\boxed{V(L,S) = L\left( S\cdot\left(1-\Phi\left(\frac{\ln\frac{S}{\mu}+\frac{1}{2}\sigma^2}{\sigma}\right)\right) + \mu\cdot \Phi\left(\frac{\ln\frac{S}{\mu}-\frac{1}{2}\sigma^2}{\sigma}\right)\right)}
$$

### Time Dependence
Note that $L$ effectively changes as parameters of the trading function change.
To see this, note that the trading function must always satisfy:
$$\Phi^{-1}\left(\frac{x}{L}\right)+\Phi^{-1}\left(\frac{y}{
\mu L}\right) + \sigma  = 0.$$
For new parameters $\mu'$ and $\sigma'$ we must find an $L'$ so that the trading function is satisfied:
$$\Phi^{-1}\left(\frac{x}{L'}\right)+\Phi^{-1}\left(\frac{y}{\mu'L'}\right) + \sigma' = 0.$$
We can find this new $L'$ using a root finding algorithm. 