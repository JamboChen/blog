# Compare and Contrast

實驗中我們會用 AONVA 表來檢定因素對結果是否是有影響的，這是 Compare。而 Contrast 則是更進一步分析，不同因素之間的主要差異在哪裡。

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
S_{\alpha;cm}=s_{cm}\sqrt{(k-1)F_{k-1,N-k,\alpha}}\quad\text{with }s_{cm}=\sqrt{\sum \frac{c_{im}^2}{n_i}MS_E}
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

## Comparing Pairs of Treatment Means

### Tukey's Method

用於比較兩個 trt 的平均值是否有顯著差異，並且保證所有的成對比較的總類型 I 錯誤率不超過 $\alpha$

設有 $k$ 個 trt ，它們的平均分別為 $\mu_1,\cdots,\mu_k$

$\forall i\neq i'$ 用 $\bar{Y}_{i\cdot}-\bar{Y}_{i'\cdot}$ 來估計 $\mu_i-\mu_{i'}$

$$
\implies \bar{Y}_{i\cdot}-\bar{Y}_{i'\cdot}\sim N\left(\mu_i-\mu_{i'},\frac{\sigma_\varepsilon^2}{n_i}+\frac{\sigma_\varepsilon^2}{n_i'}\right)

\implies \frac{\bar{Y}_{i\cdot}-\bar{Y}_{i'\cdot}}{\sqrt{MS_E\left(\frac{1}{n_i}+\frac{1}{n_i'}\right)}}\sim t_{N-k}
$$

$$
\implies 1-\alpha=P\left(\mu_i-\mu_{i'}\in\underbrace{\left[\bar{Y}_{i\cdot}-\bar{Y}_{i'\cdot}\pm t_{N-k,\alpha/2}\sqrt{MS_E\left(\frac{1}{n_i}+\frac{1}{n_i'}\right)}\right]}_{CI(\mu_i-\mu_{i'};\alpha)}\right)
$$

但所有信賴區間都成功的幾率會小於 $1-\alpha$ ，因此我們希望能找到一個區間 $CI^*$ s.t. $P(\mu_i-\mu_{i'}\in CI^*,\forall i\neq i')\ge 1-\alpha$

:::info[Definition]
$$
T_\alpha=\frac{q_\alpha(k,f)}{\sqrt{2}}\sqrt{(\frac{1}{n_i}+\frac{1}{n_i'})MS_E}\xlongequal{\text{bal}}q_\alpha(k,f)\sqrt{\frac{MS_E}{n}}
$$

- $k=$ number of trt
- $f=$ df of error
:::

:::tip[Theorem]
Tukey's Result:

$$
P\left(\mu_i-\mu_{i'}\in\left[\bar{Y}_{i\cdot}-\bar{Y}_{i'\cdot}\pm T_\alpha\right],\forall i\neq i'\right)\ge 1-\alpha
$$
:::

i.e. $\forall i\neq i'$ with $H_0:\mu=\mu'$ vs $H_1:\mu\neq\mu'$, reject $H_0$ $\iff |\bar{Y}_{i\cdot}-\bar{Y}_{i'\cdot}|>T_\alpha$ with overall sig. level $\le \alpha$

### Fisher Least Significant Difference (LSD) Method

The Fisher Least Significant Difference (LSD) Method. P99-101

### Student-Newman-Keuls (SNK) Method

檢驗一對 trt 的平均值中，數值較大的 trt 是否顯著大於數值較小的 trt。

$$
H_0:\mu_i=\mu_j\quad\text{v.s.}\quad H_1:\mu_i>\mu_j\quad\text{with }\bar{Y}_{i\cdot}>\bar{Y}_{j\cdot}
$$

使用 fabric 的數據在 $\alpha=0.05$ 下進行 SNK 檢定：

1. 將所有的 trt 平均從小到大排序
   | fabric      | A    | D    | C    | B    |
   | ----------- | ---- | ---- | ---- | ---- |
   | sample mean | 2.19 | 2.32 | 2.42 | 2.68 |

   將所有的 trt 進行兩兩比較，並且計算它們的差值
   |     | A    | D    | C    | B   |
   | --- | ---- | ---- | ---- | --- |
   | A   |      |      |      |     |
   | D   | 0.13 |      |      |     |
   | C   | 0.23 | 0.10 |      |     |
   | B   | 0.49 | 0.36 | 0.26 |     |


2. 從 ANVOA 表中得到數據 $MS_E=0.0203$ 和 $df=12$，並計算要比較的兩個 trt 的方差：
   
   $$
   S_{AB}=\sqrt{\frac{MS_E}{2}(\frac{1}{n_i}+\frac{1}{n_j})}\xlongequal{\text{bal}}\sqrt{\frac{MS_E}{n}}
   $$

   在這組數據下 $S_{\bar{Y}_{i\cdot}}=\sqrt{\frac{0.0203}{4}}=0.0712$

3. 通過查表得到 $q_\alpha(p, df)$ ，其中 $p=2,\cdots,k$ 代表要比較的兩個 trt 在排序中的差距。
   
   | $q_{0.05}(2,12)$ | $q_{0.05}(3,12)$ | $q_{0.05}(4,12)$ |
   | ---------------- | ---------------- | ---------------- |
   | 3.05             | 3.77             | 4.20             |

   將 $q_\alpha(p, df)$ 與 $S_{AB}$ 相乘得到 $SNK(p,0.05)$
   |     | A    | D    | C    | B   |
   | --- | ---- | ---- | ---- | --- |
   | A   |      |      |      |     |
   | D   | 0.22 |      |      |     |
   | C   | 0.27 | 0.22 |      |     |
   | B   | 0.30 | 0.27 | 0.22 |     |
4. 將所有的差值與 $SNK(p,0.05)$ 進行比較，如果差值大於 $SNK(p,0.05)$ 則拒絕 $H_0:\mu_i=\mu_j$。
   
   |     | A               | D               | C           | B   |
   | --- | --------------- | --------------- | ----------- | --- |
   | A   |                 |                 |             |     |
   | D   | $0.13\not>0.22$ |                 |             |     |
   | C   | $0.23\not>0.27$ | $0.10\not>0.22$ |             |     |
   | B   | $0.49>0.30$     | $0.36>0.27$     | $0.26>0.22$ |     |
    
得到結論：$A,D,C$ 的平均值沒有顯著差異，但 $B$ 的平均值顯著大於其他三個。

使用 Tukey's Method 和 Scheffe's Method 則會得到不同的結論：$A,D,C$ 之間沒有顯著差異，$C,B$ 之間有顯著差異，但 $B$ 顯著大於 $A,D,C$。

並且 $T_\alpha=0.30, S_{\alpha,cm}=\sqrt{\frac{MS_E*2}{4}}\sqrt{3\cdot F_{3,12,0.05}}=0.326$ 都是偏保守的檢定。

---

$$
Y_{ij}=\mu+\tau_i+\varepsilon_{ij}\implies \tau_i\begin{cases}
    \text{fixed}\\
    \text{random}
\end{cases}
$$

$\implies$ ANOVA for testing $H_0:$ No trt effect v.s. $H_1:$ At least one trt effect $\to H_0$ usually rejected.

- $\tau_i$: fixed $\to$ contrast for detailed analysis
- $\tau_i$: random $\to$ Variance components estimation problem. Basic way to do this is by ANOVA method.
  
  $\implies$ solve for each variance component and the solution is an est for that variance component.

  e.g. One-fator CRD (random model)

  $$
  \begin{gather*}
    E(MS_E)=\sigma_\varepsilon^2\xlongequal{\text{set}}MS_E\quad E(MS_{trt})=\sigma_\varepsilon^2+n\sigma_\tau^2\xlongequal{\text{set}}MS_{trt}\\
    \implies \hat{\sigma}_\varepsilon^2=MS_E\quad\hat{\sigma}_\tau^2=\frac{MS_{trt}-MS_E}{n}
  \end{gather*}
  $$

  與 MOME 類似。估計量可能為負，在這種情況下，我們應該將估計量設為 0 (P510-511)。
  