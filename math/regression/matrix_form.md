# 矩陣形式的迴歸模型

當我們有 $n$ 筆數據，並且有 $k$ 個自變數時，我們有以下的迴歸模型：

$$
\begin{align*}
    Y_1=&\beta_0+\beta_1X_{11}+\beta_2X_{12}+\cdots+\beta_kX_{1k}+\epsilon_i\\
    Y_2=&\beta_0+\beta_1X_{21}+\beta_2X_{22}+\cdots+\beta_kX_{2k}+\epsilon_2\\
    &\vdots\\
    Y_n=&\beta_0+\beta_1X_{n1}+\beta_2X_{n2}+\cdots+\beta_kX_{nk}+\epsilon_n
\end{align*}
$$

我們可以將這個模型寫成矩陣形式：

$$
\utilde{Y}=\underbrace{\begin{bmatrix}
    1 & X_{11} & X_{12} & \cdots & X_{1k}\\
    1 & X_{21} & X_{22} & \cdots & X_{2k}\\
    \vdots & \vdots & \vdots & \ddots & \vdots\\
    1 & X_{n1} & X_{n2} & \cdots & X_{nk}
\end{bmatrix}}_{\text{Design Matrix}}\utilde{\beta}+\utilde{\epsilon}
$$

因為 $\utilde{X}$ 是由我們提供，所以我們將與 $\utilde{\beta}$ 有關的部分稱為設計矩陣（Design Matrix），記作 $D$。

$$
\implies \utilde{Y}_{n\times 1}=D_{n\times p}\utilde{\beta}_{p\times 1}+\utilde{\epsilon}_{n\times 1}
$$