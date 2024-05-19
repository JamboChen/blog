# 變異數分析模型（ANOVA Model）

Anova Model 通常在分析定性型（Qualitative）變數時使用，例如：教育程度，血型等。在這種情況下，我們將變數稱為*因素（factor）*。

Remark: ANOVA Model 只是回歸模型的一種特例。

## One-way ANOVA Model

*One-way* 是指在單個變數上進行變異數分析。我們會使用以下模型

$$
Y=E(Y|X)+\varepsilon
$$

在第 $i$ 種 X 下 $E(Y|X=i)=\mu_i,i=1,\cdots,k$。e.g. 如果 X 是血型，那麼 $i=$ A, B, AB, O。

而我們的假設檢定也會是

$$
H_0:\text{ X 對 Y 無影響 }\iff H_0:\mu_1=\mu_2=\cdots=\mu_k 
$$

我們在收集數據時，每個種類 $i$ 都會收集 $n_i$ 個樣本，總共收集 $n$ 個樣本。因此我們會把模型寫成

$$
Y_{ij}=\mu_i+\varepsilon_{ij},\quad i=1,\cdots,k,j=1,\cdots,n_i, n=\sum_{i=1}^k n_i
$$

$$
\begin{pmatrix*}
    Y_{11}\\
    \vdots\\
    Y_{1n_1}\\
    Y_{21}\\
    \vdots\\
    Y_{2n_2}\\
    \vdots\\
    Y_{kn_k}
\end{pmatrix*}= \begin{pmatrix*}
    \mu_1\\
    \vdots\\
    \mu_1\\
    \mu_2\\
    \vdots\\
    \mu_2\\
    \vdots\\
    \mu_k
\end{pmatrix*}+\begin{pmatrix*}
    \varepsilon_{11}\\
    \vdots\\
    \varepsilon_{1n_1}\\
    \varepsilon_{21}\\
    \vdots\\
    \varepsilon_{2n_2}\\
    \vdots\\
    \varepsilon_{kn_k}
\end{pmatrix*}\implies
\utilde{Y}_{n\times 1}=\underbrace{\begin{bmatrix*}
    \utilde{1}_{n_1}&0&\cdots&0\\
    0&\utilde{1}_{n_2}&\cdots&0\\
    \vdots&\vdots&\ddots&\vdots\\
    0&0&\cdots&\utilde{1}_{n_k}
\end{bmatrix*}}_{D}
\underbrace{\begin{bmatrix*}
    \mu_1\\
    \mu_2\\
    \vdots\\
    \mu_k
\end{bmatrix*}}_{\utilde{\beta}}+\utilde{\varepsilon}_{n\times 1}
$$

$$
\begin{align*}
    \implies & D^tD=\text{diag}(n_1,\cdots,n_k)\\
    &D^t\utilde{Y}=\begin{pmatrix*}
        \sum_{j=1}^{n_1}Y_{1j}\\
        \vdots\\
        \sum_{j=1}^{n_k}Y_{kj}
    \end{pmatrix*}=
    \begin{pmatrix*}
        Y_{1\cdot}\\
        \vdots\\
        Y_{k\cdot}
    \end{pmatrix*}\quad \text{where } Y_{i\cdot}=\sum_{j=1}^{n_i}Y_{ij} \text{ called treatment totals}\\
\end{align*}
$$

LSE of $\utilde{\mu}$

$$
\utilde{b}=\utilde{\hat{\mu}}=(D^tD)^{-1}D^t\utilde{Y}=\begin{pmatrix*}
    n_1&0&\cdots&0\\
    0&n_2&\cdots&0\\
    \vdots&\vdots&\ddots&\vdots\\
    0&0&\cdots&n_k
\end{pmatrix*}^{-1}\begin{pmatrix*}
    Y_{1\cdot}\\
    \vdots\\
    Y_{k\cdot}
\end{pmatrix*}=\begin{pmatrix*}
    \bar{Y}_{1\cdot}\\
    \vdots\\
    \bar{Y}_{k\cdot}
\end{pmatrix*}=\text{ treatment sample mean}
$$

$\utilde{\hat{Y}}=D\utilde{b}$, i.e. $\hat{Y}_{ij}=\bar{Y}_{i\cdot}, \forall i,j$ 
- SSR = $\sum\sum(\hat{Y}_{ij}-\bar{Y}_{\cdot\cdot})=\sum\sum(\bar{Y}_{i\cdot}-\bar{Y}_{\cdot\cdot})^2$
- SSE = $\sum\sum(Y_{ij}-\hat{Y}_{ij})^2=\sum\sum(Y_{ij}-\bar{Y}_{i\cdot})^2$

One-way ANOVA with 

$$
Y_{ij}=\mu_i+\varepsilon_{ij}=E(Y|X=i)+\varepsilon_{ij} \text{ is called cell-mean model}
$$

Let $\mu=\frac{1}{k}\sum_{i=1}^k\mu_i=\mu, \tau_i=\mu_i-\mu,i=1,\cdots,k$ and $\sum_{i=1}^k\tau_i=0$

$$
Y_{ij}=\mu_i+\varepsilon_{ij}=\mu+\mu_i-\mu+\varepsilon_{ij}=\mu+\tau_i+\varepsilon_{ij} \text{ is called factor-effect model}
$$

## Two-way ANOVA Model

Two factors:
- A with levels a
- B with levels b

$$
Y=E(Y|A=i,B=j)+\varepsilon
$$

cell-mean model: $Y_{ijk}=\mu_{ij}+\varepsilon_{ijk}$ where $\varepsilon_{ijk}\sim N(0,\sigma^2), k=1,\cdots,n_{ij}$

factor-effect model: $Y_{ijk}=\mu+A_i+B_j+(\alpha\beta)_{ij}+\varepsilon_{ijk}$ 

