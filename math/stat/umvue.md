# 最小方差无偏估计（UMVUE）

:::note[Recall]
If $\delta$ is unbiased est's, $MSE(\delta, \eta(\theta))=Var(\delta(\utilde{X}))$
:::

如果我們限定在無偏的估計方法中找最優的，那麼就是找方差最小的估計方法。也就是 Uniformly Minimum Variance Unbiased Estimator ，最小方差無偏估計。

:::danger
無偏並不一定是最好的。
:::

:::tip[Definition]
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

:::tip[Definition]
$\utilde{X}\stackrel{\text{iid}}{\sim} f(\utilde{x};\theta), \theta\in\Omega\subseteq\R^r$

$\mathscr{F}=\set{f(;\theta);\theta\in\Omega}$ is *complete* $\iff$

$\forall h$ s.t. $E_\theta[h(\utilde{X})]=0\implies P_\theta(h(\utilde{X})=0)=1, \forall \theta$ i.e. $h(\utilde{X})=0$ almost surely.
:::

Now, $\utilde{X}\stackrel{\text{iid}}{\sim} f(\utilde{x};\theta), \theta\in\Omega$, let $T=T(\utilde{X})$ with pdf $f_T(t;\theta)$ and $\mathscr{F}_T=\set{f_T(;\theta);\theta\in\Omega}$

:::tip[Definition]
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

:::tip[Definition]
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


:::tip[Definition]
**the Fisher-info**

$$
I_n(\theta)\xlongequal{also}I_{\utilde{X}}(\theta)\triangleq E_\theta[(\frac{\partial}{\partial\theta}\log f(\utilde{X};\theta))^2]
$$
:::

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

**重新參數化（reparametrization）**

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