---
sidebar_position: 2
---

# 最小方差无偏估计（UMVUE）

:::note[Recall]
If $\delta$ is unbiased est's, $MSE(\delta, \eta(\theta))=Var(\delta(\utilde{X}))$
:::

如果我們限定在無偏的估計方法中找最優的，那麼就是找方差最小的估計方法。也就是 Uniformly Minimum Variance Unbiased Estimator ，最小方差無偏估計。

:::danger
無偏並不一定是最好的。
:::

:::info[Definition]
$\delta^*(\utilde{X})$ is called UMVUE for $\eta(\theta)$ iff
1. $E_\theta\delta^*(\utilde{X})=\eta(\theta), \forall\theta$ i.e. unbiased.
2. $\forall$ unbiased est. $\delta(\utilde{X})$ for $\eta(\theta)$,
   
   $Var_\theta(\delta^*(\utilde{X}))\le Var_\theta(\delta(\utilde{X}))$  i.e. 方差最小。
:::

> Q: 充分統計量在估計上有什麼用？

:::info[Recall]
1. 隨機變量 $T, W$
   $$
    \begin{align*}
        E[E(T|W)] &= E[T]\\
        Var(T) &= E[Var(T|W)]+Var[E(T|W)]\\
        &\ge Var[E(T|W)]
    \end{align*}
    $$
2. If $T=T(\utilde{X})$ is suff for $\theta$ $\iff f_{\utilde{X}|T}(\utilde{x}|t) \perp \theta$
   
   $\implies \forall \delta(\utilde{X}), E[\delta(\utilde{X})|T] = \int \delta(\utilde{x})*f_{\utilde{X}|T}(\utilde{x}|t)d\utilde{x} \perp\theta$
   
   since both $\delta(\utilde{x})$ and $f_{\utilde{X}|T}(\utilde{x}|t)$ are independent of $\theta$.
:::

:::tip[Theorem]
**Rao-Blackwellized**

Suppse $\utilde{X}\stackrel{\text{iid}}{\sim} f(\utilde{x};\theta), \theta\in\Omega$

$S(\utilde{X})$ be **any** est. for $\eta(\theta)$ and $T=T(\utilde{X})$ is suff for $\theta$, with $E_\theta|S(\utilde{X})|\le\infty$

Let $\delta(T)=E[S(\utilde{X})|T]$, then

1. $\delta(T)$ is a statistic with $E_\theta\delta(T)=E_\theta(S(\utilde{X})), \forall\theta$
2. For any function $\eta(\theta)$, $MSE(\delta(T), \eta(\theta))\le MSE(S(\utilde{X}), \eta(\theta)), \forall\theta$
3. If $Var(S(\utilde{X}))\le\infty, \forall\theta$, then 
   
   $MSE(\delta(T), \eta(\theta)) < MSE(S(\utilde{X}), \eta(\theta)), \forall\theta$
:::

**EX**: $X_1,\cdots,X_n\stackrel{\text{iid}}{\sim} B(1, p), p\in\Omega=(0,1)$, let $\eta(p)=p$

$\implies T=\sum_{i=1}^nX_i$ is suff for $p$.

隨便找一個 $\eta(p)$ 的無偏估計 $S(\utilde{X})=X_1$, then $E_p(S(\utilde{X}))=E_p(X_1)=p$ and $Var_p(S(\utilde{X}))=p(1-p)$ 

Let $\delta(T)=E[S(\utilde{X})|T]=E[X_1|T]=\frac{T}{n}=\bar{X}$, and $Var(\bar{X})=\frac{p(1-p)}{n}<p(1-p)$ is better.

:::note
$nE(X_1|T)=\sum E(X_i|T)=E(\sum X_i|T)=E(T|T)=1$
:::

我們可以利用充分統計量，通過 *Rao-Blackwellized* 來改善估計方法，使得偏差不變的情況下，方差更小。

但如果我們有兩個充分統計量 $T_1, T_2$ ，以及任意一個對於 $\theta$ 的估計量 $S(\utilde{X})$
$$
\begin{align*}
    \xRightarrow[Blackwell]{Rao}
    \delta_1(T_1)&=E[S(\utilde{X})|T_1]\\
    \delta_2(T_2)&=E[S(\utilde{X})|T_2]
\end{align*}
$$
$\delta_1(T_1)$ 和 $\delta_2(T_2)$ 都比 $S(\utilde{X})$ 好，但是我們無法比較 $\delta_1(T_1)$ 和 $\delta_2(T_2)$ 誰更好。

為了找到最好的充分統計量，我們需要引入完備（complete）的概念。

:::info[Definition]
$\utilde{X}\stackrel{\text{iid}}{\sim} f(\utilde{x};\theta), \theta\in\Omega\subseteq\R^r$

$\mathscr{F}=\set{f(;\theta);\theta\in\Omega}$ is *complete* $\iff$

$\forall h$ s.t. $E_\theta[h(\utilde{X})]=0\implies P_\theta(h(\utilde{X})=0)=1, \forall \theta$ i.e. $h(\utilde{X})=0$ almost surely.
:::

Now, $\utilde{X}\stackrel{\text{iid}}{\sim} f(\utilde{x};\theta), \theta\in\Omega$, let $T=T(\utilde{X})$ with pdf $f_T(t;\theta)$ and $\mathscr{F}_T=\set{f_T(;\theta);\theta\in\Omega}$

:::info[Definition]
$T$ is *complete*

$\iff \mathscr{F}_T$ is *complete*

$\iff \forall h$ with $E_\theta h(T)=0 \implies h(T)=0$ almost surely $\forall\theta$
:::


:::tip[Theorem]

**Lehmann-Scheffe**

Let $\utilde{X}\stackrel{\text{iid}}{\sim} f(\utilde{x};\theta), \theta\in\Omega\subseteq\R^r$

$T=T(\utilde{X})$ is suff for $\theta$ and is complete
:::

**EX**: $X\stackrel{\text{iid}}{\sim} B(m,p), p\in(0,1), m=1,2,\cdots$

Let $h$ s.t. $E_ph(X)=0, \forall p\in(0,1)$

$\iff \sum_{x=0}^m h(x)f_X(x)=\sum_{x=0}^m h(x)\binom{m}{x}p^x(1-p)^{m-x}=0$, $\forall p\in(0,1)$

Since $p\in(0,1)\implies p^x(1-p)^{m-x}\neq 0$

$\iff h(x)\binom{m}{x}$, $\forall x=0,1,\cdots,m$

$\iff h(x)=0$, $\forall x=0,1,\cdots,m$

i.e. $P_p(h(X)=0)=1$

$\implies B(m,p)$ is complete.

---

**EX**: $X_1,\cdots, X_n \stackrel{\text{iid}}{\sim} B(1,p), p\in(0,1)$

1. $T_0=T_0(\utilde{X})=\utilde{X}$ is **suff** for $p$, but **not complete**.
   
   let $h(T_0)=X_1-X_2$, then $E_p(h(T_0))=0, \forall p\in(0,1)$ but $h(T_0)\neq 0$ almost surely.

2. T_1=X_1 is **not suff** for $p$, but $T_1=X_1\stackrel{\text{iid}}{\sim} B(1,p)$ is **complete** but the previous EX.

3. $T=T(\utilde{X})=\sum_{i=1}^n X_i \stackrel{\text{iid}}{\sim} B(n,p)$ is **suff** for $p$ and is **complete**.

---

Let $T=T(\utilde{X})$ be suff for $\theta$ and $u(\utilde{X})$ be any **unbiased** est.

By *Rao-Blackwellized*, $\delta^*(T)=E[u_0(\utilde{X})|T]$ dominates $u(\utilde{X})$.

For any $u(\utilde{X})$ **unbiased** est, and $\delta(T)=E[u(\utilde{X})|T]$ is also dominates $u(\utilde{X})$.

Let $h(T)=\delta^*(T)-\delta(T)$
$$
\begin{align*}
   E_\theta(\delta^*(T)-\delta(T)) &= E_\theta\delta^*(T)-E_\theta\delta(T)\\
   &=E_\theta u_0(\utilde{X}) - E_\theta u(\utilde{X})\\
   &=\eta(\theta)-\eta(\theta)\\
   &=0
\end{align*}
$$

Since $T$ is complete $\implies h(T)=0$ almost surely $\forall\theta$
$\implies \delta(T)=\delta^*(T)$

i.e. For any unbiased est. $u(\utilde{X})$ is dmoniated by $\delta^*(T)$, $\implies\delta^*(T)$ is the UMVUE for $\eta(\theta)$.

任何無偏估計都可以用充分完備統計量來改善。改善後的結果是唯一的，並且改善得到的估計量一定比原來的更好（或一樣好）。所以我們只需要找到一個充分完備統計量，就可以得到 UMVUE。

:::tip[Theorem]
**Lehmann-Scheffe**

Let $\utilde{X}\stackrel{\text{iid}}{\sim} f(\utilde{x};\theta), \theta\in\Omega\subset \R^r$

$T=T(\utilde{X})$ is suff for $\theta$ and is complete

Let $u(\utilde{X})$ be any unbiased est. for $\eta(\theta)$ with finite variance

and $\delta(T)=E[u(\utilde{X})|T]$, then $\delta(T)$ is the **unique** UMVUE for $\eta(\theta)$
:::

因此，如果我們有一個對於 $\theta$ 充分且完備的統計量，那麼我們可以有兩種方法計算 $\eta(\theta)$ 的UMVUE：
1. 隨便找一個無偏估計，然後通過 *Rao-Blackwellized* 計算 $\delta(T) = E[\delta(\utilde{X})|T]$，則 $\delta(T)$ 是 UMVUE。但計算過程可能很複雜。

2. 根據 $\eta(\theta)$ 猜一個 $T$ 的函數 $g(T)$，計算 $E[g(T)]$，然後根據結果調整 $g(T)$，直到 $E[g(T)]=\eta(\theta)$，則 $g(T)$ 是 UMVUE。

**EX**: $X_1, \cdots, X_n\stackrel{\text{iid}}{\sim} B(1,p),p\in(0,1)$

$\implies T=\sum_{i=1}^nX_i$ is suff for $p$ and is complete.

$\forall p ,E(T)=np\implies E(\frac{T}{n})=p$, so $\frac{T}{n}$ is the UMVUE for $p$.

---

**EX**: $X_1,\cdots,X_n\stackrel{\text{iid}}{\sim} U(0,\theta), \theta>0$

know: $T=X_{(n)}$ is suff for $\theta$

Q: Is $X_{(n)}$ complete?

$\forall h$ s.t. $E_\theta h(X_{(n)})=0, \forall\theta>0$

$$
\begin{align*}
   0&=\int_{-\infty}^\infty h(t)f_T(t) dt\\
   &=\int_{-\infty}^\infty h(t)n(F(t))^{n-1}f(t) dt\\
   &= n \int_{0}^\infty h(t)(\frac{t}{\theta})^{n-1} dt\\
   &= \frac{n}{\theta^n}\int_{0}^\infty h(t)t^{n-1} dt\\
   \iff &\int_{0}^\infty h(t)t^{n-1} dt=0, \forall\theta>0
\end{align*}
$$

假設 $h(t)$ 是連續的，則 $h(t)t^{n-1}$ 也是連續的，那麼 $h(t)t^{n-1}=0, \forall t\in(0,\infty)$

$\implies h(t)=0, \forall t\in(0,\infty)$

$\implies X_{(n)}$ is complete.

Hence, $X_{(n)}$ is suff for $\theta$ and is complete.

1. $\eta(\theta)=\theta=E_\theta(\frac{X_1}{2}),\forall \theta >0$
   1. $\frac{X_1}{2}$:unbaised for $\eta$
      $\implies$ The UMVUE for $\theta$ is $E(\frac{X_1}{2}|T)$, 但其中的計算過程很複雜。

   2. know, $f_T(t)=n\frac{t^{n-1}}{\theta^n}, 0<t<\theta$
      
      $$
      \begin{align*}
         E_\theta[X_{(n)}] &= \int_0^\theta t\cdot n\frac{t^{n-1}}{\theta^n} dt\\
         &=\frac{n}{\theta^n}\int_0^\theta t^n dt\\
         &=\frac{n}{\theta^n}\frac{t^{n+1}}{n+1}\bigg|_0^\theta\\
         &=\frac{n}{\theta^n}\frac{\theta^{n+1}}{n+1}\\
         &=\frac{n}{n+1}\theta
      \end{align*}
      $$

      $\implies E(\frac{X_{(n)}}{n+1})=\theta$, i.e. $\frac{X_{(n)}}{n+1}$ is the UMVUE for $\theta$.

      直觀來說，$X_i,i=1,\dots,n$ 將 $(0,\theta)$ 分成了 $n+1$ 個區間。因為是均勻分佈，所有每個區間的平均長度都會是 $\frac{\theta}{n+1}$，而 $X_{(n)}$ 就是從左到右第 $n$ 個分隔點，所以 $E(X_{(n)})=\frac{n}{n+1}\theta$。

2. $\eta(\theta)=\theta^2$

   Try $E_\theta(X_{(n)}^2)=\frac{n}{\theta^n}\int_0^\theta t^{n+1} dt=\frac{n}{n+2}\theta^2$

   $\implies E(\frac{n+2}{n}X_{(n)}^2)=\theta^2$

:::info[Definition]
**Exp Family**
Let $\utilde{X}\stackrel{\text{iid}}{\sim} f(\utilde{x};\theta), \theta\in\Omega\subseteq\R^r$
Suppose that $\forall \utilde{x},\theta$ 
$$
f(\utilde{x};\theta)=c(\theta)\exp\set{\sum_{i=1}^k Q_i(\theta)T_i(\utilde{x})}h(\utilde{x})
$$
and 
$$
X=\set{\utilde{x}:f(\utilde{x};\theta)>0}\perp\theta, c(\theta)>0, h(\utilde{x})>0
$$
then $f(\utilde{x};\theta\in)$ k-parameter exp family.
:::

指數部分需要有 K 個部分組成，那麼 $f(\utilde{x};\theta)$ 就是一個 K-parameter exp family。

:::tip[Theorem]

Let $\utilde{X}\stackrel{\text{iid}}{\sim} f(\utilde{x};\theta), \theta\in\Omega\subseteq\R^r$ and $f(\utilde{x};\theta)\in$ k-parameter exp family with
$$
B\triangleq\set{(Q_1(\theta),\cdots,Q_k(\theta));\theta\in\Omega}\in\R^k
$$

Suppose $\mathring{B}\neq\empty$ （內點集非空）, i.e. $\dim(B)=k$

then $T(\utilde{X})=(T_1(\utilde{X}),\cdots,T_k(\utilde{X}))$ is suff for $\theta$ and is complete.
:::

**EX**: $X_1,\cdots,X_n\overset{\text{iid}}{\sim}B(1,p), p\in(0,1)$

$$
\begin{align*}
   f(\utilde{x};p)&=p^{\sum x_i}(1-p)^{n-\sum x_i},\quad x_i=0,1\\
   &=(1-p)^n\cdot\left(\frac{p}{1-p}\right)^{\sum x_i}\\
   &=(1-p)^n\exp\left[\sum x_i\ln\left(\frac{p}{1-p}\right)\right]\\
   &=c(p)\exp\left(Q(p)T(\utilde{x})\right)h(\utilde{x})
\end{align*}
$$

and $X=\set{\utilde{x}:f(\utilde{x};p)>0}=\set{x:x_i=0 \text{ or }1, \forall i}\perp p$

$\implies f(\utilde{x};p)\in$ 1-par exp family with

$$
\begin{align*}
   B&=\set{Q(p):p\in\Omega}\subseteq R^1\\
   &=\set{\ln\left(\frac{p}{1-p}\right):p\in(0,1)}\\
   &=(-\infty,\infty)=\R^1\\
   \text{i.e. }& \mathring{B}\neq\empty
\end{align*}
$$

$\implies T(\utilde{X})=\sum X_i$ is suff for $p$ and is complete.

e.g. $\eta(P)=P(1-P)=P(X_1=1,X_2=0)=E_p(I(X_1=1,X_2=0))$

$\implies$ The UMVUE for $P(1-P)$ is

$$
\begin{align*}
   E(I(X_1=1,X_2=0)|T) &= P(X_1=1,X_2=0|T)\\
   &=\frac{P(X_1=1,X_=0.T=t)}{P(T=t)}\\
   &=\frac{P(X_1=1)P(X_2=0)P(\sum_{i=3}^nX_i=t-1)}{P(T=t)}\\
   &=\frac{P(1-P)\binom{n-2}{t-1}P^{t-1}(1-P)^{n-1-t}}{\binom{n}{t}P^t(1-P)^{n-t}}\\
   &=\frac{\frac{(n-2)!}{(t-1)!(n-1-t)!}}{\frac{n!}{t!(n-t)!}}\\
   &=\frac{t(n-t)}{n(n-1)}\\
\end{align*}
$$

---

**EX**: $X_1,\cdots,X_n\overset{\text{iid}}{\sim}P(\lambda)$

$\implies f(\utilde{x};\lambda)=\prod\frac{e^{-\lambda}\lambda^{x_i}}{x_i!}=e^{-n\lambda}\lambda^{\sum x_i}\frac{1}{\prod x_i!}=c(\lambda)\exp\left(\sum x_i \ln\lambda\right)h(\utilde{x})\in$ 1-par exp family and $X=\set{\utilde{x}:x\in \N}\perp \lambda$

with $B=\set{\ln\lambda:\lambda>0}=(-\infty,\infty)\implies \mathring{B}\neq\empty$

$\implies T=\sum X_i$ is suff for $\lambda$ and is complete.

1. $\eta(\lambda)=\lambda$
   
   Note $T\sim P(n\lambda)$ with $E(T)=n\lambda\implies E(\frac{T}{n})=\lambda$ i.e. $\frac{T}{n}$ is the UMVUE for $\lambda$. 
2. $\eta(\lambda)=e^{-2\lambda}\lambda^3=3!P(X_1=3,X_2=0)=E\left(6I(X_1=3,X_2=0) \right)$
   
   $\implies$ The UMVUE for $\lambda^3e^{-2\lambda}$ is $E(6I(X_1=3,X_2=0)|T)=P(X_1=3,X_2=0|T)$

:::tip[Theorem]
**Basu's Theorem**
Let $T=T(\utilde{X})$ be suff for $\theta$ and is complete

Let $V=V(\utilde{X})$ be a statistic whose distribution does not depend on $\theta$

Then $T$ and $V$ are independent.
:::

**EX**: $X_1,\cdots,X_n\stackrel{\text{iid}}{\sim} N(\mu,\sigma^2)$ with $\sigma^2=\sigma^2_0$ known

$\implies$ $\bar{X}$ is suff for $\mu$ and is complete.

note that

$$
\frac{(n-1)S^2}{\sigma_0^2}\stackrel{\text{iid}}{\sim}\chi^2_{n-1}\perp\mu
$$

by Basu's Theorem, $\bar{X}$ and $S^2$ are independent.

---

**EX**: $X_1,\cdots,X_n\overset{\text{iid}}{\sim}N(\mu,\sigma^2)$

$$
\begin{align*}
   f(\utilde{x};\mu,\sigma^2)&=\frac{1}{(2\pi\sigma^2)^{n/2}}\exp\left(-\frac{1}{2\sigma^2}\sum(x_i-\mu)^2\right)\\
   &=\left(\frac{1}{\sqrt{2\pi\sigma^2}}\right)^n\exp\left(-\frac{1}{2\sigma^2}\sum x_i^2 +\frac{\mu}{\sigma^2}\sum x_i-\frac{n\mu^2}{2\sigma^2}\right)\\
\end{align*}
$$

1. $\sigma^2=\sigma_0^2$ known
   
   $$
   f(\utilde{x};\mu)=\left(\frac{1}{\sqrt{2\pi\sigma_0^2}}\right)^ne^{-\frac{n\mu^2}{2\sigma^2}}\exp\left(\frac{\mu}{\sigma^2}\sum x_i\right)e^{-\frac{1}{2\sigma^2}\sum x_i^2}
   $$

   with $X=\R^n\perp\mu\implies f(\utilde{x};\mu)\in$ 1-par exp family and $B=\set{Q(\mu):\mu\in\Omega}=\set{\frac{mu}{\sigma_0^2}:\mu\in\R}=\R$

   $\implies \mathring{B}\neq\empty$

   $\implies T=\sum X_i$ is suff for $\mu$ and is complete $\xRightarrow{1-1} \bar{X}$ is suff for $\mu$ and is complete.

   1. $\eta(\mu)=\mu, \bar{X}\sim N(\mu,\frac{\sigma_0^2}{n})$ with $E(\bar{X})=\mu$ i.e. $\bar{X}$ is the UMVUE for $\mu$.
   2. $\eta(\mu)=\mu^2, \bar{X}\sim N(\mu,\frac{\sigma_0^2}{n})$ with $Var(\bar{X})=\frac{\sigma_0^2}{n}\implies E(\bar{X}^2)=\mu^2+\frac{\sigma_0^2}{n}$

      $\implies \bar{X}^2-\frac{\sigma_0^2}{n}$ is the UMVUE for $\mu^2$.

2. $\mu=\mu_0$: known, $\theta=\sigma^2\in\Omega=(0,\infty)$
   
   $$
   f(\utilde{x};\sigma^2)=\left(\frac{1}{\sqrt{2\pi\sigma^2}}\right)^n\exp\left(-\frac{1}{2\sigma^2}\sum(x_i-\mu_0)^2\right) \in \text{1-par exp family}
   $$

   with $B=\set{-\frac{1}{2\sigma^2}:\sigma^2>0}=(-\infty,0)\subset R^1\implies \mathring{B}\neq\empty$

   $\implies T=T(\utilde{X})=\sum(X_i-\mu_0)^2$ is suff for $\sigma^2$ and is complete.

   Note: $\sum\left(\frac{X_i-\mu_0}{\sigma}\right)^2\sim\chi^2_n$ with $E(\chi^2_n)=n$, i.e. $\frac{T}{\sigma^2}\sim\chi^2_n$

   $\implies E(T)=\sigma^2E(\frac{T}{\sigma^2})=\sigma^2n\implies E(\frac{T}{n})=\sigma^2$

   i.e. $\frac{1}{n}\sum(X_i-\mu_0)^2$ is the UMVUE for $\sigma^2$.

   for $\eta(\sigma^2)=\sigma=\sqrt{\sigma^2}$, try $E(\sqrt{T})=\sigma E\left(\sqrt{\frac{T}{\sigma^2}}\right)=\sigma E(\sqrt{Y})=\sigma C_n$ with $Y\sim\chi^2_n$

   $$
   \begin{align*}
      E(\sqrt{Y})&=\int_0^\infty \sqrt{y}\frac{1}{\Gamma(\frac{n}{2})2^{n/2}}y^{n/2-1}e^{-y/2}dy\\
      &=\frac{\Gamma(\frac{n+1}{2})2^{(n+1)/2}}{\Gamma(\frac{n}{2})2^{n/2}}\underbrace{\int_0^\infty \frac{1}{\Gamma(\frac{n+1}{2})2^{(n+1)/2}}y^{(n+1)/2-1}e^{-y/2}dy}_{=1}\\
      &=\sqrt2\frac{\Gamma(\frac{n+1}{2})}{\Gamma(\frac{n}{2})}\triangleq C_n
   \end{align*}
   $$

   $\implies\frac{\sqrt{T}\Gamma(\frac{n}{2})}{\sqrt{2}\Gamma(\frac{n+1}{2})}$ is the UMVUE for $\sigma$.

3. $\theta=(\mu,\sigma^2)\in\Omega=\R\times(0,\infty)$
   
   $$
   f(\utilde{x};\mu,\sigma^2)=\left(\frac{1}{\sqrt{2\pi\sigma^2}}\right)^ne^{-\frac{n\mu^2}{\sigma^2}} \exp\left(-\frac{1}{2\sigma^2}\sum X_i^2+\frac{\mu}{\sigma^2}\sum X_i\right)\in\text{2-par exp family}
   $$

   with $B=\set{\frac{\mu}{\sigma^2},-\frac{1}{2\sigma^2}}=\R\times(-\infty,0)\implies \mathring{B}\neq\empty$

   $\implies T^*=(\sum X_i,\sum X_i^2)$ is suff for $(\mu,\sigma^2)$ and is complete.

   $\xRightarrow{1-1}T=(\bar{X},S^2)$ is suff for $(\mu,\sigma^2)$ and is complete.

   1. $\eta(\theta)=\mu$, $\delta(T)=\delta(S^2,\bar{X})=\bar{X}$, $E\delta(T)=E\bar{X}=\mu$

      $\implies \bar{X}$ is the UMVUE for $\mu$ (same as when $\sigma^2$ is known)

   2. $\eta(\theta)=\mu^2$
      
      $E(\bar{X}^2)=Var(\bar{X})+(E\bar{X})^2=\frac{\sigma^2}{n}+\mu^2$, $E(S^2)=\frac{\sigma^2}{n-1}E\left(\frac{(n-1)S^2}{\sigma^2}\sim\chi^2_{n-1}\right)=\sigma^2$

      $\implies E(\bar{X}^2-\frac{S^2}{n})=\mu^2$

      $\implies \bar{X}^2-\frac{S^2}{n}$ is the UMVUE for $\mu^2$ (same as when $\sigma^2$ is known)

   3. $\eta(\theta)=\sigma^2$
      $$
      \frac{(n-1)S^2}{\sigma^2}\sim\chi^2_{n-1}\implies E(S^2)=\sigma^2
      $$

      $\implies S^2$ is the UMVUE for $\sigma^2$ ($\neq$ when $\mu$ is known)

   4. $\eta(\theta)=\frac{\mu}{\sigma}$

      $$
      \text{Try }E\left(\frac{\bar{X}}{S}\right)=E(\bar{X})E\left(\frac{1}{S}\right)=\mu E\left(\frac{1}{\sqrt{\frac{(n-1)S^2}{\sigma^2}\sim \chi^2_{n-1}}}\right)\frac{\sqrt{n-1}}{\sigma}=\frac{\mu}{\sigma}\sqrt{n-1}a_n
      $$

      for $Y\sim \chi^2_m$

      $$
      \begin{align*}
         E\left(\frac{1}{Y}\right)&=\int_0^\infty \frac{1}{y}\frac{1}{\Gamma(\frac{m}{2})2^{m/2}}y^{m/2-1}e^{-y/2}dy\\
         &=\frac{\Gamma(\frac{m-1}{2})2^{(m-1)/2}}{\Gamma(\frac{m}{2})2^{m/2}}\underbrace{\int_0^\infty \frac{1}{\Gamma(\frac{m-1}{2})2^{(m-1)/2}}y^{(m-1)/2-1}e^{-y/2}dy}_{=1}\\
         &=\frac{\Gamma(\frac{n-2}{2})}{\Gamma(\frac{n-1}{2})\sqrt2}\triangleq a_n
      \end{align*}
      $$

      $\implies \frac{\bar{X}}{S}\frac{\sqrt{2}\Gamma(\frac{n-1}{2})}{\sqrt{n-1}\Gamma(\frac{n-2}{2})}$ is the UMVUE for $\frac{\mu}{\sigma}$
---

$X\sim f(x;\theta)$, $T=T(\utilde{X})$ is suff for $\theta$ and is complete

如果對給定 $x_0$ 時的 pdf 或 cdf 感興趣，i.e. $\eta(\theta)=f(x_0;\theta)$ or $F(x_0;\theta)$

Note: $F(x_0;\theta)=P(X\le x_0)=E[I(X\le x_0)]$

$\implies$ The UMVUE for $F(x_0;\theta)$ is $E[I(X\le x_0)|T]=P(X\le x_0|T)=\delta(T)$

$\implies$ The UMVUE for $f(x_0;\theta)$ is $\frac{\partial}{\partial x_0}\delta(T)$

## Cramer-Rao Lower Bound

$\forall u(\utilde{X})$ is unbiased for $\eta(\theta)$, 如果我們有一個下界 $LB(\theta)$ 使得 $Var(u(\utilde{X}))\ge LB(\theta),\forall \theta$，並且存在一個無偏估計 $\delta^*(\utilde{X})$ 使得 $Var(\delta^*(\utilde{X}))=LB(\theta)$

$\implies \delta^*(\utilde{X})$ is the UMVUE for $\eta(\theta)$.

Recall that $(cov(Y,W))^2 \le Var(Y)Var(W) \iff Var(W)\ge\frac{(cov(Y,W))^2}{Var(Y)}$

Take $W=u(\utilde{X})$ is unbiased for $\eta(\theta)$ and $Y=\frac{\partial}{\partial\theta}\log f(\utilde{X};\theta)$

Note that
$$
\begin{align*}
   E_\theta(Y)&=E_\theta(\frac{\partial}{\partial\theta}\log f(\utilde{X};\theta))\\
   &=E_\theta(\frac{1}{f(\utilde{X};\theta)}\frac{\partial}{\partial\theta}f(\utilde{X};\theta))\\
   &=\int_{R^n}\frac{\frac{\partial}{\partial\theta}f(\utilde{x};\theta)}{f(\utilde{x};\theta)}f(\utilde{x};\theta)d\utilde{x}\\
   &=\frac{\partial}{\partial\theta}\int_{R^n}f(\utilde{x};\theta)d\utilde{x}\\
   &=\frac{\partial}{\partial\theta}1\\
   &=0
\end{align*}
$$
$$
\begin{align*}
   \implies Cov(u(\utilde{X},Y)) &= E_\theta(u(\utilde{X})Y)-E_\theta(u(\utilde{X}))E_\theta(Y)\\
   &=E_\theta(u(\utilde{X})Y)\\
   &=\int_{R^n}u(\utilde{x})\frac{\frac{\partial}{\partial\theta}f(\utilde{x};\theta)}{f(\utilde{x};\theta)}f(\utilde{x};\theta)d\utilde{x}\\
   &=\frac{\partial}{\partial\theta}\int_{R^n}u(\utilde{x})f(\utilde{x};\theta)d\utilde{x}\\
   &=\frac{\partial}{\partial\theta}E_\theta(u(\utilde{X}))\\
   &=\eta'(\theta) \quad \text{since $u(\utilde{X})$ is unbiased for $\eta(\theta)$}
\end{align*}
$$

$$
\begin{align*}
   Var(Y)&=E(Y^2)-(E(Y))^2\\
   &=E_\theta[(\frac{\partial}{\partial\theta}\log f(\utilde{X};\theta))^2]\\
\end{align*}
$$

:::danger
微分和積分並不總是可以交換順序。以上只能在某些情況下成立。
:::


:::info[Definition]
**the Fisher-info**

$$
I_n(\theta)\xlongequal{also}I_{\utilde{X}}(\theta)\triangleq E_\theta[(\frac{\partial}{\partial\theta}\log f(\utilde{X};\theta))^2]
$$
:::

Fisher Information 描述了隨機變量 $X$ 中包含有關 $\theta$ 的信息量。如果包含的信息量越多，則 $f(x;\theta)$ 對於 $\theta$ 的變化就越敏感，變化量越大。期望值則消除了隨機性。

:::tip[Theorem]
**Cramer-Rao ineq**

Let $\utilde{X}\stackrel{\text{iid}}{\sim} f(\utilde{x};\theta), \theta\in\Omega\subseteq\R$

Assume the following conditions hold:
1. $\Omega$ is open in $\R$
2. $X=\set{\utilde{x}:f(\utilde{x};\theta)>0}\perp\theta$
3. $\frac{\partial}{\partial\theta}\log f(\utilde{x};\theta)$ exists $\forall \utilde{x},\theta$
4. $\forall \delta(\utilde{X})$ with $E_\theta\delta(\utilde{X})<\infty,\forall\theta$
      
   $\frac{\partial}{\partial\theta}\int_{R^n}\delta(\utilde{x})f(\utilde{x};\theta)d\utilde{x}=\int_{R^n}\delta(\utilde{x})\frac{\partial}{\partial\theta}f(\utilde{x};\theta)d\utilde{x}$

5. $I_n(\theta),\forall\theta,n=1,2,\cdots$ and $\frac{\partial}{\partial\theta}\eta(\theta)$ exists
   

then $\forall u(\utilde{X})$ is unbiased for $\eta(\theta)$

$$
Var_\theta(u(\utilde{X}))\ge\frac{(\eta'(\theta))^2}{I_n(\theta)}
$$
:::

Remark：
1. 如果有一個無偏估計 $\delta^*(\utilde{X})$ 使得 $Var(\delta^*(\utilde{X}))=\frac{(\eta'(\theta))^2}{I_n(\theta)}$，則 $\delta^*(\utilde{X})$ 是 UMVUE，並且我們說 CRLB is attainable。

2. 如果所有無偏估計的方差都大於 CRLB，則 CRLB is not attainable。

---

**EX**: $X_1,\cdots,X_n\overset{\text{iid}}{\sim}B(1,P)$

$f(\utilde{x};P)=P^{\sum x_i}(1-P)^{n-\sum x_i}=P^T(1-P)^{n-T}$ with $T=\sum x_i\sim B(n,P)$

$$
\begin{align*}
   \frac{\partial}{\partial P}\log f(\utilde{x};P)&=\frac{\partial}{\partial P}\left(T\log P+(n-T)\log(1-P)\right)\\
   &=\frac{T}{P}-\frac{n-T}{1-P}\\
   &=\frac{T-nP}{P(1-P)}
\end{align*}
$$

$$
\begin{align*}
   I_n(P)&=E[(\frac{\partial}{\partial P}\log f(\utilde{X};P))^2]\\
   &=E\left[\left(\frac{T-nP}{P(1-P)}\right)^2\right]\\
   &=\frac{1}{P^2(1-P)^2}Var(T)\\
   &=\frac{n}{1-P}
\end{align*}
$$

To est $\eta(P)=P$, CRLB $=\frac{(\eta'(P))^2}{I_n(P)}=\frac{P(1-P)}{n}=Var(\bar{X})$

$\implies\bar{X}$ is the UMVUE for $P$ and CRLB is attainable.

---


Note:
1. If $X_i,i=1,\cdots,n$ are independent r.v.s 
   $$
   \begin{align*}
   &f(\utilde{X};\theta)=\Pi_{i=1}^n f_{X_i}(x_i;\theta)\\
   \implies& E[(\frac{\partial}{\partial\theta}\log f(\utilde{X};\theta))^2]= E[(\sum_{i=1}^n\frac{\partial}{\partial\theta}\log f_{X_i}(x_i;\theta))^2]\\
   &=\sum_{i=1}^nE[(\frac{\partial}{\partial\theta}\log f_{X_i}(x_i;\theta))^2]+\sum_{i\neq j}E[(\frac{\partial}{\partial\theta}\log f_{X_i}(x_i;\theta))^2(\frac{\partial}{\partial\theta}\log f_{X_j}(x_j;\theta))]\\
   &=\sum_{i=1}^nE[(\frac{\partial}{\partial\theta}\log f_{X_i}(x_i;\theta))^2]\\
   &=\sum_{i=1}^nI_{X_i}(\theta)
   \end{align*}
   $$

2. If $X_1,\cdots,X_n$ 獨立服從同分佈
   $$
   \begin{align*}
      I_{\utilde{X}}(\theta)&=I_n(\theta)\\
      &=\sum_{i=1}^nI_{X_i}(\theta)\\
      &=nI_X(\theta)
   \end{align*}
   $$

### 重新參數化（reparametrization）

$\theta=g(\xi)$ and $g'$ exists, i.e. $f(\utilde{x};\theta)=f(\utilde{x};g(\xi))$

$$
\begin{align*}
I_{\utilde{X}}(\xi)&\triangleq E[(\frac{\partial}{\partial\xi}\log f(\utilde{X};g(\xi)))^2]\\
&=E[(\frac{\partial\theta}{\partial\xi}\frac{\partial}{\partial\theta}\log f(\utilde{X};\theta))^2]\\
&=(g'(\xi))^2I_n(\theta)
\end{align*}
$$

i.e. $\theta=g(\xi)\implies I_n(\xi)=(g'(\xi))^2I_n(\theta)$

如果我們已經知道 $I_n(\theta)$ ，如果我們又關心 $\xi$ 的 Fisher-info，可以直接利用這個公式換算。

---

**EX** $X_1,\cdots,X_n\overset{\text{iid}}{\sim}N(\mu,\sigma^2)$

$$
\begin{align*}
   I(\mu)&=E[(\frac{\partial}{\partial\mu}\log f(\utilde{X};\mu,\sigma^2))^2]\\
   &=E[(\frac{X-\mu}{\sigma^2})^2]=\frac{1}{\sigma^4}E[(X-\mu)^2]\\
   &=\frac{1}{\sigma^4}Var(X)=\frac{1}{\sigma^2}
\end{align*}
$$

$\implies I_n(\mu)=nI(\mu)=\frac{n}{\sigma^2}$

$$
\text{CRLB for }\mu=\frac{(\frac{\partial}{\partial\mu}\eta)^2}{I_n(\mu)}=\frac{\sigma^2}{n}=Var(\bar{X})
$$

$\implies \bar{X}$: UMVUE for $\mu$ CRLB is attainable.

$$
I(\sigma^2)=I(\theta)=E[(\frac{\partial}{\partial\theta}\log f(\utilde{X};\mu,\theta))^2]
$$

$$
\begin{align*}
   \frac{\partial}{\partial\theta}\log f(\utilde{X};\mu,\theta)&=\frac{\partial}{\partial\theta}\left(\ln\frac{1}{\sqrt{2\pi}}-\frac{1}{2}\ln\theta-\frac{(X-\mu)^2}{2\theta}\right)\\
   &=-\frac{1}{2\theta}+\frac{(X-\mu)^2}{2\theta^2}
\end{align*}
$$

$$
\begin{align*}
   \implies I(\theta)&=E\left[\left(-\frac{1}{2\theta}+\frac{(X-\mu)^2}{2\theta^2}\right)^2\right]\\
   &\xlongequal{\theta=\sigma^2}E\left[\frac{(X-\mu)^4}{4\sigma^8}-\frac{1}{2\sigma^6}E(X-\mu)^2+\frac{1}{4\sigma^4}\right]\\
   &=\frac{1}{4\sigma^4}E\left[\frac{(X-\mu)^4}{\sigma^4}\right]-\frac{1}{2\sigma^4}E\left[\frac{(X-\mu)^2}{\sigma^2}\right]+\frac{1}{4\sigma^4}\\
   &=\frac{1}{4\sigma^4}EY^2-\frac{1}{2\sigma^4}EY+\frac{1}{4\sigma^4} \quad \text{where } Y=(\frac{X-\mu}{\sigma})^2\sim\chi^2_1\\
   &=\frac{3}{4\sigma^4}-\frac{1}{2\sigma^4}+\frac{1}{4\sigma^4}\\
   &=\frac{1}{2\sigma^4}
\end{align*}
$$

$\implies I_{\utilde{X}}(\sigma^2)=nI(\sigma^2)=\frac{n}{2\sigma^4}$

$$
\text{CRLB for }\sigma^2=\frac{(\frac{\partial}{\partial\sigma^2}\eta)^2}{I_n(\sigma^2)}=\frac{1}{\frac{n}{2\sigma^4}}=\frac{2\sigma^4}{n}
$$

1. $\mu=\mu_0$ known, the UMVUE for $\sigma^2$ is $\frac{1}{n}\sum(X_i-\mu_0)^2$

   $$
   Var(\frac{1}{n}\sum(X_i-\mu_0)^2)=\frac{\sigma^4}{n^2}\sum Var\left(\frac{X_i-\mu_0}{\sigma^2} \right)^2=\frac{2\sigma^4}{n}=\text{CRLB}
   $$

2. $\mu$ unknown, the UMVUE for $\sigma^2$ is $S^2$

   Note that $\frac{(n-1)S^2}{\sigma}\sim\chi^2_{n-1}$

   $$
   Var(S^2)=\frac{\sigma^4}{(n-1)^2}Var\left(\frac{(n-1)S^2}{\sigma^2}\right)^2=\frac{2\sigma^4}{n-1}>\frac{2\sigma^4}{n}=\text{CRLB}
   $$

   CRLB is not attainable.

3. To est $I(\sigma)$, let $\xi=\sigma\implies\sigma^2=\xi^2=g(\xi)$

   $\implies I(\xi)=(g'(\xi))^2I(\sigma^2)=4\xi^2\frac{1}{2\sigma^4}=\frac{2}{\sigma^2}$

   $\implies I_n(\sigma)=\frac{2n}{\sigma^2}$
---

**EX**: 
1. $X\stackrel{\text{iid}}{\sim} P(\lambda)$

$$
\frac{\partial}{\partial\lambda}\log f(x;\lambda)=\frac{\partial}{\partial\lambda}\log\frac{e^\lambda\lambda^x}{x!}=\frac{\partial}{\partial\lambda}(-\lambda+x\log\lambda-\log(x!))=\frac{x}{\lambda}-1=\frac{x-\lambda}{\lambda}
$$

$$
\begin{align*}
   \implies I(\lambda)&=E[(\frac{\partial}{\partial\lambda}\log f(x;\lambda))^2]\\
   &=E[(\frac{x-\lambda}{\lambda})^2]\\
   &=\frac{1}{\lambda^2}E[(x-\lambda)^2]=\frac{1}{\lambda^2}Var(X)\\
   &=\frac{1}{\lambda}
\end{align*}
$$

2. Show that the Fisher-info $\utilde{X}=(X_1,\cdots,X_n)$ from $P(\lambda)$ contains about $\sqrt\lambda$ is independent of $\lambda$.

   know $I_n(\lambda)=nI_X(\lambda)=\frac{n}{\lambda}$

   $\xi=\sqrt\lambda\implies\lambda=\xi^2=g(\xi)$
   $\implies I_n(\xi)=(g'(\xi))^2I_n(\lambda)=(2\xi)^2\frac{n}{\lambda}=4(\sqrt{\lambda})^2\frac{n}{\lambda}=4n$

---

有的時候，我們關心的 $\eta(\theta)$ 不存在無偏估計，e.g. $B(1,p),\eta(p)=\frac{1}{p}$。又或者存在無偏估計，但 UMVUE 超出了參數的範圍。

**EX**(#12.3.11): $f(x;\theta)=\theta(1-\theta)^x, x=0,1,\cdots,\theta\in\Omega=(0,1)$

$$
U(X)=
\begin{cases}
   1 & if X=0\\
   0 & if X\neq 0
\end{cases}
$$

則 $U(X)$ 是 UMVUE，但 $U(X)\notin\Omega$