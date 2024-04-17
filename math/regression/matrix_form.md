# 矩陣形式的迴歸模型

當我們有 $n$ 筆數據，並且有 $k$ 個自變數時，我們有以下的迴歸模型：

$$
\begin{align*}
    Y_1=&\beta_0+\beta_1X_{11}+\beta_2X_{12}+\cdots+\beta_kX_{1k}+\varepsilon_i\\
    Y_2=&\beta_0+\beta_1X_{21}+\beta_2X_{22}+\cdots+\beta_kX_{2k}+\varepsilon_2\\
    &\vdots\\
    Y_n=&\beta_0+\beta_1X_{n1}+\beta_2X_{n2}+\cdots+\beta_kX_{nk}+\varepsilon_n
\end{align*}
$$

我們可以將這個模型寫成矩陣形式：

$$
\utilde{Y}=\underbrace{\begin{bmatrix}
    1 & X_{11} & X_{12} & \cdots & X_{1k}\\
    1 & X_{21} & X_{22} & \cdots & X_{2k}\\
    \vdots & \vdots & \vdots & \ddots & \vdots\\
    1 & X_{n1} & X_{n2} & \cdots & X_{nk}
\end{bmatrix}}_{\text{Design Matrix}}\utilde{\beta}+\utilde{\varepsilon}
$$

因為 $\utilde{X}$ 是由我們提供，所以我們將與 $\utilde{\beta}$ 有關的部分稱為設計矩陣（Design Matrix），記作 $D$。

$$
\implies \utilde{Y}_{n\times 1}=D_{n\times p}\utilde{\beta}_{p\times 1}+\utilde{\varepsilon}_{n\times 1}
$$

## 矩陣形式下的基礎定義和結論

:::info[Definition]
$$
W_{n\times 1} = \begin{bmatrix}
    W_1\\
    W_2\\
    \vdots\\
    W_n
\end{bmatrix}: \text{ random vector}\quad

U_{I\times J} = \begin{bmatrix}
    U_{11} & U_{12} & \cdots & U_{1J}\\
    U_{21} & U_{22} & \cdots & U_{2J}\\
    \vdots & \vdots & \ddots & \vdots\\
    U_{I1} & U_{I2} & \cdots & U_{IJ}
\end{bmatrix}: \text{ random matrix}
$$

$$
\implies E[W] = \begin{bmatrix}
    E[W_1]\\
    E[W_2]\\
    \vdots\\
    E[W_n]
\end{bmatrix}\qquad
E[U] = \begin{bmatrix}
    E[U_{11}] & E[U_{12}] & \cdots & E[U_{1J}]\\
    E[U_{21}] & E[U_{22}] & \cdots & E[U_{2J}]\\
    \vdots & \vdots & \ddots & \vdots\\
    E[U_{I1}] & E[U_{I2}] & \cdots & E[U_{IJ}]
\end{bmatrix}
$$

$$
\begin{align*}
    \implies \sigma^2\set{W} &=E\left[\left(W-E[W]\right)_{m\times 1}\left(W-E[W]\right)_{1\times m} ^t\right]\\
    &=\begin{bmatrix}
        Var[W_1] & Cov[W_1,W_2] & \cdots & Cov[W_1,W_n]\\
        Cov[W_2,W_1] & Var[W_2] & \cdots & Cov[W_2,W_n]\\
        \vdots & \vdots & \ddots & \vdots\\
        Cov[W_n,W_1] & Cov[W_n,W_2] & \cdots & Var[W_n]
    \end{bmatrix}\\
    &=\begin{bmatrix}
        Var & Cov\\
        Cov & Var
    \end{bmatrix} \quad \text{is symmetric}\\
    &=\text{ Variance-Covariance Matrix of } W

\end{align*}
$$
:::

**性質**：設 $A, B, C$ 是常數向量/矩陣，$W$ 是隨機向量，$U$ 是隨機矩陣，則：
1. E[A] = A
2. E[AUB+C] = AE[U]B+C
3. $\sigma^2\set{W}=E(ww^t)-E[W](E[W])^t$
4. $\sigma^2\set{A_{n\times w}W_{w\times 1}}_{n\times n} = A_{n\times w}\sigma^2\set{W}_{w\times w}A^t_{w\times n}$
5. $\sigma^2\set{AW+B}=\sigma^2\set{AW}$

**Note**: 如果 $W$ 是 $m\times 1$ 隨機向量，則 $\sigma^2\set{W}$ 是 $m\times m$ 的對稱矩陣。並且 $\forall \utilde{a}\in\R^n$ 是常數向量
$$
\utilde{a}^t\sigma^2\set{W}\utilde{a}=\sigma^2\set{\utilde{a}^tW}\ge 0
$$

因此 $\utilde{a}^t\sigma^2\set{W}\utilde{a}$ 是半正定矩陣(Positive Semi-Definite)。而等於號僅在 $\utilde{a}=0$ 時成立。

:::info[Definition]
令 $f:\R^k\to\R$，並且 $f(\utilde{\theta})\in\R, \forall\theta=(\theta_1, \cdots, \theta_k)^t\in\R^k$

$$
\frac{\partial}{\partial\utilde{\theta}} f(\utilde{\theta}) \triangleq \begin{bmatrix}
    \frac{\partial f}{\partial\theta_1}\\
    \frac{\partial f}{\partial\theta_2}\\
    \vdots\\
    \frac{\partial f}{\partial\theta_k}
\end{bmatrix}
$$
:::

:::tip[lemma 1]
Given $\utilde{c}\in\R^k$, $f(\utilde{\theta})=\utilde{c}^t\utilde{\theta}=\utilde{\theta}^t\utilde{c}$, $\forall \utilde{\theta}\in\R^k$
$$
\frac{\partial}{\partial\utilde{\theta}}f(\utilde{\theta})=\utilde{c}
$$
i.e. $\frac{\partial}{\partial\utilde{\theta}}(\utilde{c}^t\utilde{\theta})=\frac{\partial}{\partial\utilde{\theta}}(\utilde{\theta}^t\utilde{c})=\utilde{c}$
:::

:::tip[lemma 2]
如果 $A$ 是 $k\times k$ 的對稱常數矩陣，則以下形式的矩陣被稱為二次型(Quadratic Form)：
$$
f(\utilde{\theta})=\utilde{\theta}^tA\utilde{\theta}=\sum_{i,j}\theta_iA_{ij}\theta_j
$$

並且

$$
\frac{\partial}{\partial\utilde{\theta}}f(\utilde{\theta})=2A\utilde{\theta}
$$

如果 $A$ 不一定對稱，則 $\frac{\partial}{\partial\utilde{\theta}}f(\utilde{\theta})=A\utilde{\theta}+A^t\utilde{\theta}$
:::

因此在矩陣形式下的一般線性回歸模型會有：

$$
\utilde{Y}_{n\times 1}=D_{n\times p}\utilde{\beta}_{p\times 1}+\utilde{\varepsilon}_{n\times 1}\text{ with } E[\utilde{\varepsilon}]=0, \sigma^2\set{\utilde{\varepsilon}}=\begin{bmatrix}
    \sigma^2& \cdots &0\\
    \vdots & \ddots & \vdots\\
    0 & \cdots & \sigma^2
\end{bmatrix}
=\sigma^2I_{n\times n}
$$

$$
\begin{align*}
    \implies &E[\utilde{Y}]=E[D\utilde{\beta}+\utilde{\varepsilon}]=D\utilde{\beta}=\text{ regression function}\\
    &\sigma^2\set{\utilde{Y}}=\sigma^2\set{D\utilde{\beta}+\utilde{\varepsilon}}=\sigma^2\set{\utilde{\varepsilon}}=\sigma^2I_{n\times n}
\end{align*}
$$

:::info[Definition]
$$
\begin{align*}
    Q(\utilde{\beta})\triangleq& ||\utilde{Y}-E[\utilde{Y}]||^2\\
    &=||\utilde{Y}-D\utilde{\beta}||^2\\
    &=(\utilde{Y}-D\utilde{\beta})^t(\utilde{Y}-D\utilde{\beta})\\
\end{align*}
$$

如果 $Q(\utilde{b})=\min_{\utilde{\beta}\in\R^n}$，則 $\utilde{b}$ 是 $\utilde{\beta}$ 的 LSE。
:::

注意到

$$
\begin{align*}
    Q(\utilde{\beta})&=\utilde{Y}^t_{1\times n}\utilde{Y}_{n\times 1}-\utilde{Y}^t_{1\times n}D_{n\times p}\utilde{\beta}_{p\times 1}-\utilde{\beta}^t_{1\times p}D^t_{p\times n}\utilde{Y}_{n\times 1}+\utilde{\beta}^t_{1\times p}D^t_{p\times n}D_{n\times p}\utilde{\beta}_{p\times 1}\\
    &=\utilde{Y}^t\utilde{Y}-2\utilde{\beta}^tD^t\utilde{Y}+\utilde{\beta}^tD^tD\utilde{\beta}\\
\end{align*}
$$

是 $p\times 1$ 的矩阵。因此根据之前两个 Lemma，我们可以得到

$$
\frac{\partial}{\partial\utilde{\beta}}Q(\utilde{\beta})=-2D^t\utilde{Y}+2D^tD\utilde{\beta}
$$

如果 $\utilde{b}$ 是 $\utilde{\beta}$ 的 LSE $\iff$ $\frac{\partial}{\partial\utilde{\beta}}Q(\utilde{\beta})|_{\utilde{b}}=0 \iff -2D^t\utilde{Y}+2D^tD\utilde{b}=0$

:::info[Definition]
**Normal Equation**:

$$
D^tD\utilde{b}=D^t\utilde{Y}
$$
:::

:::tip[Theorem]
$$
\utilde{b}\text{ is LSE of }\utilde{\beta}\iff D^tD\utilde{b}=D^t\utilde{Y}
$$

$$
\implies \utilde{\hat{Y}}\triangleq D\utilde{b} \text{ called fitted value}\qquad \utilde{e}\triangleq \utilde{Y}-\utilde{\hat{Y}}\text{ called residual}
$$
:::

---

有的時候我們並不需要 $\utilde{b}$，我們只關心 $\utilde{\hat{Y}}$。這裡我們討論樣本數量 $n$ 大於參數數量 $p$ 的情況。

令 $\utilde{\theta}=D\utilde{\beta} \implies E[\utilde{Y}]=D\utilde{\beta}=\utilde{\theta}$ 並且 $\utilde{Y}=D\utilde{\beta}+\utilde{\varepsilon}=\utilde{\theta}+\utilde{\varepsilon}$

$$
\begin{align*}
    \text{with } \utilde{\theta}\in\Omega&\triangleq\set{D\utilde{\beta}:\utilde{\beta}\in\R^p}\\
    &=span\set{1, X_1, X_2, \cdots, X_k}
\end{align*}
$$

令 $r=\dim(\Omega)=rank(D)\le p$，i.e. $\Omega$ 是 $\R^n$ 向量空間下的 $r$ 維子空間。令 $\Omega\triangleq V_r$，$\R^n\triangleq V_n$

$\implies E[\utilde{Y}]=\utilde{\theta}\in\Omega$，$\utilde{\hat{Y}}=D\utilde{b}\in\Omega$，並且

$$
\begin{align*}
    & Q(\beta)=||\utilde{Y}-D\utilde{\beta}||^2=||\utilde{Y}-\utilde{\theta}||^2\\
    \implies & Q(\utilde{b})=\min_{\utilde{\beta}\in\R^p}\\
    \iff & ||\utilde{Y}-D\utilde{b}||^2=||\utilde{Y}-\utilde{\hat{Y}}||^2=\min_{\utilde{\beta}\in\R^p}||\utilde{Y}-D\utilde{\beta}||^2=||\utilde{Y}-\utilde{\theta}||^2
\end{align*}
$$

i.e. $\utilde{\hat{Y}}\in\Omega$ s.t. $||\utilde{Y}-\utilde{\hat{Y}}||^2=\min_{\utilde{\theta}\in\Omega}||\utilde{Y}-\utilde{\theta}||^2$

之前我們討論過 $\utilde{Y}$ 和 $\utilde{\hat{Y}}$ 的關係：

![alt text](img/1-1.png)

這裡 $\utilde{Y}\in V_n$，$\utilde{\hat{Y}}\in\Omega$。因此 $\utilde{\hat{Y}}$ 可以看作為 $\utilde{Y}$ 在 $\Omega$ 上的投影。而 $\utilde{e}$ 所在的空間垂直於 $\Omega$，記作 $V_r^\perp=\Omega^\perp$。

i.e. $\forall \utilde{Y}\in V_n=\R^n$， $\exist!\utilde{w}\in\Omega, z\in\Omega^\perp$ s.t. $\utilde{Y}=\utilde{w}+\utilde{z}$，並且 $\utilde{w}$ 是 $\utilde{Y}$ 在 $\Omega$ 上的投影

$\implies \utilde{Y}$ 在 $\Omega$ 上的投影 $\utilde{\hat{Y}}=\utilde{\hat{\theta}}$ 是唯一讓 $||\utilde{Y}-\utilde{\theta}||^2$ 最小的點。

:::tip[Lemma 3]
$V_r\subset V_n$ 是向量空間

$\utilde{Y}\in V_n, \utilde{w}$ 是 $\utilde{Y}$ 在 $V_r$ 上的投影

$\implies \utilde{w}_{n\times 1}=P_{n\times n}\utilde{Y}_{n\times 1}$，其中 $P$ 滿足以下特性：
1. 對稱（Symmetric）：$P^t=P$
2. 幂等（Idempotent）：$P^2=P$
3. $rank(P)=r$
:::

**Proof**: 令 $\utilde{\alpha}_1,\utilde{\alpha}_2,\cdots\utilde{\alpha}_r$ 是 $V_r$ 的一組 orthogonal basis。因為是基底互相正交且長度為 1

$$
\begin{align*}
    \implies \utilde{w}&=\sum_{i=1}^r(\utilde{Y}^t\utilde{\alpha}_i)\utilde{\alpha}_i\quad \text{e.g. } \begin{bmatrix*}
        1\\2\\3
    \end{bmatrix*}=1\cdot\begin{bmatrix*}
        1\\0\\0
    \end{bmatrix*}+2\cdot\begin{bmatrix*}
        0\\1\\0
    \end{bmatrix*}+3\cdot\begin{bmatrix*}
        0\\0\\1
    \end{bmatrix*}\\
    &=(\sum_{i=1}^r\utilde{\alpha}_i\utilde{\alpha}_i^t)\utilde{Y}\quad \text{note }\utilde{Y}^t\utilde{\alpha}_i=\utilde{\alpha}_i^t\utilde{Y} \text{ is a scalar}\\
    &= T\cdot T^t\utilde{Y}=P\utilde{Y} \quad \text{where } T_{n\times r}=\begin{bmatrix}
        \utilde{\alpha}_1 & \utilde{\alpha}_2 & \cdots & \utilde{\alpha}_r
    \end{bmatrix}, P_{n\times n}=T\cdot T^t
\end{align*}
$$

$$
\implies P^t=(TT^t)^t=TT^t=P \quad \text{Symmetric}
$$

因為 $T$ 是 orthogonal matrix，所以 $T^tT=I$，因此

$$
PP=(TT^t)(TT^t)=TT^tTT^t=TT^t=P \quad \text{Idempotent}
$$

並且

$$
rank(P)=rank(TT_t)=rank(T)=rank(T^t)=r
$$

因為

$$
rank(AB)\le min(rank(A), rank(B))
$$

$$
\implies rank(P)\le min(rank(T), rank(T^t))=r
$$

$$
r=rank(T)=rank(TI)=rank(TT^tT)=rank(PT)\le min(rank(P), rank(T))\le rank(P)
$$

i.e. $rank(P)=r$

:::tip[Lemma 4]
P: $n\times n$ 的對稱幂等矩陣（Symmetric Idempotent Matrix），並且 $rank(P)=r$。則：
1. $P$ 有 $r$ 個特徵值為 1，$n-r$ 個特徵值為 0
2. $\text{tr}(P)\triangleq\sum_{i=1}^n P_{ii}=r$
3. $I-P$ 也是對稱幂等矩陣，並且 $rank(I-P)=n-r=\text{tr}(I-P)$
4. $\forall \utilde{a}\in\R^n, \utilde{a}^tP\utilde{a}\ge 0$
:::

**Proof**:

因為 $P$ 是對稱的，存在一個 orthogonal matrix $A$ (i.e. $A^tA=I$) 使得 $A^tPA=diag(\lambda_1, \lambda_2,\cdots, \lambda_n)=B$，其中 $\lambda_i$ 是 $P$ 的特徵值。

$\implies B^2=BB=A^tPA\cdot A^tPA=A^tPA=B$, i.e. $\lambda^2_i=\lambda_i$, $i=1,2,\cdots, n$

$\implies \lambda_i=0$ or $1, \forall i$，但是

$$
\begin{align*}
    r=rank(P)&=Rank(A^tPA) \quad \because A \text{ is nonsingular}\\
    &=Rank(B)\\
\end{align*}
$$

$\implies$ $B$ 有 $r$ 個 1 和 $n-r$ 個 0。並且還有

$$
\text{tr}(P)=\text{tr}(PA^tA)=\text{tr}(A^tPA)=\text{tr}(B)=r
$$

---

記得如果 $\utilde{b}$ 是 $\utilde{\beta}$ 的 LSE $\iff D^t\utilde{Y}=D^tD\utilde{b}$

根據上面的結論，我們可以得到

$$
\begin{align*}
    \utilde{\hat{Y}}&=D\utilde{b}\xlongequal{\text{Lemma 3}}P\utilde{Y} \quad \text{with } P: n\times n \text{ symmetric idempotent matrix} \text{ and } rank(P)=rank(D)\le p\\
    &= \text{ projection of } \utilde{Y} \text{ onto } \Omega\quad \text{with } \Omega\triangleq span(D)
\end{align*}
$$

這裡我們假設未知的回歸係數數量 $p=k+1\le$ 樣本數 $n$

$$
\begin{align*}
    rank(D)=p&\iff D \text{ is full rank}\\
    &\iff \dim(\Omega)=p\\
    &\iff \text{ the columns of } D \text{ are linearly independent}\\
    &\iff D^tD \text{ is nonsingular}\\
    &\iff (D^tD)^{-1} \text{ exists}\\
    &\implies \text{normal equation has unique solution}
\end{align*}
$$

i.e. $\utilde{b}_{p\times 1}=(D^tD)^{-1}D^t\utilde{Y}$

$$
\begin{align*}
    \implies \utilde{\hat{Y}}&=D\utilde{b}\\
    &=D(D^tD)^{-1}D^t\utilde{Y}\\
    &=H\utilde{Y}\quad \text{where } H\triangleq D(D^tD)^{-1}D^t\quad\text{ hat matrix}
\end{align*}
$$

並且 $\utilde{e}=\utilde{Y}-\utilde{\hat{Y}}=\utilde{Y}-H\utilde{Y}=(I-H)\utilde{Y}\triangleq MY$ 其中 $M\triangleq I-H$ 稱為殘差矩陣（Residual Matrix）。

我們可以很容易的檢查 $H$ 和 $M$ 的性質：

$$
\begin{align*}
    & H=D(D^tD)^{-1}D^t\quad n\times n\\
    \implies &H^t=H\quad \text{Symmetric}\\
    &HH=H\quad \text{Idempotent}\\
    &\text{with }rank(H)=p=\dim(\Omega)\\ 
\end{align*}
$$

並且 $M=I-H$ 也是對稱幂等矩陣, $rank(M)=n-p$

**Note**：

H 是 $\Omega$ 上的投影矩陣，而 M 是 $\Omega^\perp$ 上的投影矩陣。並且 $\R^n=\Omega+\Omega^\perp$

如果我們把已經在 $\Omega$ 上的向量再投影到 $\Omega$ 上，那麼投影後的向量不會改變。而如果把 $\Omega$ 上的向量投影到垂直於 $\Omega$ 的空間上，那麼投影後的向量會是 0。i.e.

$$
H\utilde{\theta}=\utilde{\theta} \qquad M\utilde{\theta}=0 \quad \forall \utilde{\theta}\in\Omega
$$

因為 $D\in\Omega, \utilde{e}\in\Omega^\perp\implies D^t\utilde{e}=0$。我們可以衍生得到 $\utilde{1}^t\utilde{e}=0$，$\utilde{X}_j^t\utilde{e}=0$，$j=1,2,\cdots,k$。因為 $\utilde{1}$ 和 $\utilde{X}_j$ 都在 $\Omega$ 上。

上面說 $\utilde{\hat{Y}}$ 是在 $\Omega$ 上的投影，因此 $\utilde{\hat{Y}}^t\utilde{e}=0$。由此可以得到以下性質：

1. SSE 的性質：

   $$
   \begin{align*}
       & \begin{align*}
           \bullet\quad\text{SSE} &\triangleq \utilde{e}^t\utilde{e}=(MY)^t(MY)=\utilde{Y}^tM^tM\utilde{Y}\\
           &=\utilde{Y}^tM\utilde{Y} \quad \because M \text{ is symmetric idempotent}\\
           &=(D\utilde{\beta}+\utilde{\varepsilon})^tM(D\utilde{\beta}+\utilde{\varepsilon})\\
           &=\utilde{\beta}^tD^tMD\utilde{\beta}+2\utilde{\beta}^tD^tM\utilde{\varepsilon}+\utilde{\varepsilon}^tM\utilde{\varepsilon}\\
           &=\utilde{\varepsilon}^tM\utilde{\varepsilon} \quad \because D^tM=0
       \end{align*}\\

       & \begin{align*}
           \bullet\quad E[\text{SSE}]&=E[\utilde{\varepsilon}^tM\utilde{\varepsilon}]=\sum_{i=1}^n\sum_{j=1}^nE[\varepsilon_iM_{ij}\varepsilon_j]\\
           &=\sum_{i=1}^nM_{ij}E[\varepsilon_i^2] \quad \because E[\varepsilon_i\varepsilon_j]=0, i\ne j\\
           &=\sigma^2\text{tr}(M)=\sigma^2(n-p)
       \end{align*}\\
       & \bullet\quad E[\text{MSE}]=E[\frac{\text{SSE}}{n-p}]=\sigma^2
   \end{align*}
   $$

2. $\utilde{b}$ 的性質：

   $$
   \begin{align*}
       &\begin{align*}
          \bullet\quad E[\utilde{b}]&=E[(D^tD)^{-1}D^t\utilde{Y}]\\
          &=(D^tD)^{-1}D^t\cdot E[\utilde{Y}]\\
          &=(D^tD)^{-1}D^tD\utilde{\beta}\\
          &=\utilde{\beta}\quad \text{i.e. } \utilde{b} \text{ is unbiased for } \utilde{\beta}
      \end{align*}\\
      &\begin{align*}
          \bullet\quad \sigma^2\set{\utilde{b}}&=\sigma^2\set{(D^tD)^{-1}D^t\utilde{Y}}\\
          &=(D^tD)^{-1}D^t\sigma^2\set{\utilde{Y}}D(D^tD)^{-1}\\
          &=\sigma^2 (D^tD)^{-1}\underbrace{D^tD(D^tD)^{-1}}_{=I} \quad \sigma^2\set{\utilde{Y}}=\sigma^2I\\
          &=\sigma^2(D^tD)^{-1}
      \end{align*}\\
      &\bullet\quad S^2\set{\utilde{b}}=\text{MSE}(D^tD)^{-1} \quad \text{to est } \sigma^2\set{\utilde{b}}
   \end{align*}
   $$

3. $\utilde{\hat{Y}}$ 的性質：

   $$
   \begin{align*}
       &\begin{align*}
           \bullet\quad E[\utilde{\hat{Y}}]&=E[HY]=H\cdot E[\utilde{Y}]=HD\utilde{\beta}=D\utilde{\beta}=E[\utilde{Y}]\\
           \implies& \utilde{\hat{Y}} \text{ is unbiased for } E[\utilde{Y}]
       \end{align*}\\
       &\begin{align*}
           \bullet\quad \sigma^2\set{\utilde{\hat{Y}}}&=\sigma^2\set{HY}=H\sigma^2\set{\utilde{Y}}H^t\\
           &=\sigma^2HH^t=\sigma^2H
       \end{align*}\\
   \end{align*}
   $$

4. $\utilde{e}$ 的性質：
   
   $$
   \begin{align*}
        &\bullet\quad E[\utilde{e}]=E[M\utilde{Y}]=ME[\utilde{Y}]=\underbrace{MD}_{=0}\utilde{\beta}=0\\
        &\bullet\quad \sigma^2\set{\utilde{e}}=\sigma^2\set{MY}=\sigma^2M
   \end{align*}
   $$

5. $S^2\set{\ast}\triangleq\sigma^2\set{\ast}|_{\sigma^2=\text{MSE}}$ is unbiased for $\sigma^2\set{\ast}$

## ANOVA

$$
\begin{align*}
    &\begin{align*}
        \bullet\quad &\utilde{Y}=\utilde{\hat{Y}}+\utilde{e}\\
        &\begin{align*}
            \implies ||\utilde{Y}||^2&=||\utilde{\hat{Y}}||^2+||\utilde{e}||^2+2\utilde{\hat{Y}}^t\utilde{e}=||\utilde{\hat{Y}}||^2+||\utilde{e}||^2\\
            &=||\utilde{\hat{Y}}||^2 + \text{SSE}
        \end{align*}
    \end{align*}\\
    &\begin{align*}
        \bullet\quad &\utilde{Y}=\utilde{Y}-\bar{Y}\utilde{1}+\bar{Y}\utilde{1}\quad \bar{Y}\utilde{1}\in\Omega\\

        &\begin{align*}
            \implies ||\utilde{Y}||^2 &= ||\utilde{Y}-\bar{Y}\utilde{1}||^2 + \bar{Y}^2\cdot n\\
            &=\sum_{i=1}^n(Y_i-\bar{Y})^2+n\bar{Y}^2\\
            &=\text{SSTO}+n\bar{Y}^2
        \end{align*}
    \end{align*}\\
    &\begin{align*}
        \bullet\quad &\utilde{\hat{Y}}=\utilde{\hat{Y}}-\bar{Y}\utilde{1}+\bar{Y}\\
        &\begin{align*}
            \implies ||\utilde{\hat{Y}}||^2&=||\utilde{\hat{Y}}-\bar{Y}\utilde{1}||^2+\bar{Y}^2\cdot n\\
            &=\sum_{i=1}^n(\hat{Y}_i-\bar{Y})^2+n\bar{Y}^2\\
            &=\text{SSR}+n\bar{Y}^2
        \end{align*}
    \end{align*}\\
    &\implies ||\utilde{Y}||^2=||\utilde{\hat{Y}}||^2+||\utilde{e}||^2\\
    &\implies \text{SSTO}+\bcancel{n\bar{Y}^2}=\text{SSR}+\bcancel{n\bar{Y}^2}+\text{SSE}
\end{align*}
$$

Note:

$$
\begin{align*}
    ||\bar{Y}\utilde{1}||^2&=n\bar{Y}^2\\
    &=\bar{Y}\utilde{1}^t\utilde{1}\bar{Y}\\
    &=\frac{1}{n}\utilde{Y}^t\utilde{1}\utilde{1}^t\utilde{1}\frac{1}{n}\utilde{1}^t\utilde{Y}\\
    &=\utilde{Y}^t\frac{\utilde{1}\utilde{1}^t}{n}\utilde{Y} \quad \because\utilde{1}^t\utilde{1}=n\\
    &=\utilde{Y}^t\frac{J}{n}\utilde{Y}\quad \text{where } J=[1]_{n\times n}
\end{align*}
$$

- $(\frac{J}{n})^t=\frac{1}{n}J^t=\frac{1}{n}J$
- $\frac{J}{n}\cdot\frac{J}{n}=\frac{J}{n}$

$\implies \frac{J}{n}$ is $n\times n$ symmetric idempotent matrix with rank 1。i.e. 它是一个 $span\set{\utilde{1}}$ 上的投影矩阵。

$$
\begin{align*}
    &\begin{align*}
        \bullet\quad &\utilde{Y}^tI\utilde{Y}=\utilde{Y}^t\left(\frac{J}{n}+I-\frac{J}{n}\right)\utilde{Y}\\
        &\begin{align*}
            \iff ||\utilde{Y}||^2&=\utilde{Y}^t\frac{J}{n}\utilde{Y}+\utilde{Y}^t\left(I-\frac{J}{n}\right)\utilde{Y}\\
            &=n\bar{Y}^2+\text{SSTO}
        \end{align*}\\
        &\text{i.e. SSTO }\triangleq \sum_{i=1}^n(Y_i-\bar{Y})^2=\utilde{Y}^t\left(I-\frac{J}{n}\right)\utilde{Y}
    \end{align*}\\
    &\begin{align*}
        \bullet\quad \text{SSE } &=||\utilde{e}||^2=\utilde{e}^t\utilde{e}=(M\utilde{Y})^tM\utilde{Y}=\utilde{Y}^tM^tM\utilde{Y}\\
        &=\utilde{Y}^tM\utilde{Y}=\utilde{Y}^t(I-H)\utilde{Y}
    \end{align*}\\
    &\begin{align*}
        \bullet\quad \text{SSR } &= \sum_{i=1}^n(\hat{Y}_i-\bar{Y})^2=\text{SSTO}-\text{SSE}\\
        &=\utilde{Y}^t\left(I-\frac{J}{n}\right)\utilde{Y}-\utilde{Y}^t(I-H)\utilde{Y}\\
        &=\utilde{Y}^t(H-\frac{J}{n})\utilde{Y}
    \end{align*}
\end{align*}
$$

Note: 

$$
\begin{align*}
    I&=\underbrace{\frac{J}{n}}_{\in\Omega}+\underbrace{(I-\frac{J}{n})}_{\in\Omega^\perp}\\
    &=\underbrace{H}_{\in\Omega}+\underbrace{(I-H)}_{\in\Omega^\perp}\\
    &=\underbrace{(H-\frac{J}{n})}_{\in\Omega}+\underbrace{I-H+\frac{J}{n}}_{\in\Omega^\perp}
\end{align*}\qquad\text{and}\qquad
\begin{align*}
    &\text{SSTO}=\utilde{Y}^t\left(I-\frac{J}{n}\right)\utilde{Y}\\
    &\text{SSE}=\utilde{Y}^t(I-H)\utilde{Y}\\
    &\text{SSR}=\utilde{Y}^t(H-\frac{J}{n})\utilde{Y}
\end{align*}
$$

:::tip[Corollary]
$I=P_1+P_2+\cdots+P_m$, where $P_i$ is projection matrix then $||\utilde{Y}||^2=\sum_{i=1}^m\utilde{Y}^tP_i\utilde{Y}$
:::

從上面的推理我們可以知道 SSTO, SSE, SSR 都是 quadratic form。$\utilde{Y}^tP\utilde{Y}$，其中 $P$ 是 projection matrix ($n\times n$ symmetric idempotent matrix)。

## Gauss-Markov Theorem

:::info[Definition]
Let $\xi=\utilde{c}^t\utilde{\beta}$ with $\utilde{c}\in\R^p$

$\xi$ is estimable if $\exist \utilde{a}\in\R^n$ s.t. $E[\utilde{a}^t\utilde{Y}]=\xi$, i.e. $\utilde{a}^t\utilde{Y}$ is unbiased for $\xi$
:::

:::tip[Theorem]
**Gauss-Markov Theorem**:

Suppose # of observations $n\ge p$ # of parameters

$$
\text{If }\utilde{Y}=D\utilde{\beta}+\utilde{\varepsilon}\text{ with } E[\utilde{\varepsilon}]=0, \sigma^2\set{\utilde{\varepsilon}}=\sigma^2I_{n\times n}
$$

Let $\utilde{c}\in\R^p$ s.t. $\xi =\utilde{c}^t\utilde{\beta}$ is estimable and $\hat{\xi}=\utilde{c}^t\utilde{b}$ where $\utilde{b}$ is LSE of $\utilde{\beta}$

Then

1. $\hat{\xi}$ is linear unbiased estimator for $\xi$
2. $\sigma^2\set{\hat{\xi}}\le\sigma^2\set{\tilde{\xi}}, \forall \tilde{\xi}$ is linear unbiased estimator for $\xi$

i.e. $\hat{\xi}$ is BLUE (Best Linear Unbiased Estimator) for $\xi$
:::

**Proof**:

因為我們對於所有 $\utilde{\beta}$ 都假設 $\xi=\utilde{c}^t\utilde{\beta}$ 是 estimable，其中 $\utilde{c}\in\R^p$。i.e.

$$
\begin{align*}
    &\forall\beta\in\R^p, \exist \utilde{a}\in\R^n \text{ s.t. } E[\utilde{a}^t\utilde{Y}]=\xi\\
    \iff& \utilde{a}^tD\utilde{\beta}=\utilde{c}^t\utilde{\beta},\forall\utilde{\beta}\in\R^p\\
    \iff& \left(\utilde{a}^tD-\utilde{c}^t\right)\utilde{\beta}=0,\forall\utilde{\beta}\in\R^p\\
    \iff& \utilde{a}^tD-\utilde{c}^t=0\quad\because \forall\utilde{\beta}\in\R^p\\
    \iff& \utilde{a}^tD=\utilde{c}^t
\end{align*}
$$

Let $\Omega=$ column space of $D$，which is a sub-vector space of $\R^n$.

$\implies \forall\utilde{a}\in\R^n,\exist!\utilde{d}\in\Omega,\utilde{\omega}\in\Omega^\perp$ s.t. $\utilde{a}=\utilde{d}+\utilde{\omega}$. 對於每個 $\utilde{a}$，我們都可以找到相應的唯一分解。

$$
\begin{align*}
    \implies E[\utilde{a}^t\utilde{Y}]&=E[(\utilde{d}+\utilde{\omega})^t\utilde{Y}]=E[\utilde{d}^t\utilde{Y}] + E[\utilde{\omega}^t\utilde{Y}]\\
    &=\utilde{d}^tD\utilde{\beta}+\underbrace{\utilde{\omega}^tD\utilde{\beta}}_{=0}\quad \because \utilde{\omega}\in\Omega^\perp\text{ and } D\in\Omega\\
    &=\utilde{d}^tD\utilde{\beta}
\end{align*}
$$

而 $E[\utilde{a}^t\utilde{Y}]=\xi=\utilde{c}^t\utilde{\beta},\forall\utilde{\beta}\in\R^p\implies\utilde{d}^t\utilde{Y}$ is also unbiased for $\xi$ and $\utilde{d}^tD=\utilde{c}^t$

**Claim**: $\forall\utilde{\beta}\in\R^p,\utilde{d}$ is the only vector in $\Omega$ s.t. $E[\utilde{d}^t\utilde{Y}]=\xi$

> **Proof**: Let $\utilde{\alpha}\in\Omega$ s.t. $E[\utilde{\alpha}^t\utilde{Y}]=\xi=\utilde{c}^t\utilde{\beta},\forall\utilde{\beta}\in\R^p$
> $$
> \begin{align*}
>    &\implies E[(\utilde{\alpha}-\utilde{d})^t\utilde{Y}]=0\\
>    &\implies E[(\utilde{\alpha}-\utilde{d})^tD\utilde{\beta}]=0\quad \forall\utilde{\beta}\in\R^p\\
>    &\implies (\utilde{\alpha}-\utilde{d})^tD=0\quad \forall\utilde{\beta}\in\R^p\\
>    &\implies \utilde{\alpha}-\utilde{d}\in\Omega^\perp\text{, but }\utilde{\alpha} \text{ and } \utilde{d} \text{ are both in } \Omega\\
>    &\implies \utilde{\alpha}-\utilde{d}\in \Omega\cap\Omega^\perp=\empty\\
>    &\implies \utilde{\alpha}=\utilde{d}
> \end{align*}
> $$

i.e. 即使能為 $\xi$ 找到多個線性無偏估計，但他們在 $\Omega$ 上的投影是相同的。

令他們共同的投影為 $\utilde{d}$，則我們有 $\utilde{a}^tD=\utilde{c}^t, \utilde{d}^tD=\utilde{c}^t$ 並且 $E[\utilde{d}^t\utilde{Y}]=\xi=\utilde{c}^t\utilde{\beta}, \forall\utilde{\beta}\in\R^p$

Note $\utilde{d} \in \Omega$ and $\utilde{Y}-D\utilde{b}=\utilde{Y}-\utilde{\hat{Y}}=\utilde{e} \in\Omega^\perp\implies \utilde{d}^t\utilde{e}=0$

$\implies \utilde{d}^t(\utilde{Y}-D\utilde{b})=\utilde{d}^t\utilde{Y}-\utilde{d}^tD\utilde{b}=\utilde{d}^t\utilde{Y}-\utilde{c}^t\utilde{b}=0$

i.e. $\hat{\xi}\triangleq\utilde{c}^t\utilde{b}=\utilde{d}^t\utilde{Y}$

i.e. $\hat{\xi}$ is linear estimator for $\xi$

Now let $\tilde{\xi}$ be any linear unbiased estimator for $\xi$

i.e. $\tilde{\xi}=\utilde{a}^t\utilde{Y}$ for some $\utilde{a}\in\R^n$ s.t. $E[\utilde{a}^t\utilde{Y}]=\xi=\utilde{c}^t\utilde{\beta}, \forall\beta\in\R^p$

Let $\utilde{d}$ be $\utilde{a}$'s projection onto $\Omega$

$$
\begin{align*}
    \sigma^2\set{\tilde{\xi}}&=\sigma^2\set{\utilde{a}^t\utilde{Y}}=\utilde{a}^t\sigma^2\set{\utilde{Y}}\utilde{a}=\sigma^2||\utilde{a}||^2=\sigma^2||\utilde{d}+\utilde{\omega}||^2\\
    &\ge \sigma||\utilde{d}||^2=\utilde{d}^t\sigma^2I\utilde{d}=\sigma^2\set{\utilde{d}^t\utilde{Y}}=\sigma^2\set{\hat{\xi}}
\end{align*}
$$

i.e. $\sigma^2\set{\tilde{\xi}\ge}\sigma^2\set{\hat{\xi}},\forall \tilde{\xi}$

---

**Remark**:

1. *Gauss-Markov Theorem* 不需要分配的假設，並且以上證明對於 $D$ 不是 full rank 也成立 
2. 對於 $\xi=\utilde{c}^t\utilde{\beta}$ 是 estimable
   
   i.e. $\exist\utilde{a}\in\R^n$ s.t $E[\utilde{a}^t\utilde{Y}]=\xi=\utilde{c}^t\utilde{\beta},\forall\utilde{\beta}\in\R^p$

   $\implies\utilde{a}^tD=\utilde{c}^t$

   i.e. $\utilde{c}^t$ 是由 D 的 row vectors 線性組合而成的

   i.e. $\utilde{c}^t\in$ row space of D

   **i.e. $\utilde{c}^t\utilde{\beta}$ is estimable $\iff\utilde{c}^t\in$ row space of D**

   對於某些設計矩陣 D，$\utilde{c}^t\utilde{\beta}$ 可能不是 estimable
   > $$
   >  \text{e.g. }D=\begin{bmatrix*}
   >      1&0\\
   >      \vdots&\vdots\\
   >      1&0
   >  \end{bmatrix*}\quad \text{row space}=\set{(c,0);c\in\R}
   > $$
   > 
   > $\iff$ estimable par is $c\beta_0,\forall c\in\R$ and $\beta_1$ is not estimable

   If row space of $D=\R^p$, i.e. D is full rank, or $(D^tD)^{-1}$ exists $\iff\forall\utilde{c}\in\R^p, \utilde{c}^t\utilde{\beta}$ is estimable

3. 如果 $\text{rank}(D)=p$(i.e. $(D^tD)^{-1}$ exists), 那麼 *Gauss-Markov Theorem* 有一個更簡單的證明：
   $$
    \begin{align*}
        &\sigma^2\set{\tilde{\xi}}=\sigma^2\utilde{a}^tI\utilde{a}\\
        &\begin{align*}
            \sigma^2\set{\hat{\xi}}&=\sigma^2\set{\utilde{c}^t\utilde{b}}=\sigma^2\set{\utilde{c}^t(D^tD)^{-1}D^t\utilde{Y}}\\
            &=\utilde{c}^t(D^tD)^{-1}D^t\sigma^2\set{\utilde{Y}}D(D^tD)^{-1}\utilde{c}\\
            &=\sigma^2\utilde{c}^t(D^tD)^{-1}D^tD(D^tD)^{-1}\utilde{c}\\
            &=\sigma^2\utilde{c}^t(D^tD)^{-1}\utilde{c}\\
            &=\sigma^2\utilde{a}^tD(D^tD)^{-1}D^t\utilde{a}\quad \because \utilde{c}^t=\utilde{a}^tD\\
            &=\sigma^2\utilde{a}^tH\utilde{a}
        \end{align*}\\
        &\sigma^2\set{\tilde{\xi}}-\sigma^2\set{\hat{\xi}}=\sigma^2\utilde{a}^t(I-H)\utilde{a}=\sigma^2\utilde{a}^tM\utilde{a}\ge 0\quad\because M \text{ is Positive definite}
    \end{align*}
   $$

## Multivariate Normal

$Z_1,Z_2,\cdots, Z_n\overset{\text{iid}}{\sim}N(0,1)$

- Joint pdf $f(\utilde{z})=\left(\frac{1}{\sqrt{2\pi}}\right)^ne^{-\sum_{i=1}^n z^2_i /2}=\left(\frac{1}{\sqrt{2\pi}}\right)^ne^{-||z||^2/2},\utilde{z}\in\R^n$ where $||\utilde{z}||^2=\sum_{i=1}^nz^2_i\sim\chi^2_n$
- $E[\utilde{Z}]=0, \sigma^2\set{\utilde{Z}}=I$
- $\utilde{Z}\sim N_n(0, I)$

Let $A_{m\times n}, \utilde{\eta}_{m\times 1}$ be non-random matrix and $\utilde{W}_{m\times 1}=A\utilde{Z}_{n\times 1}$+ $\utilde{\eta}$

$\implies E[\utilde{W}]=AE[\utilde{Z}]+\utilde{\eta}=\utilde{\eta}, \sigma^2\set{\utilde{W}}=AA^t$ uniquely determine the distribution of $\utilde{W}$

$$
\utilde{W}\sim N_m(E[\utilde{W}], \sigma^2\set{\utilde{W}})=N_m(\utilde{\eta}, \bcancel\Sigma_{\utilde{w}}) \quad \text{where } \bcancel\Sigma_{\utilde{w}}=AA^t
$$

If $\bcancel{\Sigma}_{\utilde{W}}^{-1}$ exist, say that $\bcancel{\Sigma}_{\utilde{W}}$ is nonsingular, then pdf of $\utilde{W}$ is

$$
f_{\utilde{W}}(\utilde{w})=\left(\frac{1}{\sqrt{2\pi}}\right)^m|\bcancel{\Sigma}_{\utilde{W}}|^{\frac{-1}{2}}e^{-\frac{Q}{2}}\quad \text{where } |\bcancel{\Sigma}_{\utilde{W}}|=\det(\bcancel{\Sigma}_{\utilde{W}}), Q=(\utilde{w}-\utilde{\eta})^t\bcancel{\Sigma}_{\utilde{W}}^{-1}(\utilde{w}-\utilde{\eta})
$$

---

$$
f_{\utilde{W}}(\utilde{w})=\prod_{i=1}^m\frac{1}{\sqrt{2\pi}}\frac{1}{\sigma_i}\exp\set{\frac{-(w_i-\eta_i)^2}{2\sigma_i^2}}
$$

$$
\begin{align*}
    &\iff \bcancel{\Sigma}_{\utilde{w}}=\text{diag}(\sigma_1^2,\sigma_2^2,\cdots,\sigma_m^2)\\
    &\iff \sigma\set{w_i,w_j}=\text{cov}(w_i,w_j)=0\quad \forall i\ne j\\
    &\iff w_1,w_2,\cdots,w_m \text{ are independent}
\end{align*}
$$

:::tip[Lemma 6]
$\utilde{W}\sim N_m(\utilde{\eta}, \bcancel{\Sigma}_{\utilde{W}})$ where $\bcancel{\Sigma}_{\utilde{W}}$ is nonsingular

$\implies Q=(\utilde{W}-\utilde{\eta})^t\bcancel{\Sigma}_{\utilde{W}}(\utilde{W}-\utilde{\eta})\sim \chi^2_m$
:::

**Proof**:

- Way 1: $\bcancel{\Sigma}_{\utilde{W}}$ is sym non-singular matrix, $\exist B$: non-singular s.t. $B\bcancel{\Sigma}_{\utilde{W}}B^t=I$

  $\implies \bcancel{\Sigma}_{\utilde{W}}=(B^tB)^{-1}$, i,e, $\bcancel{\Sigma}_{\utilde{W}}^{-1}=B^tB$

  $B(\utilde{W}-\utilde{\eta})\sim N_m(\utilde{0}, B\sigma^2{\utilde{W}}B^t)=N_m(\utilde{0}, I)$

  $\implies (B(\utilde{W}-\utilde{\eta}))^t*(B(\utilde{W}-\utilde{\eta}))\sim\chi^2_m$

  $\implies(\utilde{W}-\utilde{\eta})^t\bcancel{\Sigma}_{\utilde{W}}B(\utilde{W}-\utilde{\eta})\sim\chi^2_m$