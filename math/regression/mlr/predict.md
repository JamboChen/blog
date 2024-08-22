# 预测

## 回歸參數的信賴區間

**Remark**:

$$
\utilde{W}\sim N_m(\utilde{\eta}, \cancel{\Sigma}_{\utilde{W}})\implies B_{l\times m}\utilde{W}+\utilde{C}_{l\times 1}\sim N_l(B\utilde{\eta}+\utilde{C}, B\cancel{\Sigma}_{\utilde{W}}B^t)
$$

If $D$ is full rank, then $\utilde{b}=(D^tD)^{-1}\utilde{Y}$, where $\utilde{Y}\sim N_n(D\utilde{\beta}, \sigma^2I)$
- $E\utilde{b}=\utilde{\beta}$
- $\sigma^2\set{\utilde{b}}=(D^tD)^{-1}D^t\sigma^2I(D^tD)^{-1}=\sigma^2(D^tD)^{-1}$

$\implies \utilde{b}\sim N_p(\utilde{\beta}, \sigma^2(D^tD)^{-1})$

**Note**:

$\utilde{e}=\utilde{Y}-\utilde{\hat{Y}}=(I-H)\utilde{Y}=M\utilde{Y}\sim N_n(\utilde{0}, \sigma^2M)$, and $\text{tr}(M)=n-p=\text{rank}(M)\implies M^{-1}$ 不存在
- $E[\utilde{e}]=M\cdot E[\utilde{Y}]=M\utilde{\theta}=\utilde{0}$
- $\sigma^2\set{\utilde{e}}=\sigma^2\cdot M$

$$
\begin{align*}
    \implies \sigma^2\left\{\begin{pmatrix}
      \utilde{b}\\
      \utilde{e}
  \end{pmatrix}\right\}&=\sigma^2\left\{\begin{pmatrix}
        (D^tD)^{-1}D^t\\
        M
  \end{pmatrix}\utilde{Y} \right\}\\
  &=\begin{pmatrix}
        (D^tD)^{-1}D^t\\
        M
  \end{pmatrix}\sigma^2I\begin{pmatrix}
        D(D^tD)^{-1}, M^t
  \end{pmatrix}\\
  &=\sigma^2\begin{bmatrix}
    (D^tD)^{-1} & 0\\
    0 & M
  \end{bmatrix}\\
  &=\begin{bmatrix}
    \sigma^2\set{\utilde{b}} & 0\\
    0 & \sigma^2\set{\utilde{e}}
  \end{bmatrix}\\
  \implies& \utilde{b}\perp\utilde{e}\implies \utilde{b}\perp\text{SSE}\triangleq ||\utilde{e}||^2
\end{align*}
$$

---

Now, $\utilde{b}\sim N_p(\beta, \sigma^2(D^tD)^{-1})$

$$
\implies b_j=(0, 0, \cdots, \underbrace{1}_{j\text{-th}}, 0, \cdots, 0)\utilde{b}=\utilde{c_j}^t\utilde{b}\sim N(\utilde{c_j}^t\utilde{\beta}, \underbrace{\utilde{c_j}^t\sigma^2\set{\utilde{b}}\utilde{c_j}}_{j\times j\text{ of }\sigma^2\set{\utilde{b}}})
$$

- $S^2\set{b_j}=\sigma^2\set{b_j}|_{\sigma^2=\text{MSE}}$

1. $\forall j=0,1,\cdots, k$
   $$
   \frac{b_j-\beta_j}{S\set{b_j}}\sim t_{n-p}
   $$
2. $$
   [b_j\plusmn S\set{b_j}t_{n-p,1-\alpha/2}]\triangleq \text{CI}(j;\alpha)\quad\text{ is }1-\alpha\text{ confidence interval for }\beta_j\quad\forall j=0,1,\cdots,k
   $$

   i.e. $P(\beta_j\in\text{CI}(j;\alpha))=1-\alpha$

3. 我們可以用上面的信賴區間來作建設鑒定
   $$
   H_0:\beta_j=\beta_{j,0}\quad\text{ v.s. }\quad H_1:\beta_j\ne \beta_{j,0}
   $$

   並且我們拒絕 $H_0$ 的條件是 $\beta_j\notin\text{CI}(j;\alpha)$。通常 $\beta_{j,0}=0$

**Note**: $CI(j;\alpha)$ 是對於每個 $\beta_j$ 的信賴區間。

在許多參數中，我們會關心哪些參數是顯著的，哪些參數是不顯著的。雖然我們可以一個一個做檢定，但有的時候我們需要同時考慮多個參數。這時我們需要用這些參數的共同（simultaneous）信賴區間來做判斷。

比如我們需要同時判斷 $(\beta_1, \beta_3, \beta_9)$ 是否顯著，我們可以用這三個參數的共同信賴區間來做判斷。雖然我們能找到它們各自的 $1-\alpha$ 信賴區間，但將三個區間作交集並不能得到它們共同的 $1-\alpha$ 信賴區間。i.e.

$$
\begin{align*}
    &P\left(\beta_1\in\text{CI}(1;\alpha), \beta_3\in\text{CI}(3;\alpha), \beta_9\in\text{CI}(9;\alpha)\right)\\
    =&P\left(\begin{pmatrix}
    \beta_1\\
    \beta_3\\
    \beta_9
\end{pmatrix}\in\text{CI}(1;\alpha)\times\text{CI}(3;\alpha)\times\text{CI}(9;\alpha)\right)\\
\ne& 1-\alpha \quad\text{ 通常 <} 1-\alpha
\end{align*}
$$

注意到

$$
\begin{align*}
    &P\left(\beta_j\in\text{CI}(j;\alpha)\right)=1-\alpha\\
    \iff& 1-P\left(\beta_j\notin\text{CI}(j;\alpha)\right)=1-\alpha\\
    \iff& P\left(\beta_j\notin\text{CI}(j;\alpha)\right)=\alpha
\end{align*}
$$

$$
\begin{align*}
    \implies & P(\beta_1\in\text{CI}(1;\alpha), \beta_3\in\text{CI}(3;\alpha), \beta_9\in\text{CI}(9;\alpha))\\
    =&1-P(\beta_1\notin\text{CI}(1;\alpha)\text{ or }\beta_3\notin\text{CI}(3;\alpha)\text{ or }\beta_9\notin\text{CI}(9;\alpha))\\
    \ge& 1-\sum_{j=1,3,9}P(\beta_j\notin\text{CI}(j;\alpha))=1-3\alpha
\end{align*}
$$

這樣我們就有了一個下界。在給定 $\alpha$ 時，我們可以找一個 $\alpha_0$ s.t.

$$
P\left(\beta_j\in\text{CI}(j;\alpha_0), j=1,3,9\right)\ge 1-3\alpha=1-\alpha\implies\alpha_0=\frac{\alpha}{3}
$$

一般來說，$\beta_{\pi(j)}, j=1,2,\cdots, g$ where $\pi(j), j=1,\cdots, g\in\set{0,1,\cdots, k}$。即我們在 $k+1$ 個參數中選擇 $g$ 個不同的參數，找它們的共同信賴區間。

$$
P\left(\beta_{\pi(j)}\in\text{CI}(\pi(j);\alpha_0), j=1,2,\cdots,g\right)\ge 1-g\alpha=1-\alpha\implies\alpha_0=\frac{\alpha}{g}
$$

:::info[Definition]
**Bonferroni joint confidence interval**:

*Bonferroni joint confidence interval* for $g, \beta_j$'s with family confidence coefficient $\ge 1-\alpha$ is

$$
\prod_{j=1}^g\text{CI}(\pi(j);\alpha_0)=\prod_{j=1}^g\left[\beta_{\pi(j)}\plusmn S\set{b_{\pi(j)}}t_{n-p,1-\frac{\alpha}{2g}}\right]
$$
:::

用這種方法在做檢驗 $H_0:\beta_{\pi(j)}=0, \forall j=1,2,\cdots, g$ vs $H_1:\text{ not }H0$ 時，reject $H_0$ at level $\alpha\iff 0\notin \text{CI}(\pi(j);\alpha/g)$ for some $j$.

而這個信賴區間在向量空間中會是一個立方體。這個立方體每個邊所在的子空間就是 $\beta_{\pi(j)}$ 所在的空間。

**Remark**: 當 $g$ 小的時候，Bonferroni 方法是好用的。但當 $g$ 很大時，$\alpha/g$ 會變得很小，這樣單個參數的信賴區間就會變得很寬，這樣的話我們就很難拒絕虛無假設了，這個方法的 power 會變得很低。

因為 *Bonferroni joint confidence interval* 獲得的共同信賴區間是大於 $1-\alpha$ 的，但我們當然更想要的是獲得恰好等於 $1-\alpha$ 的信賴區間。

**Note**:

By *lemma 6*, $\utilde{W}\sim N_m(\utilde{\eta}, \cancel{\Sigma}_{\utilde{W}})$, when $\cancel{\Sigma}_{\utilde{W}}$ exists, $(\utilde{W}-\utilde{\eta})^t\cancel{\Sigma}_{\utilde{W}}^{-1}(\utilde{W}-\utilde{\eta})\sim\chi^2_m$

$$
\utilde{b}\sim N_p(\beta, \sigma^2\set{\utilde{b}})
\implies (\utilde{b}-\utilde{\beta})^t(\sigma^2\set{\utilde{b}})^{-1}(\utilde{b}-\utilde{\beta})\sim\chi^2_p
$$

但我們不知道方差 $\sigma^2\set{\utilde{b}}$，所以我們用 $S^2\set{\utilde{b}}$ 來估計。因為 $S^2\set{\utilde{b}}/\sigma^2\set{\utilde{b}}=MSE/\sigma^2\implies S^2\set{\utilde{b}}=\sigma^2\set{\utilde{b}}MSE/\sigma^2$

$$
(\utilde{b}-\utilde{\beta})^t(S^2\set{\utilde{b}})^{-1}(\utilde{b}-\utilde{\beta})
=\frac{(\utilde{b}-\utilde{\beta})^t(\sigma^2\set{\utilde{b}})^{-1}(\utilde{b}-\utilde{\beta})\sim \chi^2_p}{\frac{MSE}{\sigma^2}\sim\frac{\chi^2_{n-p}}{n-p}}
$$

因為分母與 $\utilde{b}$ 相關，而分子與 $\utilde{e}$ 相關，並且 $\utilde{b}\perp\utilde{e}$，所以分子與分母獨立。

$$
\implies \frac{(\utilde{b}-\utilde{\beta})^t(S^2\set{\utilde{b}})^{-1}(\utilde{b}-\utilde{\beta})}{p}\sim \frac{\chi^2_p/p}{\chi^2_{n-p}/(n-p)}\sim F_{p,n-p}
$$

$$
\begin{align*}
    1-\alpha&=P\left(\underbrace{\frac{(\utilde{b}-\utilde{\beta})^t(S^2\set{\utilde{b}})^{-1}(\utilde{b}-\utilde{\beta})}{p}>F_{p,n-p,\alpha}}_{(*)}\right)\\
    &=P\left(\utilde{\beta}\in C^*(\utilde{b};\alpha)\right)
\end{align*}
$$

$\implies C^*(\utilde{b};\alpha)=\set{\utilde{\beta}:(*)\text{ holds}}$ 就恰好是 $\utilde{\beta}$ 的 $1-\alpha$ 信賴區間。這對於 $\utilde{\beta_\pi}$ 也同樣成立。

而這個信賴區間在向量空間中會是一個橢圓體（ellipsoid）。並且這個橢球的體積會比 Bonferroni 方法得到的立方體體積小，相當於是把立方體的角削圓了。

## 新值的預測

> If $D$ 是 full rank $\implies\forall\utilde{x_h}^t\in\R^p$ can manke $\utilde{x_h}^t\utilde{\beta}$ estimable

當給定一組新的 $\utilde{x_h}$ 時，我們會關係一些問題，比如：

- $E(Y_h)=\utilde{x_h}\utilde{\beta}$，即 $Y_h$ 的期望值。對於這個問題我們會做點估計和信賴區間估計。
- 隨機變量 $Y_h$ 。而這個問題我們會做預測區間估計（prediction interval）。

我們仍然會使用 normal 回歸模型，即

$$
Y_h=\utilde{x_h}^t\utilde{\beta}+\varepsilon_h\qquad \varepsilon_h\sim N(0, \sigma^2)\text{ and }\varepsilon_h\perp\utilde{\varepsilon}
$$

### 期望值的點估計

根據 *Gauss-Markov Theorem*，$\hat{Y}_h=\utilde{x_h}^t\utilde{b}$ 是 $E(Y_h)=\utilde{x_h}^t\utilde{\beta}$ 的 BLUE。

- $E[\hat{Y}_h]=\utilde{x_h}^t\utilde{\beta}$ (unbiased)
- $\sigma^2\set{\hat{Y}_h}=\utilde{x_h}^t\sigma^2\set{\utilde{b}}\utilde{x_h}\sigma^2=\sigma^2\utilde{x_h}^t(D^tD)^{-1}\utilde{x_h}$

i.e. $\hat{Y}_h\sim N(\utilde{x_h}^t\utilde{\beta}, \sigma^2\set{\hat{Y}_h})$

$$
\frac{\hat{Y}_h-\utilde{x_h}\utilde{\beta}}{S\set{\hat{Y}_h}}\sim t_{n-p}
$$

### 期望值的信賴區間

對於所有的 $\utilde{x_h}, E[Y_h]=\utilde{x_h}^t\utilde{\beta}$ 的 $1-\alpha$ 信賴區間為 $[\hat{Y}_h\plusmn S\set{\hat{Y}_h}t_{n-p, 1-\frac{\alpha}{2}}]$, i.e.

$$
\forall\utilde{x_h}\in\R^p, P\left(\utilde{x_h}^t\utilde{\beta}\in[\hat{Y}_h\plusmn S\set{\hat{Y}_h}t_{n-p, 1-\frac{\alpha}{2}}]\right)=1-\alpha
$$

### 新值的預測區間

我們現在想要要預測 $\utilde{X_h}$ 上的新值 $Y_{n,\text{new}}$ 。

$$
Y_{h, \text{new}}=\utilde{x_h}^t\utilde{\beta}+\varepsilon_h\sim N(\utilde{x_h}^t\utilde{\beta}, \sigma^2)
$$

因為平均的預測 $\hat{Y}_h$ 是 $\utilde{b}$ 的線性組合，而 $e\perp\utilde{b}\implies Y_{h, \text{new}}\perp\hat{Y}_h$

$$
\begin{align*}
    &\implies \hat{Y_h}-Y_{h, \text{new}}\sim N(0, \sigma^2\set{\hat{Y}_h}+\sigma^2)\qquad \sigma^2\set{\text{perd}}\triangleq\sigma^2\set{\hat{Y}_h}+\sigma^2\\
    &\implies \frac{\hat{Y_h}-Y_{h, \text{new}}}{S\set{\text{perd}}}\sim \frac{N(0,1)}{\sqrt{\text{MSE}/\sigma^2}}\sim t_{n-p}
\end{align*}
$$

因此 $\forall \utilde{x_h}\in\R^p$，$Y_{h,\text{new}}$ 的 $1-\alpha$ 預測區間為 $[\hat{Y}_h\plusmn S\set{\text{perd}}t_{n-p, 1-\frac{\alpha}{2}}]$

### m 個預測值平均的預測區間

如果要我們在同一個點 $\utilde{x_h}$ 獲得 $m$ 個新值，我們會想要知道這 $m$ 個新值的平均值的預測區間。

$$
Y_{h, \text{new}}=\utilde{x_h}^t\utilde{\beta}+\varepsilon_h\sim N(\utilde{x_h}^t\utilde{\beta}, \sigma^2)
$$

因為平均的預測 $\hat{Y}_h$ 是 $\utilde{b}$ 的線性組合，而 $e\perp\utilde{b}\implies \bar{Y}_{h, \text{new},m}\perp\hat{Y}_h$

$$
\begin{align*}
    &\implies\hat{Y}-\bar{Y}_{h, \text{new},m}\sim N(0, \sigma^2\set{\hat{Y}_h}+\frac{\sigma^2}{m})\\
    &\implies\frac{\hat{Y}-\bar{Y}_{h, \text{new},m}}{S\set{\text{perd. mean}}+\frac{\sigma^2}{m}}\sim t_{n-p} \qquad \sigma^2\set{\text{perd. mean}}\triangleq\sigma^2\set{\hat{Y}_h}+\frac{\sigma^2}{m}
\end{align*}
$$

因此 $\forall \utilde{x_h}\in\R^p$，$m$ 個新值的平均值的 $1-\alpha$ 預測區間為 $[\hat{Y}_h\plusmn S\set{\text{perd. mean}}t_{n-p, 1-\frac{\alpha}{2}}]$

### 回歸線的信賴區帶

我們想要知道整條回歸線的 $1-\alpha$ 信賴區帶，即對於所有的 $\utilde{x_h}\in\R^p, \hat{Y}_h=\utilde{x_h}^t\utilde{\beta}$ 都在這個區帶內的機率為 $1-\alpha$。

因此我們需要找到一個常數 $M_\alpha$ s.t.

$$
P\left(\left|\frac{\hat{Y}_h-\utilde{x_h}^t\utilde{\beta}}{S\set{\hat{Y}_h}}\right|<M_\alpha, \forall \utilde{x_h}\in\R^p\right)=1-\alpha
$$

Note:

$$
\begin{align*}
    &\frac{\hat{Y}_h-\utilde{x_h}^t\utilde{\beta}}{S\set{\hat{Y}_h}}\le M_\alpha, \forall \utilde{x_h}\in\R^p\\
    \iff &\frac{(\hat{Y}_h-\utilde{x_h}^t\utilde{\beta})^t}{S^t\set{\hat{Y}_h}}\le M_\alpha^2, \forall \utilde{x_h}\in\R^p\\
    \iff&\frac{1}{\text{MSE}}\max_{\utilde{x}_h\in\R^p}\left[\frac{(\utilde{x_h}^t(\utilde{b}-\utilde{\beta}))^2}{\utilde{x_h}^t\underbrace{(D^tD)^{-1}}_{p\times p\text{ sym.}}\utilde{x_h}}\right]\le M_\alpha^2
\end{align*}
$$

:::tip[Lemma 8]
Let S: $p\times p$ sysmetric positive definite matrix, then $\forall\utilde{r}, \utilde{x}\in\R^p$

$$
(\utilde{r}^t\utilde{x})^2\le (\utilde{r}^tS\utilde{r})(\utilde{x}^tS^{-1}\utilde{x})

$$
e.g. $S=I\implies$ Cauchy-Schwarz inequality
:::

$$
\implies \max_{\utilde{x}\in\R^p}\left[\frac{(\utilde{r}^t\utilde{x})^2}{\utilde{x}^tS^{-1}\utilde{x}}\right]=\utilde{r}^tS\utilde{r}
$$

$$
\begin{align*}
    \iff& \frac{1}{\text{MSE}}(\utilde{b}-\utilde{\beta})^t(D^tD)^{-1}(\utilde{b}-\utilde{\beta})\le M_\alpha^2\\
    \iff& \frac{(\utilde{b}-\utilde{\beta})^t(\sigma^2\set{\utilde{b}})^{-1}(\utilde{b}-\utilde{\beta})}{\frac{\text{MSE}}{\sigma^2}}\le M_\alpha^2
\end{align*}
$$

根據 *lemma 6*，分子的部分服從 $\chi^2_p$，並且因為分子是 $\utilde{b}$ 的線性組合，所以與分母獨立。因此

$$
\frac{(\utilde{b}-\utilde{\beta})^t(\sigma^2\set{\utilde{b}})^{-1}(\utilde{b}-\utilde{\beta})\sim\chi^2_p}{\frac{\text{MSE}}{\sigma^2}\sim\frac{\chi^2_{n-p}}{n-p}}\overset{\text{d}}{=} \frac{p\cdot \frac{\chi^2_p}{p}}{\frac{\chi^2_{n-p}}{n-p}}\overset{\text{d}}{=}p\cdot F_{p,n-p}
$$

$$
\begin{align*}
    \implies 1-\alpha&=P\left(\left|\frac{\hat{Y}_h-\utilde{x_h}^t\utilde{\beta}}{S\set{\hat{Y}_h}}\right|<M_\alpha, \forall \utilde{x_h}\in\R^p\right)\\
    &=P(p\cdot W\le M_\alpha^2)\quad\text{ where }W\sim F_{p,n-p}\\
    \implies \frac{M_\alpha^2}{p} &= F_{p,n-p,\alpha}\\
    \implies M_\alpha&=\sqrt{p\cdot F_{p,n-p,\alpha}}
\end{align*}
$$

因此 $\left[\hat{Y}_h\plusmn S\set{\hat{Y}_h}\sqrt{p\cdot F_{p,n-p,\alpha}}\right]$ 就是整條回歸線的 $1-\alpha$ 信賴區帶。

**Remark**:

對於以上幾種不同的區間在同一個 $\utilde{x_h}$ 時，他們的寬度有以下關係：

- 預測值 > m 個預測值平均 > 期望值 < 回歸線
- 當 m 越大時，預測值平均的方差會趨近於期望值的方差，因此兩者的區間會趨近於相等。

## General Linear Test Approach

當我們在做例如 $H_1:\beta_1=\beta_3=\beta_9=0$ v.s. $H_1: $ not $H_0$ 的檢定時，*Bonferroni joint confidence interval* 可以給我們一個方法得到 level $\le \alpha$ 的信賴區間。但這種方法可能會太保守。而 *General Linear Test Approach* 可以給我們一個方法得到 level $=\alpha$ 的檢定。並且 GLT 在有計算機的情況下是很容易計算的。

Recall: SLR, $p=2, i=1,2,\cdots,n$

$$
Y_i=\beta_0+\beta_1x_i+\varepsilon_i \quad \varepsilon_i\overset{\text{iid}}{\sim} N(0, \sigma^2)
$$

To test $H_0:\beta_1=0$ v.s. $H_1:\beta_1\ne 0$

$$
\begin{align*}
    &\text{test stat is } F^*=\frac{MSR}{MSE}=\frac{SSR/p-1}{SSE/(n-p)}=\frac{\frac{\text{SSTO-SSE}}{n-1-(n-p)}}{\frac{\text{SSE}}{n-p}}\\
    &\text{reject } H_0\iff F^*>F_{p-1,n-p,\alpha}
\end{align*}
$$

一般來說，$H_1$ 會假設模型是一個 full model，$H_0$ 會假設模型是一個 reduced model i.e. full model 中的某些參數被假設為 0。因此

$$
\text{Reduced model }\subset\text{ Full model}
$$

$$
\implies \Omega_0=\text{ col space of reduced model}\subset\Omega_1=\text{ col space of full model}
$$

無論哪種假設，對 $\utilde{Y}$ 做估計都是把 $\utilde{Y}$ 投影到 col space 上。假設
- $\utilde{\hat{Y}}_{\Omega_1}: \utilde{Y}$'s projection onto $\Omega_1$
- $\utilde{\hat{Y}}_{\Omega_0}: \utilde{Y}$'s projection onto $\Omega_0$

e.g. 

$$
H_0:Y=\beta_0+\beta_2x_2+\epsilon\text{ v.s. }H_1:Y=\beta_0+\beta_1x_1+\beta_2x_2+\epsilon
$$
$$
\implies D = [\utilde{1}, \utilde{x_2}] \text{ for } H_0\text{ and } D=[\utilde{1}, \utilde{x_1}, \utilde{x_2}] \text{ for } H_1
$$

$\implies ||\utilde{\hat{\utilde{Y}}}_{\Omega_1}||^2\ge||\utilde{\hat{\utilde{Y}}}_{\Omega_0}||^2$，因為 $\utilde{\hat{Y}}_{\Omega_1}$ 在某些方向上的投影會比 $\utilde{\hat{Y}}_{\Omega_0}$ 更長。因為兩種假設是在同一個 $\utilde{Y}$ 上做估計，所以

$$
\begin{align*}
    ||\utilde{Y}||^2&=||\utilde{\hat{Y}}_{\Omega_1}||^2+||e_{\Omega_1}||^2\\
    &=||\utilde{\hat{Y}}_{\Omega_0}||^2+||e_{\Omega_0}||^2
\end{align*}
$$

$$
\implies \text{SSE}_{\text{F}}=||e_{\Omega_1}||^2\le ||e_{\Omega_0}||^2=\text{SSE}_{\text{R}}
$$

因為我們認為越簡單的模型越好。如果一個參數少的模型和參數多的模型有接近的解釋能力，我們會選擇參數少的模型。因此這裡的想法是，如果 $\text{SSR}_\text{R}\approx\text{SSR}_\text{F}$，那麼我們就會選擇 reduced model。i.e.

$$
\begin{align*}
    \text{Reject }H_0&\iff \text{SSR}_\text{R}-\text{SSR}_\text{F}\text{ is large}\\
    &\iff F^{**}=\frac{\frac{\text{SSE}_\text{R}-\text{SSE}_\text{F}}{\text{df}_\text{R}-\text{df}_\text{F}}}{\frac{\text{SSE}_\text{F}}{\text{df}_\text{F}}}>F(\text{df}_\text{R}-\text{df}_\text{F}, \text{df}_\text{F}, \alpha)
\end{align*}
$$

其中 $\text{df}_\text{R}-\text{df}_\text{F}$ 代表了 $H_0$ 下被假設為 0 的參數的數量。

在 SLR 中：

$$
F^{**}=\frac{\frac{\text{SSTO}-\text{SSE}}{(n-1)-(n-p)}}{\frac{\text{SSE}}{\text{df}}=\frac{\frac{\text{SSR}}{p-1}}{\frac{\text{SSE}}{n-p}}}=\frac{\text{MSR}}{\text{MSE}}=F^*
$$

GLT 3 steps:
1. Fit the full model and obtain $\text{SSE}_\text{F}, \text{df}_\text{F}$
2. Fit the reduced model and obtain $\text{SSE}_\text{R}, \text{df}_\text{R}$
3. $$
    F^{**}=\frac{\frac{\text{SSE}_\text{R}-\text{SSE}_\text{F}}{\text{df}_\text{R}-\text{df}_\text{F}}}{\frac{\text{SSE}_\text{F}}{\text{df}_\text{F}}}\overset{H_0}{\sim} F(\text{df}_\text{R}-\text{df}_\text{F}, \text{df}_\text{F})
   $$

**Remark**:
1. $\text{df}_\text{R}-\text{df}_\text{F}$ 代表了 $H_0$ 下被假設為 0 的參數的數量。
2. SSTO 在任何假設下都是一樣的 $\implies \text{SSE}_\text{R}-\text{SSE}_\text{F}=\text{SSR}_\text{F}-\text{SSR}_\text{R}$

---

**EX**: $Y=\beta_0+\beta_1x_1+\beta_2x_2+\beta_3x_3+\beta_4x_4+\varepsilon$

To test $H_0:\beta_1=\beta_2=0$ v.s. $H_1:$ not $H_0$

$$
F^*=\frac{\frac{\text{SSE}_{\text{fm34}}-\text{SSE}_{\text{fm1234}}}{n-3-(n-5)}}{\frac{\text{SSE}_{\text{fm1234}}}{n-5}}>F(2,5,\alpha)\begin{cases}
    \text{Yes: reject }H_0\\
    \text{No: not reject }H_0
\end{cases}
$$