# LRT 的應用

$$
H_0:\theta\in \omega_0\quad\text{vs.}\quad H_1:\theta\in \omega_1
$$

$$
\lambda(\utilde{x})\triangleq \frac{\sup_{\theta\in\omega_0}L(\theta;\utilde{x})}{\sup_{\theta\in\omega}L(\theta;\utilde{x})}\in[0,1]
$$


LRT reject $H_0$ $\iff\lambda(\utilde{x})<k$


## Contingency table's Chi-square test

Note: LRT reject $H_0$ $\iff\lambda(\utilde{x})<k\iff-2\ln \lambda(\utilde{x})>c$

:::tip[Theorem]
As $n\to\infty$, 在一些條件下
$$
-2\ln\lambda(\utilde{x})\xrightarrow{d}\chi^2_{\text{df}}\quad\forall\theta\in \underbar{$\omega_0$}
$$
- $$\underbar{$\omega_0$}$$ : 在 $H_0$ 下發生 "$=$" 的 $\theta$ 的集合
- df = $$\dim(\Omega)-\dim(\underbar{$\omega_0$})$$，$\dim$ 指未知參數的數量

$\implies$ Reject $H_0\iff -2\ln\lambda(\utilde{x})>\chi^2_{\text{df},\alpha}$ 
:::

$$
\utilde{X}\sim\text{Multinomial}_n(k,\utilde{P}) \quad\text{with } k= I\times J
$$

我們關心 $H_0:A\perp B$ vs. $H_1:A\not\perp B$，並且收集到以下數據

| A\B      | 1        | 2        | $\cdots$ | j        | $\cdots$ | J        |     |
| -------- | -------- | -------- | -------- | -------- | -------- | -------- | --- |
| 1        | $X_{11}$ | $X_{12}$ | $\cdots$ | $X_{1j}$ | $\cdots$ | $X_{1J}$ |     |
| 2        | $X_{21}$ | $X_{22}$ | $\cdots$ | $X_{2j}$ | $\cdots$ | $X_{2J}$ |     |
| $\vdots$ | $\vdots$ | $\vdots$ | $\ddots$ | $\vdots$ | $\ddots$ | $\vdots$ |     |
| i        | $X_{i1}$ | $X_{i2}$ | $\cdots$ | $X_{ij}$ | $\cdots$ | $X_{iJ}$ |     |
| $\vdots$ | $\vdots$ | $\vdots$ | $\ddots$ | $\vdots$ | $\ddots$ | $\vdots$ |     |
| I        | $X_{I1}$ | $X_{I2}$ | $\cdots$ | $X_{Ij}$ | $\cdots$ | $X_{IJ}$ |     |
|          |          |          |          |          |          |          | $n$ |

$$
\begin{align*}
    p_{ij}&=P(A=i,B=j)\\
    &=\text{prob of being classified to } (i,j) \text{ cell}\\
\end{align*}
$$

i.e. $X_{ij}\sim B(n,p_{ij})\quad\forall i,j\implies\hat{p}=\frac{X_{ij}}{n}$

$$
\begin{align*}
    &\implies \Omega=\set{\utilde{p}\in\R^{I\times J}:\forall i,j\quad p_{ij}\in[0,1], \quad\sum_i\sum_j p_{ij}=1}\\
    &\implies \dim(\Omega)=I\times J-1\quad\text{(知道之前的數據，最後一個數據也能知道)} \\
\end{align*}
$$


$$
\begin{align*}
    H_0:A\perp B\iff p_{ij}&=p_{i\cdot}p_{\cdot j}\quad\forall i,j\\
    &=P(A=i)\cdot P(B=j)\\
    &=\sum_{j=1}^J p_{ij}\cdot\sum_{i=1}^I p_{ij}\\
    &\triangleq p_i\cdot p_j
\end{align*}
$$

$$
\text{i.e.}\quad \underbar{$\omega_0$}=\omega_0=\set{\utilde{p}\in\Omega:\forall i,j\quad p_{ij}=p_i\cdot p_j,\quad \sum_{i=1}^I p_i=1,\quad \sum_{j=1}^J p_j=1}
$$

$$
\begin{align*}
    &\implies \dim(\omega_0)=I-1+J-1=I+J-2\\
    &\implies \text{df}=\dim(\Omega)-\dim(\underbar{$\omega_0$})=IJ-1-(I+J-2)=(I-1)(J-1)
\end{align*}
$$

$$
\begin{align*}
    \text{Under } \Omega:&\quad\widehat{p_{ij}}_\Omega=\frac{X_{ij}}{n}\\
    &\implies (i,j) \text{ cell 的 期望值為 } n\cdot\widehat{p_{ij}}_\Omega
\end{align*}
$$

$$
\begin{align*}
    \text{Under }\omega_0:&\quad\widehat{p_{i\omega_0}}=\frac{X_i}{n},\quad \widehat{p_{j\omega_0}}=\frac{X_j}{n}\implies \widehat{p_{ij\omega_0}}=\widehat{p_{i\omega_0}}\cdot \widehat{p_{j\omega_0}}\\
    &\implies (i,j) \text{ cell 的 期望值為 } n\cdot\widehat{p_{ij\omega_0}} =\frac{X_i\cdot X_j}{n}\triangleq E_{ij}
\end{align*}

$$

$$
L(\utilde{p};\utilde{X})=\frac{n!}{X_{11}!\cdots X_{IJ}!}\prod_{i=1}^I\prod_{j=1}^J p_{ij}^{X_{ij}}
$$

$$
\begin{align*}
    \implies\lambda(\utilde{X})&=\frac{L(\widehat{p_{ij\omega_0}};\utilde{X})}{L(\widehat{p_{ij\Omega}};\utilde{X})}\\
    &=\prod_{i,j}\left(\frac{\frac{X_i\cdot X_j}{\cancel{n}}}{\frac{X_ij}{\cancel{n}}}\right)^{X_{ij}}\\
    &=\prod_{i,j}\left(\frac{X_i\cdot X_j}{X_{ij}}\right)^{X_{ij}}\\
\end{align*}
$$

更具一些泰勒展開的計算，可以得到以下結論

$$
    -2\ln\lambda(\utilde{X})\approx \sum_{i=1}^I\sum_{j=1}^J\frac{(X_{ij}-E_{ij})^2}{E_{ij}}\triangleq\chi^2 \quad\text{with } E_{ij}=\frac{X_i\cdot X_j}{n}
$$

$\implies $Reject $H_0$ at level $\alpha\iff \chi^2>\chi^2_{(I-1)(J-1),\alpha}$

---

**EX**：對於“全職與兼職對於退休金計劃的選擇”得到以下數據

| 類型\退休金計劃 | 1   | 2   | 3   |     |
| --------------- | --- | --- | --- | --- |
| 全職            | 160 | 140 | 40  | 340 |
| 兼職            | 40  | 60  | 60  | 160 |
|                 | 200 | 200 | 100 | 500 |

可以計算 $E_{ij}$

| $E_{ij}$ | 1   | 2   | 3   |
| -------- | --- | --- | --- |
| 1        | 136 | 136 | 68  |
| 2        | 64  | 64  | 32  |



$$
\begin{align*}
    \implies \chi^2&=\frac{(160-136)2}{136}+\frac{(140-136)2}{136}+\cdots+\frac{(32-32)2}{32}\\
    &=49.63>5.99=\chi^2_{(2-1)(3-1),0.05}
\end{align*}
$$

$\implies H_0$ is rejected at level 0.05. I.e. 全職與兼職對於退休金計劃的選擇不獨立。

---

**EX**：對於“某門課的成績與統計能力”是否有關，得到以下數據

| stat. grade\OR grade | A   | B   | C   | other |     |
| -------------------- | --- | --- | --- | ----- | --- |
| A                    | 25  | 6   | 17  | 13    | 61  |
| B                    | 17  | 16  | 15  | 6     | 54  |
| C                    | 18  | 4   | 18  | 10    | 50  |
| other                | 10  | 8   | 11  | 20    | 49  |
|                      | 70  | 34  | 61  | 49    | 214 |

$$
\begin{align*}
    &\implies \chi^2=\frac{(25-19.95)^2}{19.95}+\frac{(6-9.69)^2}{9.69}+\cdots=25.5>21.67=\chi^2_{(4-1)(4-1),0.01}\\
    &\implies H_0 \text{ is rejected at level 0.01.}
\end{align*}
$$

I.e. 這門課的成績與統計能力有顯著關係。

## Goodness-of-fit test

在分析資料時，我們都會先假設資料符合某種分佈，然後去估計分佈的參數。但我們還需要先檢驗，我們假設的分佈是否適合這些資料。

$$
\utilde{X}\sim \text{Multinomial}_n(k,\utilde{P})
$$

| 1             | 2             | $\cdots$ | k             | total     |
| ------------- | ------------- | -------- | ------------- | --------- |
| $X_1\mid P_1$ | $X_2\mid P_2$ | $\cdots$ | $X_k\mid P_k$ | $n\mid 1$ |

given $c_i\in[0,1]$ and $\sum c_i=1\implies H_0:P_i=c_i, \forall i\quad$ vs. $H_1:P_i\neq c_i$ for some $i$

$\implies \Omega=\set{\utilde{P}:P_i\in[0,1], \sum P_i=1}\quad\dim(\Omega)=k-1$

$$\implies \underbar{$\omega_0$}=\set{\utilde{P}:O_i=c_i, \forall i}\quad \dim(\underbar{$\omega_0$})=0$$

$E_i=n\cdot c_i,\forall i$

$$
\implies \chi^2=\sum_{i=1}^k\frac{(X_i-E_i)^2}{E_i}\xrightarrow{d}\chi^2_{k-1}
$$

$\implies$ Reject $H_0$ at level $\alpha\iff \chi^2>\chi^2_{k-1,\alpha}$

---

**EX**：用一個骰子得到以下數據

| 1   | 2   | 3   | 4   | 5   | 6   | total |
| --- | --- | --- | --- | --- | --- | ----- |
| 100 | 94  | 103 | 89  | 110 | 104 | 600   |

$H_0:$ 骰子是公平的 $\iff H_0:\utilde{p}=(\frac{1}{6},\cdots,\frac{1}{6})$

在 null hypothesis 下，$E_i=600\cdot\frac{1}{6}=100$

$$
\implies \chi^2=\frac{0^2+6^2+3^2+11^2+10^2+4^2}{100}=2.82\not > 9.236=\chi^2_{6-1,0.1}
$$

$\implies H_0$ is not rejected at level 0.1. I.e. 沒有足夠證據認為這個骰子不公平。

---

$Y_1\cdots,Y_n\sim Y$， $H_0: Y\sim f(y;\theta)$ vs. $H_1: Y\not\sim f(y;\theta)$，其中 $\theta$ 可能是未知的。

如果 $Y$ 是連續分佈，我們可以將 $f_y$ 分割成 $k$ 個區間，然後計算數據落在每個區間的數量，從而轉變成 Multinomial 分佈的問題。其中每個區間的幾率可以用積分來計算。


| 區間 | $I_1$ | $I_2$ | $\cdots$ | $I_k$ | n   |
| ---- | ----- | ----- | -------- | ----- | --- |
| prob | $P_1$ | $P_2$ | $\cdots$ | $P_k$ | 1   |
| 數量 | $X_1$ | $X_2$ | $\cdots$ | $X_k$ | n   |

$$
X_i=\sum_{j=1}^n I(Y_j\in I_i)\quad\frac{X_i}{n}=P_\theta(Y\in I_i)=P_i\quad P_i=\int_{I_i}f(y;\theta)dy
$$

假設檢定就轉變為 $X\sim \text{Multinomial}_n(k,\utilde{p})$

$$
H_0:\utilde{p}=\utilde{P} \text{ v.s. } H_1:\utilde{p}\in\Omega \text{ , with }\Omega=\set{\utilde{P}:P_i\in[0,1],\sum P_i=1}
$$

$$
\implies E_i=n\cdot \hat{P}_i=n\int_{I_i}f(y;\hat{\theta}_\text{MLE})dy
$$

$\implies$ Reject $H_0$ at level $\alpha\iff \chi^2>\chi^2_{\text{df},\alpha}$ with $\chi^2=\sum_{i=1}^k\frac{(X_i-E_i)^2}{E_i}$ and df=$$\dim(\Omega)-\dim(\underbar{$\omega_0$})=k-1-m$$，其中 $m$ 是 $\theta$ 的未知參數數量。

e.g.
- $H_0: Y\sim N(60,100)\implies \chi^2_{k-1}$
- $H_0: Y\sim N(\mu,100)\implies\chi^2_{k-1-1}$
- $H_0: Y\sim N(\mu,\sigma^2)\implies \chi^2_{k-1-2}$
- $H_0: Y\sim \text{Beta}(\theta,2)\implies \chi^2{k-1-1}$

:::note
因為 $\chi^2\to\infty$ as $E_i\to 0$，如果 $E_i$ 足夠接近 0，那麼幾乎可以確定 $H_0$ 會被拒絕。並且這個方法是建立在 $n\to\infty$ 的假設下的，需要足夠多的數據。因此在設置分割數 $k$ 時，有以下原則：
- $k-1-m\ge 1\iff k\ge m+1$
- $E_i\le 5$
:::