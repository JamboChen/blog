---
sidebar_position: 1
---

# 統計推論（Statistical Inferences）

## 統計量（Statistics）

:::info[Definition]
$T:\R^n\to \R^m$ with $T(\utilde{X})=(T_1(\utilde{X}),T_2(\utilde{X})\ldots,T_m(\utilde{X}))$ is called **statistics** of $\utilde{X}=(X_1,X_2,\ldots,X_n)$.

Usually, $m\le n$.
:::

- eg. 觀察到樣本 $X_1\ldots X_{10}$，定義統計量 $T(\utilde{X})=(\frac{1}{n}\sum_i^{10}X_i,\frac{1}{n}\sum_i^{10}X_i^2)$，i.e. $T: \R^{10}\to\R^2$

## 充分統計量（Sufficient Statistics）

-  Idea: 認為樣本 $\utilde{X}$ 包含了所有關於參數 $\theta$ 的信息。我們希望使用的統計量能夠保留所有關於參數 $\theta$ 的信息。

:::info[Definition]
A stat $T=T(\utilde{X})$ is sufficient for $\theta$ (or sufficient for $\mathscr{F}$, where $\mathscr{F}=\set{f(:\theta):\theta\in\Omega}$)

**iff** the conditional distribution of $\utilde{X}$ given $T(\utilde{X})$ does not depend on $\theta$.

i.e. $P_\theta(\utilde{X}\le\utilde{x}|T=t)\perp\theta$
:::

與 $\theta$ 相關的信息都通過 $T$ 提供了，所以 $P_\theta(\utilde{X}\le\utilde{x}|T=t)$ 不會隨著 $\theta$ 變化。

:::note[Remark]
我們很難通過定義找到充分統計量
1. Guess a $T=T(\utilde{X})$ （直覺）
2. Compute $f_{\utilde{X}|T}(\utilde{x}|t)$ （可能涉及複雜計算）
:::

:::tip[Theorem]
分解定理（Factorization Theorem）

Let $\utilde{X}=(X_1,\ldots,X_n)$ be a random sample with pdf $f(\utilde{x};\theta),\theta\in\Omega=\R^r$.

A stat $T=T(\utilde{X})=(T_1(\utilde{X}),\ldots,T_m(\utilde{X}))$ is sufficient for $\theta$ $\iff$ $\exists$ functions $g(t;\theta)\ge 0$ and $h(\utilde{x})$ such that $f(\utilde{x};\theta)=g(t;\theta)h(\utilde{x})$.
:::

$\forall \utilde{x}\in\R^n, \theta\in\Omega, t=T(\utilde{x})$

$f(\utilde{x};\theta)=g(t;\theta)h(\utilde{x})$ $\iff$ $T=T(\utilde{X})$ is sufficient for $\theta$.

$\implies$ if $T=H(u)$ with $u=u(\utilde{X})$, then 
$$
\begin{align*}
    f(\utilde{x};\theta)&=g(t;\theta)h(\utilde{x})\\
    &=g^*(u;\theta)h(\utilde{x})
\end{align*}
$$
$\implies u=u(\utilde{X})$ is also sufficient for $\theta$.

and if $T^*=T^*(\utilde{X})\xleftrightarrow{1-1} T(\utilde{X})$, then $f(\utilde{x};\theta)=g^*(t^*;\theta)h(\utilde{x})$.

$\implies T^*$ is also sufficient for $\theta$.

:::tip[Corollary]
1. $T$ is sufficient for $\theta$ and $T^*=H(T)$ with $H$ is 1-1 $\implies$ $T^*$ is also sufficient for $\theta$.
2. $T$ is sufficient for $\theta$, $u$ is statistic and $T=H(u)$ with some function $H$ $\implies$ $u$ is also sufficient for $\theta$.
:::

**EX**: $X_1,\ldots,X_n\stackrel{\text{iid}}{\sim} B(1,p)$ independent, $p\in\Omega=(0,1)$

$$
f(\utilde{x};p)=p^{\sum_i^n x_i}(1-p)^{n-\sum_i^n x_i}\triangleq g(t;p)h(\utilde{x}), \forall p\in(0,1)
$$
with $t=\sum_i^n x_i$.

$\implies T=T(\utilde{X})=\sum_{i=1}^n X_i$ is sufficient for $p$.

$\implies \bar{X}=\frac{1}{n}\sum_{i=1}^n X_i$ is also sufficient for $p$.

$\implies e^{\bar{X}}$ is also sufficient for $p$.

**or** 

$$
f(\utilde{x};p)=p^{x_1+\sum_{i=2}^n x_i}(1-p)^{n-(x_1+\sum_{i=2}^n x_i)}\triangleq g(t;p)h(\utilde{x}), \forall p\in(0,1)
$$
with $t=(t_1,t_2)=(x_1,\sum_{i=2}^n x_i), g(t;p)=g(t_1,t_2;p)=p^{t_1+t_2}(1-p)^{n-(t_1+t_2)}$

$\implies T=(T_1, T_2)=(X_1,\sum_{i=2}^n X_i)$ is sufficient for $p$.

---

**EX**: $X_1,\ldots,X_n\stackrel{\text{iid}}{\sim} P(\lambda)$ independent, $\lambda\in\Omega=(0,\infty)$
$$
f(\utilde{x};\lambda)=\frac{e^{-n\lambda}\lambda^{\sum_i^n x_i}}{\Pi_i^n x_i!}\triangleq g(t;\lambda)h(\utilde{x}), \forall \lambda\in(0,\infty)
$$

with $t=\sum_i^n x_i$ is suff for $\lambda$, $\implies \bar{X}$ is also suff for $\lambda$.

---

**EX**: $X_1,\ldots,X_n\stackrel{\text{iid}}{\sim} N(\mu,\sigma^2)$ independent, $(\mu,\sigma)\in\Omega=\R\times(0,\infty)$
$$
f(\utilde{x};\mu,\sigma^2)=(\frac{1}{\sqrt{2\pi}\sigma})^ne^{-\frac{1}{2\sigma^2}\sum_i^n(x_i-\mu)^2}
$$

- $\mu=\mu_0$ known, $\theta=\sigma^2$ unknown
  $$ 
    f(\utilde{x};\sigma^2)=(\frac{1}{\sqrt{2\pi}\sigma})^ne^{-\frac{1}{2\sigma^2}\sum_i^n(x_i-\mu_0)^2}*1\triangleq g(t;\sigma^2)h(\utilde{x})
  $$
  with $t=\sum_i^n(x_i-\mu_0)^2$ is suff for $\sigma^2$.

- $\sigma^2=\sigma_0$ known, $\theta=\mu$ unknown
  $$
  \begin{align*}
    f(\utilde{x};\mu)=(\frac{1}{\sqrt{2\pi}\sigma_0})^ne^{-\frac{1}{2\sigma_0^2}\sum_i^n(x_i-\mu)^2}&=\exp(-\frac{1}{2\sigma_0^2}(-2\mu\sum_i^n x_i+n\mu^2))(\frac{1}{\sqrt{2\pi}\sigma_0^2})^n\\&\triangleq g(t;\mu)h(\utilde{x})
  \end{align*}
    
  $$
  with $t=\sum_{i=1}^nx_i$ is suff for $\mu$ and $\bar{X}$ is also suff for $\mu$.

- $\theta=(\mu,\sigma^2)$ unknown
  $$
  \begin{align*}
    f(\utilde{x};\mu,\sigma^2)&=(\frac{1}{\sqrt{2\pi}\sigma})^ne^{-\frac{1}{2\sigma^2}\sum_i^n(x_i-\mu)^2}\\
    &=(\frac{1}{\sigma})^n\exp(-\frac{n-1}{2\sigma^2}S^2-\frac{n}{2\sigma^2}(\bar{X}-\mu)^2)(\frac{1}{\sqrt{2\pi}})^n\\
    &\triangleq g(t;\mu,\sigma^2)h(\utilde{x})
  \end{align*}
  $$
  with $t=(t_1, t_2)=(\bar{X}, S^2)$, $\implies T=(\bar{X},S^2)$ is suff for $\theta=(\mu, \sigma^2)$.

:::note
$S^2=\frac{1}{n-1}\sum_{i=1}^n (X_i-\bar{X})^2=\frac{1}{n-1}(\sum_{i=1}^n X_i^2-n\bar{X}^2)$ and $\bar{X}=\frac{1}{n}\sum_{i=1}^n X_i$

$\implies(\bar{X}, S^2)\xleftrightarrow{1-1}(\bar{X}, \sum_{i=1}^nX_i^2)\xleftrightarrow{1-1}(\sum X_i, \sum X_i^2)$
:::

**EX**: $X_1,\ldots,X_n\stackrel{\text{iid}}{\sim} U(0,\theta), \theta>0$
$$
\begin{align*}
    f(\utilde{x};\theta)&=\Pi_{i=1}^n\frac{1}{\theta}I_{(0,\theta)}(x_i)\\
    &=\frac{1}{\theta^n}I(x_{(n)}\le \theta)I(x_{(1)}\ge 0)\\
    &\triangleq g(t;\theta)h(\utilde{x})
\end{align*}
$$
with $t=x_{(n)}$ $\implies T=X_{(n)}$ is suff for $\theta$.

---

**EX**: $X_1,\ldots,X_n\stackrel{\text{iid}}{\sim} U(\alpha,\beta), \alpha<\beta$

- $\theta=\alpha$ unkonwn, $\beta$ known

  $\implies X_{(1)}$ is suff for $\alpha$.

- $\theta=\beta$
  
  $\implies X_{(n)}$ is suff for $\beta$.

- $\theta=(\alpha,\beta)$ unknown
  
  $\implies (X_{(1)}, X_{(n)})$ is suff for $\theta$.

## 最小充分統計量（Minimal Sufficient Statistics）

:::info[Definition]
A sufficient stat $T^*$ is called **minimal sufficient** for $\theta$ 

**iff** $\forall$ suff stat $T$ for $\theta$, $\exists$ function $h$ such that $T^*=h(T)$.

:::

最小充分統計量可以從其他任何充分統計量歸納得到，並且無法進一步歸納（縮減信息）。即信息量在足夠充分的情況下是最小的。

:::danger
最小統計量不是指統計量的維度最小，而是信息量最小。
:::

:::tip[Theorem]
Let $\utilde{X}\stackrel{\text{iid}}{\sim} f(\utilde{x};\theta)$, $\theta\in\Omega$, suppose $\exist a$ stat $T=T(\utilde{X})$ s.t. 
$\frac{f(\utilde{x};\theta)}{f(\utilde{y};\theta)}\perp\theta\iff T(\utilde{x})=T(\utilde{y})$

then $T=T(\utilde{X})$ is minimal sufficient for $\theta$.
$$
:::