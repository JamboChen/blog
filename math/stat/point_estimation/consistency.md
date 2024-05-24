---
sidebar_position: 6
---

# 一致性（Consistency）

當我們獲得的數據量越來越多時，一個好的估計方法應該要能更準確的估計出真實的數據分布。這種性質稱為一致性。

:::info[Definition]
$\forall \varepsilon >0$ ，we say $\delta_n \xrightarrow[n\to\infty]{P} \eta(\theta)$ if

$$
P_\theta(|\delta_n-\eta(\theta)|\le \varepsilon) \xrightarrow[n\to\infty]{} 1
$$

也就是 $\delta_n$ 以幾率收斂到 $\eta(\theta)$ 。
:::

:::tip[Theorem]
讓 $\delta_n$ 是 $\eta(\theta)$ 的一致估計，則對於任何符合以下要求的數列 $a_n, b_n$：
$$
\lim_{n\to\infty} a_n = 1 \quad\lim_{n\to\infty} b_n = 0
$$

$\implies\delta^*_n=a_n\delta_n+b_n$ 也是 $\eta(\theta)$ 的一致估計。
:::

點估計中，有很多一致性估計。所以我們應該要捨棄那些不一致的估計方法。

**Note**: 

$\forall\varepsilon>0$

$$
P_\theta(|\delta_n-\eta(\theta)|>\varepsilon) = P_\theta(|\delta_n-\eta(\theta)|^2>\varepsilon^2) \le \frac{E_\theta(\delta_n-\eta(\theta))^2}{\varepsilon^2} \quad\text{Chebyshev's inequality}
$$

$$
\implies E_\theta(\delta_n-\eta(\theta))^2 \xrightarrow[n\to\infty]{} 0
$$

Note that $\delta_n=Var(\delta_n)+Bias^2(\delta_n)$

i.e. If $E_\theta(\delta_n)\to\eta(\theta)$ and $Var(\delta_n)\to 0$ then $\delta_n$ is a consistent estimator of $\eta(\theta)$.

i.e. 方差會隨樣本數增加而趨近於 0 的無偏估計方法是一致的。

---

**EX**: $X_1, \cdots, X_n \overset{\text{iid}}{\sim}E(X_i)=\theta$.

let $\delta^*$ be the UMVUE of $\theta$, i.e. $Var(\delta^*)\le Var(\delta)$ for all unbiased estimator $\delta$.

$\implies \delta^*$ is consistent.

E.g. $\delta=\bar{X}\implies Var(\bar{X})=\frac{\sigma^2}{n}\to 0$

## 大數法則

:::info[Theorem]
1. $X_1,\cdots,X_n$ are iid with $E(X_i)=\mu$ $\implies \bar{X}\xrightarrow{P}\mu$, i.e. $\bar{X}$ is consistent.
2. $X_n\xrightarrow{P}c\implies g(Y_n)\xrightarrow{P}g(c), \forall g$ continuous. e.g. $\bar{X}^2\xrightarrow{P}\mu^2$
3. $X_n\xrightarrow{P}c, Y_n\xrightarrow{P}d\implies X_n+Y_n\xrightarrow{P}c+d$
:::

**EX**: $X_1,\cdots,X_n\overset{\text{iid}}{\sim}E(X_i)=\mu, Var(X_i)=\sigma^2<\infty$

$\implies X_1^2+\cdots+X_n^2\overset{\text{iid}}{\sim}E(X_i^2)=Var(X_i)+E(X_i)^2=\sigma^2+\mu^2$

By LLN, $\bar{X^2}=\frac{1}{n}\sum_{i=1}^nX_i^2\xrightarrow{P}\sigma^2+\mu^2$ and $\bar{X}\xrightarrow{P}\mu\implies \bar{X}^2\xrightarrow{P}\mu^2$

$$
\implies S_*^2=\frac{1}{n}\sum_{i=1}^n(X_i-\bar{X})^2=\frac{1}{n}\sum_{i=1}^nX_i^2-\bar{X}^2\xrightarrow{P}\sigma^2
$$

$$
\because \frac{n}{n-1}\xrightarrow[n\to\infty]{}1\implies S^2=\frac{n}{n-1}S_*^2\xrightarrow{P}\sigma^2
$$

以上並沒有假設 $X_i$ 的分佈，只要 $E(X_i)=\mu, Var(X_i)=\sigma^2<\infty$ 即可。