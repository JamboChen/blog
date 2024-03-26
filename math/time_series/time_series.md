# 時間序列

## 分析數據的步驟

當收集到數據 $(X_t)_{t=1,2,\cdot, n}$

1. 畫出數據的分布圖 $(t, X_t)$
2. 嘗試找出週期長度，並去除週期因素
   
   $X^*_t=X_t-S_t$

3. 畫出 $(t, X^*_t)$ 的分布圖
4. 檢查是否有長期趨勢，如果有，去除長期趨勢

   $X^{\#}_t=X^*_t-m_t$

5. $(t, X^{\#}_t)$ 應該要會是穩定的時間序列，即大部分數據點都會在某個區間內波動。

## 分析

:::tip[Definition]
任兩個間隔爲 $k$ 的數據，其協方差是固定的，則稱這個時間序列是*穩定的*，i.e.:

$$
Cov(X^*_t, X^*_t+k)=\gamma(k)\perp t \quad \forall k
$$

$\gamma(k)$ 被稱爲 *ACF(autocovariance function)*

*自我相關係數（ autocorrelation function ）*

$$
\rho_X(k)=\frac{\gamma(k)}{\gamma(0)}=Corr(X^*_t, X^*_{t+k})\in[-1, 1]
$$
:::

電腦在作 ACF 的圖時，同時會給出 95% 的信心區間，如果 $\hat{\gamma}(k)$ 落在這個區間內，則認爲 $\gamma(k)=0$。

如果當 $k>5$ 時，$\hat{\gamma}(k)$ 還沒落在區間內，或者 $\hat{\gamma}(k)$ 有明顯的規律，則認爲時間序列是非穩定的。

當得到數據後，我們會用以下方式來估計 $\gamma(k)$

$$
\hat{\gamma}(k)=\frac{1}{n}\sum_{i=1}^{n-k}[X_t-\bar{X}_n][X_{t+k}-\bar{X}_n]
$$

$$
\implies \hat{\rho}(k)=\frac{\hat{\gamma}(k)}{\hat{\gamma}(0)}
$$

## 模型

### MA(q) 模型（Moving Average）

當第 $t$ 個數據點是由前 $q$ 個白噪音決定時，我們稱這個時間序列是 MA(q) 模型。

:::tip[Definition]
$$
X_t = \sum_{i=0}^{q}\theta_iZ_{t-i} \quad Z_t\sim \text{mean }0, \text{variance } \sigma^2 
$$
:::

因此，根據定義，MA(1) 模型是

$$
X_t = Z_t + \theta Z_{t-1}
$$

並且

$$
\gamma(k)=\begin{cases}
    \sigma^2(1+\theta^2) & k=0\\
    \sigma^2\theta & k=1\\
    0 & k>1
\end{cases}
$$

$$
\rho(k)=\begin{cases}
    1 & k=0\\
    \frac{\theta}{1+\theta^2} & k=\pm 1\\
    0 & k>1
\end{cases}
$$

而對於 MA(q) 模型，$\gamma(k)$ 會在 $k>q$ 時為 0。