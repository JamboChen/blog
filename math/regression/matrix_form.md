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

# 矩陣形式下的基礎定義和結論


:::tip[Definition]
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
\end{bmatrix}: \text{ random matrix}\\
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
    \implies \sigma^2\set{W} &=E\left[\left(W-E[W]\right)_{m\times 1}\left(W-E[W]\right)_{1\times m} ^T\right]\\
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
3. $\sigma^2\set{W}=E(ww^T)-E[W](E[W])^T$
4. $\sigma^2\set{A_{n\times w}W_{w\times 1}}_{n\times n} = A_{n\times w}\sigma^2\set{W}_{w\times w}A^T_{w\times n}$
5. $\sigma^2\set{AW+B}=\sigma^2\set{AW}$

**Note**: 如果 $W$ 是 $m\times 1$ 隨機向量，則 $\sigma^2\set{W}$ 是 $m\times m$ 的對稱矩陣。並且 $\forall \utilde{a}\in\R^n$ 是常數向量
$$
\utilde{a}^T\sigma^2\set{W}\utilde{a}=\sigma^2\set{\utilde{a}^TW}\ge 0
$$

因此 $\utilde{a}^T\sigma^2\set{W}\utilde{a}$ 是半正定矩陣(Positive Semi-Definite)。而等於號僅在 $\utilde{a}=0$ 時成立。

:::tip[Definition]
令 $f:\R^k\to\R$，並且 $f(\utilde{\theta})\in\R, \forall\theta=(\theta_1, \cdots, \theta_k)^T\in\R^k$

$$
\frac{\partial}{\partial\utilde{\theta}} f(\utilde{\theta}) \triangleq \begin{bmatrix}
    \frac{\partial f}{\partial\theta_1}\\
    \frac{\partial f}{\partial\theta_2}\\
    \vdots\\
    \frac{\partial f}{\partial\theta_k}
\end{bmatrix}
$$
:::

:::tip[Lemmma]
Given $\utilde{c}\in\R^k$, $f(\utilde{\theta})=\utilde{c}^T\utilde{\theta}=\utilde{\theta}^T\utilde{c}$, $\forall \utilde{\theta}\in\R^k$
$$
\frac{\partial}{\partial\utilde{\theta}}f(\utilde{\theta})=\utilde{c}
$$
i.e. $\frac{\partial}{\partial\utilde{\theta}}(\utilde{c}^T\utilde{\theta})=\frac{\partial}{\partial\utilde{\theta}}(\utilde{\theta}^T\utilde{c})=\utilde{c}$
:::

:::tip[Lemmma]
如果 $A$ 是 $k\times k$ 的對稱常數矩陣
$$
f(\utilde{\theta})=\utilde{\theta}^TA\utilde{\theta}=\sum_{i,j}\theta_iA_{ij}\theta_j
$$

$$
\implies \frac{\partial}{\partial\utilde{\theta}}f(\utilde{\theta})=2A\utilde{\theta}
$$

如果 $A$ 不一定對稱，則 $\frac{\partial}{\partial\utilde{\theta}}f(\utilde{\theta})=A\utilde{\theta}+A^T\utilde{\theta}$
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

:::tip[Definition]
$$
\begin{align*}
    Q(\utilde{\beta})\triangleq& ||\utilde{Y}-E[\utilde{Y}]||^2\\
    &=||\utilde{Y}-D\utilde{\beta}||^2\\
    &=(\utilde{Y}-D\utilde{\beta})^T(\utilde{Y}-D\utilde{\beta})\\
\end{align*}
$$

如果 $Q(\utilde{b})=\min_{\utilde{\beta}\in\R^n}$，則 $\utilde{b}$ 是 $\utilde{\beta}$ 的 LSE。
:::

注意到

$$
\begin{align*}
    Q(\utilde{\beta})&=\utilde{Y}^T_{1\times n}\utilde{Y}_{n\times 1}-\utilde{Y}^T_{1\times n}D_{n\times p}\utilde{\beta}_{p\times 1}-\utilde{\beta}^T_{1\times p}D^T_{p\times n}\utilde{Y}_{n\times 1}+\utilde{\beta}^T_{1\times p}D^T_{p\times n}D_{n\times p}\utilde{\beta}_{p\times 1}\\
    &=\utilde{Y}^T\utilde{Y}-2\utilde{\beta}^TD^T\utilde{Y}+\utilde{\beta}^TD^TD\utilde{\beta}\\
\end{align*}

$$