<!-- ADD TAU IN -->

# Log Normal
This will be all the math involved with the Log Normal (LN?) trading function.

## Conceptual Overview
The normal strategy provides the LP with a a log-normal shaped liquidity distribution centered around a price $K$ with a width given by $\sigma$.
This strategy can be made time-dependent by an additional $\tau$ parameter that is the time til the pool will "expire".
In this case, the LN trading function provides the LP with a payoff that is equivalent to a Black-Scholes covered call option with strike $K$, implied volatility $\sigma$, and time to expiration $\tau$. The parameters $K$ and $\sigma$ can also be made time dependent.

## Core
LN has three variable parameters:
- $K \equiv \mathtt{strike}$
- $\sigma \equiv \mathtt{volatility}$
- $\tau \equiv \mathtt{time\_to\_expiration}$
- These parameters must satisfy:
$$
K>0\\
\sigma>0\\
\tau>0
$$
Let's separate variables from parameters in function inputs with $;$ just to stay clear (though it may be verbose).
The trading function for this strategy is given by
$$
\begin{equation}
\varphi(x,y,L;K,\sigma, \tau) = \Phi^{-1}\left(\frac{x}{L}\right)+\Phi^{-1}\left(\frac{y}{KL}\right)+\sigma \sqrt{\tau}.
\end{equation}
$$
In the equation above, $x$ and $y$ are reserves, and $L$ is the *liquidity*. 
We can put:
$$
L \equiv \mathtt{liquidity}
$$
Note that $L$ has units of Token (this is what we want).
Given the domain of $\Phi^{-1}$ we can see that $x\in\[0,L\]$ and $y\in\[0,KL\]$.
As the pool's liquidity increases, the maximal amount of each reserve increases and both are scaled by the same factor (this is what we want).

## Useful Notation
We will use the following notation:
$$
\begin{equation}
d_1(S;K,\sigma,\tau) = \frac{\ln\frac{S}{K}+\frac{1}{2}\sigma^2 \tau }{\sigma \sqrt{\tau}}
\end{equation}
$$
$$
\begin{equation}
d_2(S;K,\sigma,\tau) = \frac{\ln\frac{S}{K}-\frac{1}{2}\sigma^2 \tau }{\sigma \sqrt{\tau}}
\end{equation}
$$

## Price
If we compute the derivatives and simplify the expression, we get that the pool price is given by either:
$$
\begin{equation}
\boxed{P_X(x, L; K, \sigma, \tau) = K \exp\left(\Phi^{-1} \left(1 - \frac{x}{L}\right) \sigma \sqrt{\tau} - \frac{1}{2} \sigma^2 \tau \right)} 
\end{equation}
$$
$$
\begin{equation}
\boxed{P_Y(y, L; K, \sigma, \tau) = K \exp\left(\Phi^{-1} \left(\frac{y}{KL}\right) \sigma \sqrt{\tau} + \frac{1}{2} \sigma^2 \tau \right)}
\end{equation}
$$
Do not that other DFMMs such as the [geometric mean market maker](./geometric_mean.md) have a price that can be determined from both reserves at once so we typically do not write `P_X` and `P_Y`.

## Determining $L$ from Price and Reserves
There are a few distinct times where we need to determine the value of $L$, but they all come down to liquidity being deposited into the pool and not from swaps.
We want to disentangle swaps and liquidity provision/donation.
Note that for [G3M](./geometric_mean.md), we don't have this same need as the $L$ is determined by the trading function explicitly.

### $L$ from $x$
Without showing all the work, we can recall that $\frac{x}{L}$ is one of the option binaries:
$$
\begin{equation}
\frac{x}{L} = 1-\Phi((d_1(S;K,\sigma, \tau))
\end{equation}
$$
Since we know $x$ and we know $S$, we can solve for $L$ to find:
$$
\begin{equation}
\boxed{L_X(x, S; K, \sigma, \tau) = \frac{x}{1-\Phi(d_1(S;K,\sigma, \tau))}}
\end{equation}
$$

### $L$ from $y$
The work here is basically a mirrored image of the above.
$$
\begin{equation}
\frac{y}{KL} = \Phi(d_2(S;K,\sigma, \tau))
\end{equation}
$$
From here we get $L$:
$$
\begin{equation}
\boxed{L_Y(y, S; K, \sigma, \tau) = \frac{y}{K\Phi(d_2(S;K,\sigma, \tau))}}
\end{equation}
$$


## Pool Initialization
When the pool is initialized, we need to determine the value of $L$.
The user will provide a price $S$ and an amount of $x$ or an amount of $y$ that they wish to tender. 
From there, we should be able to determine how much of both tokens must be allocated as well as the value of $L$.

### Specifying $x$
Suppose that the user specifies the amount $x_0$ they wish to allocate and they also choose a price $S_0$.
Further, we need to know how much $y_0$ to allocate, which we can also use the other binary in Equation $(8)$.
At this point, we know $S$ and $L$ and so we can get:
$$
\boxed{y_0 = y(x_0,S_0;K,\sigma, \tau) = K L_X(x_0, S_0; K, \sigma, \tau) \Phi(d_2(S;K,\sigma, \tau))}
$$
Note that the above is not simplified and likely could be drastically simplified.


### Specifying $y$
Suppose that the user specifies the amount $y$ they wish to allocate and they also choose a price $S$.
Now we need to get $x$:
$$
\boxed{x_0 = x(y_0, S_0) = L_Y(y_0, S_0; K, \sigma, \tau) \left(1-\Phi\left(d_1(S;K,\sigma, \tau)\right)\right)}
$$

## Adding/Removing Liquidity
When a user adds liquidity, they will specify an amount of $x$ or an amount of $y$, and the pool's price $S$ and liquidity $L$ will already be known. 
When adding liquidity, we assume that price will not change whatsoever and only the value of $L$ will change.

### Specifying $x$
Given some amount of $\delta_x$ the user wants to add, we can just use the equation for $L(x,S)$ above to get:
$$
L_X(x+\delta_x,S; K, \sigma, \tau) = \frac{x+\delta_x}{1-\Phi\left(d_1(S;K,\sigma, \tau)\right)}
$$
In fact, $L$ is linear in the first variable, so:
$$
L_X(x+\delta_x,S; K, \sigma, \tau) = L_X(x,S; K, \sigma, \tau)+\underbrace{L_X(\delta_x,S; K, \sigma, \tau)}_{\delta_L}
$$
can be used to make the calculation easier. In fact, if we just want to determine $\delta_L$, we can just use the above equation and subtract $L_X(x,S)$ from both sides and use our $S= P_X(x)$ equation to get:
$$
\boxed{\delta_{L,X} = \delta_X \frac{L}{x}}
$$
where $x$ is the amount of $X$ reserves in the pool before the liquidity is added.

### Specifying $y$
Given some amount of $\delta_y$ the user wants to add, we can just use the equation for $L(y,S)$ above to get:
$$
L_Y(y+\delta_y,S; K, \sigma, \tau) = \frac{y+\delta_y}{K\Phi\left(d_1(S;K,\sigma, \tau)\right)}
$$
Again, $L$ is linear in the first variable. We can use the same technique above to show that:
$$
\begin{equation}
\boxed{\delta_{L,Y} = \delta_Y \frac{L}{y}}
\end{equation}
$$

### Getting the Inputs
Given that we have either $\delta_{L,X}$ or $\delta_{L,Y}$, we can use the above equations to get the amount of $X$ or $Y$ that must be added to the pool.
Specifically, if the user wants to add $\delta_X$, we find $\delta_{L,X}$ then use Equation (10) like so:
$$
\delta_{L,X} = \delta_Y \frac{L}{y}.
$$
The other way follows mutatis mutandis.

### Removing Liquidity
When a user removes liquidity, they will specify an amount of $x$ or an amount of $y$, and the pool's price $S$ and liquidity $L$ will already be known. 
When removing liquidity, we assume that price will not change whatsoever and only the value of $L$ will change.
We can just use the same formulation as above and note that $\delta_x$ and $\delta_y$ may be positive or negative.

## Swaps
When a user swaps, it must be that the trading function $\varphi$ remains invariant. 
Specifically,
$$
\varphi(x+\Delta_X, y+\Delta_Y, L+\delta_L) = 0.
$$
**Note again I'm allowing for $\Delta_X$ and $\Delta_Y$ to be positive or negative.**
In absence of fees, the liquidity $L$ is invariant, that is $\delta_L = 0$.
We will continue to use the lowercase $\delta$ for fee-based changes and $\Delta$ for swap inputs and outputs to distinguish these components.
To compute a swap, it is a matter of finding the $\Delta_X(\Delta_Y)$ or $\Delta_Y(\Delta_X)$ that satisfies the above equation. 

### With Fees
Assume now that there is a fee parameter $\gamma$ such that the fee invested into the pool is $1-\gamma$. 
Assume further that the fee is always taken out of the input token for the swap.
Think of the swap as a two step process:

1. Adding liquidity. E.g., $\delta_Y \coloneqq (1-\gamma)\Delta_Y$. 
This is the amount of the input token that is added to the pool and it is what is used to calculate the change in liquidity $\delta_L$.
From here, we can imagine that the swapper then takes temporary debt in adding $\delta_Y$ to the pool where the $\delta_Y$ can by found by using Equation $(10)$ and we specifically get:
$$
\delta_Y = y \frac{\delta_L}{L}
$$
2. Computing a no-fee swap with the remaining amount of the input token. E.g., $\widetilde{\Delta_x} \coloneqq \gamma\Delta_x$.
Note at this point, the reserves are then $x+\delta_x$ and $y+\delta_y$ and the liquidity $L+\delta_L$. 
So we must use these in the swap calculation.
Then we can use all of the rules we defined here.

#### $\Delta_y$ given $\Delta_x$
Suppose that the user wants to swap $x$ for $y$ and the price is $S$.
They specifically tender $\Delta_x$ and the fee parameter is $\gamma$.
Now $\delta_x=(1-\gamma)\Delta_x$ and $\widetilde{\Delta_x}=\gamma\Delta_x$.
From this we get 
$$
\delta_L=L_X(\delta_x, S)=\frac{\delta_x}{1-\Phi\left(\frac{\ln\frac{S}{K}+\frac{1}{2}\sigma^2}{\sigma}\right)}
$$
Using the trading function, we solve for $\Delta_y$:
$$
\Phi^{-1}\left(\frac{x+\Delta_x}{L+\delta_L}\right)+\Phi^{-1}\left(\frac{y+\Delta_y}{K(L+\delta_L)}\right)=-\sigma\\

\boxed{\Delta_y(\Delta_x) = K(L+\delta_L)\cdot\Phi\left(-\sigma-\Phi^{-1}\left(\frac{x+\Delta_x}{L+\delta_L}\right)\right)-y}
$$

#### $\Delta_x$ given $\Delta_y$
<!-- TODO UPDATE THIS MATH -->
Suppose that the user wants to swap $y$ for $x$ and the price is $S$.
They specifically tender $\Delta_y$ and the fee parameter is $\gamma$.
Now $\delta_y=(1-\gamma)\Delta_y$ and $\widetilde{\Delta_y}=\gamma\Delta_y$.
From this we get 
$$
\delta_L=L_Y(\delta_y, S)=\frac{\delta_y}{K\cdot\Phi\left(\frac{\ln\frac{S}{K}-\frac{1}{2}\sigma^2}{\sigma}\right)}
$$
Using the trading function, we solve for $\Delta_x$:
$$
\Phi^{-1}\left(\frac{x+\Delta_x}{L+\delta_L}\right)+\Phi^{-1}\left(\frac{y+\Delta_y}{K(L+\delta_L)}\right)=-\sigma\\
\boxed{\Delta_x(\Delta_y) = (L+\delta_L)\cdot\Phi\left(-\sigma-\Phi^{-1}\left(\frac{y+\Delta_y}{K(L+\delta_L)}\right)\right)-x}
$$

## Arbitrage Math

### Raising the price
When we need to raise the price, we need to tender in $Y$. 
If the current price is $S$ and we want to raise it to $S'$, then we need to tender in $Y$ such that we go from $y$ to $y'$ and:
$$
y' = K\cdot (L+\delta_L) \cdot \Phi\left(\frac{\ln\frac{S'}{K}-\frac{1}{2}\sigma^2}{\sigma}\right)
$$ 
and we know $\delta_L$ in terms of $\Delta_y$:
$$
\frac{(1-\gamma) \Delta_y}{K\cdot\Phi\left(\frac{\ln\frac{S}{K}-\frac{1}{2}\sigma^2}{\sigma}\right)}
$$
therefore the amount of $Y$ to tender is:
$$
\Delta_y = y'-y = K\cdot (L+\delta_L) \cdot \Phi\left(\frac{\ln\frac{S'}{K}-\frac{1}{2}\sigma^2}{\sigma}\right)-y\\
=  K\cdot L \cdot \Phi\left(\frac{\ln\frac{S'}{K}-\frac{1}{2}\sigma^2}{\sigma} \right) + (1-\gamma)\Delta_y \cdot \frac{\Phi\left(\frac{\ln\frac{S'}{K}-\frac{1}{2}\sigma^2}{\sigma}\right)}{\Phi\left(\frac{\ln\frac{S}{K}-\frac{1}{2}\sigma^2}{\sigma}\right) }  -y\\
\implies \boxed{\Delta_y = \frac{KL\Phi\left(\frac{\ln\frac{S'}{K}-\frac{1}{2}\sigma^2}{\sigma}\right) - y}{1+(\gamma-1)\frac{\Phi\left(\frac{\ln\frac{S'}{K}-\frac{1}{2}\sigma^2}{\sigma}\right)}{\Phi\left(\frac{\ln\frac{S}{K}-\frac{1}{2}\sigma^2}{\sigma}\right) }}}
$$


### Lowering the price
When we need to lower the price, we need to tender in $X$.
If the current price is $S$ and we want to lower it to $S'$, then we need to tender in $X$ such that we go from $x$ to $x'$ and:
$$
\Delta x = (L + \delta_L)\cdot\left(1-\Phi\left(\frac{\ln\frac{S'}{K}+\frac{1}{2}\sigma^2}{\sigma}\right)\right) - x \\
\implies \boxed{ \Delta_x = \frac{L\left(1-\Phi\left(\frac{\ln\frac{S'}{K}+\frac{1}{2}\sigma^2}{\sigma}\right)\right)-x}{1+(\gamma-1)\frac{1-\Phi\left(\frac{\ln\frac{S'}{K}+\frac{1}{2}\sigma^2}{\sigma}\right)}{1-\Phi\left(\frac{\ln\frac{S}{K}+\frac{1}{2}\sigma^2}{\sigma}\right)}}}
$$

## Value Function on $L(S)$
Relate to value on $V(L,S)$ and $V(x,y)$. 
Then we can use this to tokenize. We have $L_X(x, S)$ and $L_Y(y, S)$.
We know that:
$$
V(x(S),y(S)) = x S + y
$$
Now we also have the following
$$
x = LS\cdot\left(1-\Phi\left(\frac{\ln\frac{S}{K}+\frac{1}{2}\sigma^2}{\sigma}\right)\right)\\
y = K\cdot L\cdot \Phi\left(\frac{\ln\frac{S}{K}-\frac{1}{2}\sigma^2}{\sigma}\right)
$$
Therefore:
$$
\boxed{V(L,S) = L\left( S\cdot\left(1-\Phi\left(\frac{\ln\frac{S}{K}+\frac{1}{2}\sigma^2}{\sigma}\right)\right) + K\cdot \Phi\left(\frac{\ln\frac{S}{K}-\frac{1}{2}\sigma^2}{\sigma}\right)\right)}
$$
Note that $V$ is linear in $L$ and so we can use this to tokenize.

### Time Dependence
Note that $L$ effectively changes as parameters of the trading function change.
To see this, note that the trading function must always satisfy:
$$
\Phi^{-1}\left(\frac{x}{L}\right)+\Phi^{-1}\left(\frac{y}{KL}\right) + \sigma \sqrt{\tau} = 0.
$$
For new parameters, $K'$, $\sigma'$ and $\tau'$, we must find an $L'$ so that the trading function is satisfied:
$$
\Phi^{-1}\left(\frac{x}{L'}\right)+\Phi^{-1}\left(\frac{y}{K'L'}\right) + \sigma' \sqrt{\tau'} = 0.
$$
We can find this new $L'$ using a root finding algorithm. 

#### Root Finding
We will use a bisection algorithm to determine the new $L'$.
Suppose that $(K, \sigma, \tau, L)$ are the current parameters and we have $(K', \sigma', \tau')$ as the new parameters.
Then we can compute:
$$
f(L) = \Phi^{-1}\left(\frac{x}{L}\right)+\Phi^{-1}\left(\frac{y}{K'L}\right) + \sigma' \sqrt{\tau'}.
$$
- If $f(L)<0$, then:
    - Upper bound: $L$
    - Lower bound: $\max \{x, y/K\}$.
- If $f(L)>0$, then:
    - Upper bound: $L \cdot 1.5$.
    - Lower bound: $L$
