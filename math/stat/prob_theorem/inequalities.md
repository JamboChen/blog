# 不等式（Inequalities）

## Holder Inequality

:::tip[Lemma]
Let $a>0, b>0$ and $p>1, q>1$, where $\frac{1}{p}+\frac{1}{q}=1$

$\implies \frac{1}{p}a^p+\frac{1}{q}b^q \ge ab$, where equality holds $\iff a^p=b^q$
:::
$X, Y$ are r.v.'s, take
$$
a=\frac{|X|}{[E(|X|^p)]^{1/p}},\qquad b=\frac{|Y|}{[E(|Y|^q)]^{1/q}} \quad \text{in the lemma}
$$

$$
\begin{align*}
\implies & \frac{1}{p}\left(\frac{|X|}{[E(|X|^p)]^{1/p}}\right)^p+\frac{1}{q}\left(\frac{|Y|}{[E(|Y|^q)]^{1/q}}\right)^q \ge \frac{|X|}{[E(|X|^p)]^{1/p}}\frac{|Y|}{[E(|Y|^q)]^{1/q}} \\
\xRightarrow[\text{both side}]{\text{expectation}}& 1 =\frac{1}{p}+\frac{1}{q}\ge \frac{E|XY|}{[E|X|^p]^{1/p}[E|Y|^q]^{1/q}} \\
\end{align*}
$$

:::tip[Theorem]
**Holder's Inequality**

$p>1, q>1$ where $\frac{1}{p}+\frac{1}{q}=1$

$$
E[|XY|] \le [E|X|^p]^{\frac{1}{p}}[E|Y|^q]^{\frac{1}{q}}
$$

where equality holds $\iff P(\frac{|X|^p}{E[|X|^p]}=\frac{|Y|^q}{E[|Y|^q]})=1$ (almost surely)
:::

$Y=1$ in Holder's Inequality, we get
$$
\begin{align*}
    & E[|X|] \le [E|X|^p]^{\frac{1}{p}} \\
    \xRightarrow{|X|\to |X|^r} & E[|X|^r] \le [E|X|^{pr}]^{\frac{1}{p}} = [E(|X|^s)]^\frac{r}{s},\quad s\ge r\\
    \implies & [E(|X|^r)]^\frac{1}{r}\le [E(|X|^s)]^\frac{1}{s},\quad s\ge r\\
    \implies & g(r) \triangleq [E(|X|^r)]^\frac{1}{r} \text{ is monotonically increasing}
\end{align*}
$$

因此，高次動差存在，可以保證低次動差存在。

:::tip[Theorem]
**Cauchy-Schwarz Inequality**

Let $p=q=2$, in Holder's Inequality, we get
$$
[E(|XY|)]^2\le E[X^2]E[Y^2]
$$

:::

Let $X\to X-E[X], Y\to Y-E[Y]$

$$
\begin{align*}
    |E[(X-E[X])(Y-E[Y])]| &\le E[|(X-E[X])(Y-E[Y])|]\\
    &\le [E(X-E[X])^2]^\frac{1}{2}[E(Y-E[Y])^2]^\frac{1}{2}\\
    &= \sqrt{\sigma^2(X)\sigma^2(Y)}\\
    &= \sigma(X)\sigma(Y)
\end{align*}
$$

i.e. $|Cov(X,y)|\le\sigma(X)\sigma{Y}\iff|\rho_{X,Y}|\le 1$

:::tip[Theorem]
**Minkowski's Inequality**

$$
[E|X+Y|^p]^\frac{1}{p}\le [E|X|^p]^\frac{1}{p}+[E|Y|^p]^\frac{1}{p},\quad p\ge 1
$$
:::

## Jensen's Inequality

:::tip[Theorem]
**Jensen's Inequality**

Any r.v. X, if $g$ is a *convex* function, then $E[g(X)]\ge g(E[X])$

Equality holds $\iff P(g(x)=a+bx)=1$
:::

**Proof**: Let $l(x)$ be the tangent line to the graph of $g(x)$ at the point $(E[X], g(E[X]))$. Note that $E[X]$ is a constant

i.e. $l(x)=a+bx$, s.t. $l(E[X])=a+bE[X]=g(E[X])$ and $l(x)\le g(x), \forall x$, since $g(x)$ is convex.

$$
\begin{align*}
    \because & l(x)\le g(x)\\
    \therefore & g(E[X])=E[l(X)]\le E[g(X)]\\
\end{align*}
$$

## 柴比雪夫不等式（Tchebycheff's Inequality）
$$
\begin{align*}
    \sigma^2 =& E[(X-\mu)^2]\\
    =& E[(X-\mu)^2I(|X-\mu|\ge\varepsilon)]+E[(X-\mu)^2I(|X-\mu|<\varepsilon)]\\
    \ge& E[(X-\mu)^2I(|X-\mu|\ge\varepsilon)]\\
    \ge& \varepsilon^2E[I(|X-\mu|\ge\varepsilon)] \quad \because (X-\mu)^2\ge\varepsilon^2 \text{ when } |X-\mu|\ge\varepsilon\\
    =& \varepsilon^2P(|X-\mu|\ge\varepsilon)\\
    \implies& P(|X-\mu|\ge\varepsilon)\le\frac{\sigma^2}{\varepsilon^2}
\end{align*}
$$
:::tip[Theorem]
**Tchebycheff's Inequality**

Let $X$ be a r.v. with $E[X]=\mu, 0\le\sigma^2=Var(X)$
$$
\implies P(|X-\mu|\ge\varepsilon)\le\frac{\sigma^2}{\varepsilon^2}
$$
:::

Take $\varepsilon=k\sigma>0\implies P(|X-\mu|\ge k\sigma)\le\frac{\sigma^2}{k^2\sigma^2}=\frac{1}{k^2} \iff P(|X-\mu|\le k\sigma)\ge 1- \frac{1}{k^2}$

For $X\stackrel{\text{iid}}{\sim} N(\mu, \sigma^2)$

$$
P(|X-\mu|\le 2\sigma)=P(\frac{|X-\mu|}{\sigma}\le 2)=P(|Z|\le 2)=0.9545\ge 0.75
$$

:::tip[Theorem]
$$
\sigma^2\triangleq Var(X)=0 \implies P(X=\mu)=1
$$
:::

**Proof**: 

$$
\begin{align*}
& \forall \varepsilon>0, P(|X-\mu|\ge\varepsilon)\le\frac{\sigma^2}{\varepsilon^2}=0 \qquad \because \sigma^2=0\\
\iff &\forall \varepsilon>0, P(|X-\mu|<\varepsilon)=1\\
\implies &\forall n=1,2,\cdots, P(|X-\mu|<\frac{1}{n})=1\\
\end{align*}
$$

Let $A_n\triangleq\set{|X-\mu|<\frac{1}{n}}$, then $A_1\supseteq A_2\supseteq\cdots$, and $\lim_{n\to\infty}A_n=\bigcap_{n=1}^\infty A_n=\set{|X-\mu|=0}$

$$
\implies 1=P(\lim_{n\to\infty}A_n)=P(\bigcap_{n=1}^\infty A_n)=P(|X-\mu|=0)=1
$$

---

雖然柴比雪夫不等式給出的上限或下限很寬泛，但它無法被進一步改進，因為有例子可以觸碰到上下限。

Give $k>0$, let $X$ where

$$
\begin{align*}
    & P(X+0)=1-\frac{1}{k^2}\\
    & P(X=1)=\frac{1}{2k^2}\\
    & P(X=-1)=\frac{1}{2k^2}
\end{align*}
$$

$$
\begin{align*}
&\begin{align*}
    \implies \mu=E[X]&=0\cdot P(X=0)+1\cdot P(X=1)+(-1)\cdot P(X=-1)\\
    &=0
\end{align*}\\

&\qquad \begin{align*}
    \sigma^2=&E[X^2]-E[X]^2=E[X^2]\\
    =&0^2\cdot P(X=0)+1^2\cdot P(X=1)+(-1)^2\cdot P(X=-1)\\
    =&\frac{1}{k^2}
\end{align*}\\

&\begin{align*}
    \implies & P(|X-\mu|\ge k\sigma) \\
    &= P(|X|\ge 1) \qquad \because \mu=0, \sigma=\frac{1}{k}\\
    &=P(X=1)+P(X=-1)\\
    &=\frac{1}{k^2}
\end{align*}
\end{align*}
$$

但對於特定的分佈，可以找到更接近的上下限。

e.g. $Z\stackrel{\text{iid}}{\sim} N(0,1)$, for $k>0$

$$
\begin{align*}
    P(|Z|\ge k) &= 2P(Z\ge k)\\
    &=2 \int_k^\infty \frac{1}{\sqrt{2\pi}}e^{-\frac{1}{2}z^2}dz\\
    &=\frac{\sqrt{2}}{\sqrt{\pi}}\int_k^\infty e^{-\frac{1}{2}z^2}dz\\
    &\le \frac{\sqrt{2}}{\sqrt{\pi}}\int_k^\infty \frac{z}{k}e^{-\frac{1}{2}z^2}dz\\
    &=\frac{\sqrt{2}}{\sqrt{\pi}}\frac{1}{k}e^{-\frac{k^2}{2}}
\end{align*}
$$

對於 $P(|Z|\ge 2)$ 來說，用這個不等式計算得到 $P(|Z|\ge 2)\le 0.054$。用柴比雪夫不等式計算得到 $P(|Z|\ge 2)\le 0.25$。而實際上 $P(|Z|\ge 2)=0.0455$。

我們可以推廣到更一般的情況：

:::tip[Lemma]
Let $g$ be a non-negative function, $\forall \varepsilon>0$

$$
P(g(X)\ge\varepsilon)\le\frac{E[g(X)]}{\varepsilon}
$$
:::

**Proof**:
$$
\begin{align*}
    E[g(X)] &= E[g(X)I(g(x)\ge\varepsilon)]+E[g(X)I(g(x)<\varepsilon)]\\
    &\ge \varepsilon E[I(g(x)\ge\varepsilon)]\\
    &\ge \varepsilon E[I(g(x)\ge\varepsilon)] \qquad \because g(X)\ge\varepsilon \text{ when } g(X)\ge\varepsilon\\
    &= \varepsilon P(g(X)\ge\varepsilon)
\end{align*}
$$