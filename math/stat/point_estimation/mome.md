---
sidebar_position: 4
---

# 動差估計（Method Of Moments Estimation, MOME）

得到樣本 $X_1,\cdots,X_n\stackrel{\text{iid}}{\sim} X$ with $X\stackrel{\text{iid}}{\sim} f(x;\theta), \theta\in\Omega\subseteq\R^r$

我們定義 k 次動差：
- 母體（Population）：$E_\theta(X^k)=m_k(\theta)$
- 樣本（Sample）: $\frac{1}{n}\sum_{i=1}^nX_i^k=m_k(\utilde{X})$

根據大數法則，隨著樣本數量增加，樣本動差會收斂到母體動差，i.e. $m_k(\utilde{X})\xrightarrow{P}m_k(\theta)$ as $n\to\infty$.

如果一個分佈有若干參數，並且這個分佈的動差可以用這些參數表示，那我們可以令 $m_k(\theta)=m_k(\utilde{X}),k=1,2,\cdots$，然後解方程組得到這些參數的估計量。

如果我們得到了 $\theta$ 的 MOME $\hat{\theta}$，那麼對於 $\eta(\theta)$，我們可以直接帶入 $\hat{\theta}$，i.e. $\widehat{\eta(\theta)}=\eta(\hat{\theta})$

**EX**: $X_1, \cdots,X_n\stackrel{\text{iid}}{\sim} E(X)=\mu, Var(X)=\sigma^2<\infty$

set

$$
\begin{cases}
    \mu=E(X)=m_1(\theta)=m_1(\utilde{X})=\frac{1}{n}\sum_{i=1}^nX_i=\bar{X}\\
    \mu^2+\sigma^2=E(X^2)=m_2(\theta)=m_2(\utilde{X})=\frac{1}{n}\sum_{i=1}^nX_i^2
\end{cases}
$$

$$
\implies
\begin{cases}
    \mu=\bar{X}\\
    \sigma^2=\frac{1}{n}\sum_{i=1}^nX_i^2-\bar{X}^2=\frac{1}{n}\sum_{i=1}^n(X_i-\bar{X})^2
\end{cases}
$$
$$
\implies
\begin{cases}
    \hat{\mu}_{MOME}=\bar{X}\\
    \hat{\sigma}^2_{MOME}=\frac{1}{n}\sum_{i=1}^n(X_i-\bar{X})^2=S^2_*
\end{cases}
$$

對於兩個參數的分佈，基本上都可以用以上的方式得到參數的 MOME。

:::danger
$$
\frac{1}{n}\sum_{i=1}^n(X_i-\bar{X})^2=S^2_*\neq S^2=\frac{1}{n-1}\sum_{i=1}^n(X_i-\bar{X})^2
$$
$S^2$ 是方差的無偏估計量，$S^2_*$ 是方差的有偏估計量。
:::

**EX**: $X_1,\cdots,X_m\stackrel{\text{iid}}{\sim} U(\alpha, \beta), \theta=(\alpha, \beta)$

$$
\begin{cases}
    \frac{\alpha+\beta}{2}=\bar{X}\\
    \frac{(\alpha-\beta)^2}{12}=S^2_*
\end{cases}
$$

$$
\implies
\begin{cases}
    \hat{\alpha}_{MOME}=\bar{X}-\sqrt{3}S_*\\
    \hat{\beta}_{MOME}=\bar{X}+\sqrt{3}S_*
\end{cases}
$$

**Recall**: $T=(X_{(1)},X_{(n)})$ is minimal sufficient for $\alpha, \beta$， and $\hat{\alpha}_{MOME},\hat{\beta}_{MOME}$ are not functions of $T$. Hence, they can be improved by Rao-Blackwell Theorem.