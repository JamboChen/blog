# Compare and Contrast

實驗中，我們會用AONVA 表來檢定因素對結果是否是有影響的，這是 Compare。而 Contrast 則是更進一步分析，不同因素之間的主要差異在哪裡。

假設我們有 4 個因素 $A,B,C,D$，我們在獲得數據之前可能有下面幾個問題：
1. Is $A$ different from $C$ ? $\implies H_0:\mu_A=\mu_C$ v.s. $H_1:\mu_A\neq\mu_C$
2. Average of $A$ and $B$ $\overset{?}{=}$ Average of $C$ and $D$ ? 
   
   $\implies H_0:\frac{1}{2}(\mu_A+\mu_B)=\frac{1}{2}(\mu_C+\mu_D)$ v.s. $H_1:\frac{1}{2}(\mu_A+\mu_B)\neq\frac{1}{2}(\mu_C+\mu_D)$
3. $A\overset{?}{=}$ average of $B,C,D$ ? $\implies H_0:\mu_A=\frac{1}{3}(\mu_B+\mu_C+\mu_D)$ v.s. $H_1:\mu_A\neq\frac{1}{3}(\mu_B+\mu_C+\mu_D)$ 

以上這三個問題可以分別寫成和為 0 的線性方程組：

| Contrast | $\mu_A$ | $\mu_B$ | $\mu_C$ | $\mu_D$ |     |
| -------- | ------- | ------- | ------- | ------- | --- |
| $C_1$    | 1       | 0       | -1      | 0       | =0  |
| $C_2$    | 1       | 1       | -1      | -1      | =0  |
| $C_3$    | 3       | -1      | -1      | -1      | =0  |

:::info[Definition]
$k=$ number of trt. A contrast of trt totals:

$$
C_{m}=\sum_{i=1}^k C_{im}Y_{i\cdot}\quad\text{with }\sum_{m=1}^k C_{im}n_i=0
$$
:::

$i=1\cdots,k$, $j=1,\cdots,n_i$

$$
\begin{align*}
    &Y_{ij}=\mu_i+\varepsilon_{ij}\quad\text{with }\varepsilon_{ij}\overset{iid}{\sim}N(0,\sigma_\varepsilon^2)\\
    \implies& Y_{i\cdot}=\sum_{j=1}^{n_i}Y_{ij}\sim N(n_i\mu_i,n_i\sigma_\varepsilon^2)
\end{align*}
$$

$$
\begin{align*}
    \implies &C_m=\sum_{i=1}^k C_{im}Y_{i\cdot}\sim N\left(\sum_{i=1}^k C_{im}n_i\mu_i,\sum_{i=1}^k C_{im}^2n_i\sigma_\varepsilon^2\right)\\
    \implies &\frac{C_m-E(C_m)}{\sqrt{\sum C_{im}^2n_i\sigma_\varepsilon^2}}\sim N(0,1)\quad\text{with }E(C_m)=\sum_{i=1}^k C_{im}n_i\mu_i
\end{align*}
$$

$$
\implies\frac{C_m-E(C_m)}{\sqrt{\sum C_{im}^2n_iMS_E}}\sim t_{N-k}\implies \frac{C_m^2}{\sum C_{im}^2n_iMS_E}\sim F_{1,N-k}
$$

:::info[Definition]
$$
SS_{cm}=\frac{C_m^2}{\sum C_{im}^2n_iMS_E}\overset{H_0}{\sim} F_{1,N-k}\quad\text{with }df=1
$$

$\implies MS_{cm}=SS_{cm}$
:::

**Note**: balance case, $n_i=n,\forall i$

$\sum C_{im}n_i=0\iff \sum C_{im}=0$

$E(C_m)=\sum C_{im}n_i\mu_i=0\iff\sum C_{im}\mu_i=0$

**EX**: 繼續使用上一章 fabric 的數據

| Contrast | $\mu_A$ | $\mu_B$ | $\mu_C$ | $\mu_D$ | $C_m$          | $SS_{cm}$                            |
| -------- | ------- | ------- | ------- | ------- | -------------- | ------------------------------------ |
| 1        | 1       | 0       | 0       | -1      | 8.76-9.26=-0.5 | $\frac{(-0.5)^2}{4(1^2+1^2)}=0.0312$ |
| 2        | 0       | 1       | -1      | 0       | 1.05           | 0.1378                               |
| 3        | 1       | -1      | -1      | 1       |                | 0.3511                               |

注意到，以上三個 Contrast 的係數如果作為向量，那麼是兩兩垂直的。同時 $SS_{c1}+SS_{c2}+SS_{c3}=0.5201=SS_{trt}$ 且自由度為 3。

:::info[Definition]
Two contrasts:
- $C_m=\sum_i^kC_{im}Y_{i\cdot}$ with $\sum_i^kC_{im}n_i=0$
- $C_q=\sum_i^kC_{iq}Y_{i\cdot}$ with $\sum_i^kC_{iq}n_i=0$

are orthogonal $\iff\sum_i^kC_{im}C_{iq}=0$
:::

Since $C_m, C_q$ are normal r.v.

$$
\begin{align*}
    C_m\perp C_q&\iff Cov(C_m,C_q)=0=Cov(\sum C_{im}Y_{i\cdot},\sum C_{iq}Y_{i\cdot})=\sum C_{im}C_{iq}n_i\sigma_\varepsilon^2\\
    &\iff \sum C_{im}C_{iq}n_i=0
\end{align*}
$$

**Remark**: with $df=k-1$, $SS_{trt}=\sum_{m=1}^{df}SS_{cm}$, with $C_m$ are orthogonal and each $SS_{cm}$ has $df=1$

$\implies$ ANOVA table:

|           | df  | SS     | MS     | F     | p-value |
| --------- | --- | ------ | ------ | ----- | ------- |
| **trt**   | 3   | 0.5201 | 0.1734 | 8.53  | 0.0026  |
| $C_1$     | 1   | 0.0312 | 0.0312 | 1.53  | 0.238   |
| $C_2$     | 1   | 0.1378 | 0.1378 | 6.77  | 0.023   |
| $C_3$     | 1   | 0.3511 | 0.3511 | 17.27 | 0.001   |
| **Error** | 12  | 0.2438 | 0.0203 |       |         |

- $A,B,C,D$ 的平均值是顯著不同的
- $B(A\&D)$ 的平均值與 $C(B\&C)$ 的平均值是顯著不同的

:::warning
要檢定的 contrasts 應該要在觀測數據之前就設定好。

“先收集數據，再從數據中找出有意義的 contrasts” 被成為數據嗅探（ data snooping ）。這會導致檢定的實際顯著水准比預期高，因為這種行為會更關注在數據看上去顯著的部分。
:::

## Multiple Comparisons Procedure

將 $C_m$ 寫成與平均相關的形式：

$$
\begin{gather*}
    C_m=\sum_{i=1}^kC_{im}Y_{i\cdot}=\sum_{i=1}^kC_{im}n_i\bar{Y}_i=\sum d_{im}\bar{Y}_i,\quad\text{with }\sum d_{im}=\sum C_{im}n_i=0\\
    E(C_m)=\sum C_{im}n_i\mu_i=\sum d_{im}\mu_i=0
\end{gather*}
$$

$$
\implies H_0: \Gamma_m\triangleq E(C_m)=0\iff\sum d_{im}\mu_i=0, \text{ with } \sum d_{im}=0
$$

$$
C_m\sim N\left(\Gamma_m,\sum d_{im}^2n_i\sigma_\varepsilon^2\right)\implies \frac{C_m-\Gamma_m}{\sqrt{\sum d_{im}^2n_iMS_E}}\sim t_{N-k}
$$

$$
\implies H_0: \Gamma_m=0\text{ v.s. }H_1: \Gamma_m\neq 0
$$

reject $H_0$ at level $\alpha\iff0\notin 1-\alpha$ confidence interval for $\Gamma_m$

$$
\begin{align*}
    1-\alpha&=P\left(\left|\frac{C_m-\Gamma_m}{\sqrt{\sum d_{im}^2n_iMS_E}}\right|>t_{\alpha/2,N-k}\right)\\
    &=P\left(\Gamma_m\in \underbrace{\left[C_m\pm t_{N-k,\alpha/2}\sqrt{\sum d_{im}^2n_iMS_E}\right]}_{CI(\Gamma_m;\alpha)}\right)
\end{align*}
$$

$\forall \Gamma_m$ 以上的 CI 都是 $1-\alpha$ 的信心區間，但 $P(\Gamma_m\in CI(\Gamma_m;\alpha), \forall\Gamma_m)\le 1-\alpha$ 。因此我們希望有一個特殊的 $CI^*$ 來保證可以得到一個 $1-\alpha$ 的信心區間。

### Scheffe 's Method

Compare all contrasts with overall probability of type I error $\le 1-\alpha$

:::info[Definition]
$$
S_{\alpha;cm}=s_{cm}\sqrt{(k-1)F_{k-1,N-k,\alpha}}\quad\text{with }s_{cm}=\sqrt{\sum c_{im}^2n_iMS_E}
$$
:::

Scheffe proves: 

:::tip[Theorem]
$$
\begin{align*}
    1-\alpha&=P\left(\Gamma_m\in\left[C_m\pm s_{cm}\sqrt{(k-1)F_{k-1,N-k,\alpha}}\right], \forall \Gamma_m\right)\\
    &=P\left(\Gamma_m\in\left[C_m\pm S_{\alpha;cm}\right], \forall \Gamma_m\right)\\
\end{align*}
$$
:::

$\implies H_0: \Gamma_m=0 vs H_1:\Gamma_m\neq 0$, reject $H_0$ at level $\alpha\iff |C_m|>S_{\alpha,m}$