# 動差（Moments）

## 期望值和方差

期望值的性質：
1. $E[c]=c$ for any constant $c$.
2. $E[ag(x)+b]=aE[g(X)]+b$
3. $g(x)\ge 0, \forall x \implies E[g(X)]\ge 0$
   
   hence $X\ge Y \implies E[X]\ge E[Y]$
4. $|E[g(X)]|\le E[|g(X)|]$
5. $r>0, E[|g(X)|^r]<\infty \implies E[|g(X)|^s]<\infty, \forall 0<s\le r$
6. $X\perp Y\implies g(X)\perp h(Y)$, and $E[g(X)h(Y)]=E[g(X)]E[h(Y)]$

Let $X$ be a r.v. with $E[X]$ exists, then $\forall c$

$$
\begin{align*}
   E[(X-c)^2] &= E[X-E[X]+E[X]-c]^2\\
   &=E[(X-E[X])^2] + 2E[(X-E[X])(E[X]-c)] + (E[X]-c)^2\\
   &=E[(X-E[X])^2] + (E[X]-c)^2
\end{align*}
$$

i.e. $E[(X-c)^2]\ge E[(X-E[X])^2]$, and the equality holds if and only if $c=E[X]$.

也就是说 $E[X]$ 是 $X$ 的均值（中心），因为每个点到 $E[X]$ 的距离的平方的期望值最小。

:::tip[Definition]
**Variance**

$X$ 每个點  到中心的距离的平方的期望值，即 $X$ 的分散程度，称为 $X$ 的方差。

$$
\sigma^2(X)\xlongequal{also} Var(X) = E[(X-E[X])^2]=E[X^2]-(E[X])^2\ge 0
$$
:::

方差的性質：
1. $Var(c)=0$
2. $Var(ag(X)+b)=a^2Var(g(X))$
3. 
   $$
   \begin{align*}
      Var(X+Y) &= E[(X+Y-E[X]-E[Y])^2]\\
      &= E[(X-E[X])^2] + E[(Y-E[Y])^2] + 2E[(X-E[X])(Y-E[Y])]\\
      &= Var(X) + Var(Y) + 2Cov(X,Y)
   \end{align*}
   $$

   with $Cov(X,Y)\triangleq E[(X-E[X])(Y-E[Y])]=E[XY]-E[X]E[Y]$ is called the covariance（共變數） of $X$ and $Y$.

也就是說，當 $X-E[X]$ 和 $Y-E[Y]$ 同號時， $Cov(X,Y)>0$，異號時 $Cov(X,Y)<0$。

將 $Cov$ 進行歸一化（unit-free）得到

$$
\frac{Cov(X,Y)}{\sqrt{Var(X)Var(Y)}}\triangleq \rho_{X,Y}
$$
 
稱為 $X$ 和 $Y$ 的相關係數（correlation coefficient）。

如果 $X \perp Y \implies E[XY]=E[X]E[Y]\implies Cov(X,Y)=0\implies\rho_{X,Y}=0$ and $Var(X+Y)=Var(X)+Var(Y)$

## 動差（Moments）

:::tip[Definition]
The Moment Generation Function of $X$ is defined as
$$
M_X(t)\triangleq E[e^{tX}],\quad t\in\mathbb{R}
$$
:::

Facts:
1. $M_X(0)=E[E^{0X}]=E[1]=1$
2. If $M_X(t)$ exists $\forall |t|<\delta$ for some $\delta>0$, then $E[X^k]$ exists $\forall k=1,2,\cdots$
   and is given by $E[X^k=M_X^{(k)}]|_{t=0}=M_X^{(t)}(0)$
3. $X\xlongequal{d}Y \iff M_X(t)=M_Y(t)$

