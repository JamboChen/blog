# 一致性（Consistency）

當我們獲得的數據量越來越多時，一個好的估計方法應該要能更準確的估計出真實的數據分布。這種性質稱為一致性。

:::tip[Definition]
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