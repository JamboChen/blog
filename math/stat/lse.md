# 最小平方估計（Least Square Estimation，LSE）

Let $X_i$ with $E_\theta[X_i]$ exists

$\implies Q(\theta)\triangleq\sum_{i=1}^n(X_i-E_\theta[X_i])^2\xlongequal{\text{Vec}}||\utilde{X}-E_\theta[\utilde{X}]||^2$

LSE 則是找到那個 $\theta$ 使得 $Q(\theta)$ 最小，i.e. $\hat{\theta}=\min_{\theta\in\Omega} Q(\theta)$

**EX**: $X_1, \cdots, X_n \stackrel{\text{iid}}{\sim} P(\theta)$, i.e. $E[X_i]=\theta$

$\implies Q(\theta)=\sum_{i=1}^n(X_i-\theta)^2\implies \frac{d}{d\theta}Q(\theta)=-2\sum_{i=1}^n(X_i-\theta)\stackrel{\text{set}}{<} 0$

$\iff \sum X_i>n\theta\iff \hat{\theta}_{LSE}=\bar{X}$

---

**EX**: $i=1,2,\cdots, n \quad E_\theta[X_i]=\beta_0+\beta_1t_i,t_i\in\R$, given and $\theta=(\beta_0, \beta_1)\in\Omega=\R^2$

$\implies Q(\theta)=Q(\beta_0, \beta_1)=\sum_{i=1}^n(X_i-E_\theta(X_i))^2=\sum_{i=1}^n(X_i-\beta_0-\beta_1t_i)^2$

$$
\begin{align*}
    &\implies \text{set} 
    \begin{cases}
        \frac{\partial}{\partial\beta_0}Q(\theta)=-2\sum_{i=1}^n(X_i-\beta_0-\beta_1t_i)=0\\
        \frac{\partial}{\partial\beta_1}Q(\theta)=-2\sum_{i=1}^n(X_i-\beta_0-\beta_1t_i)t_i=0
    \end{cases}\\
    &\iff
    \begin{cases}
        \sum X_i=n\beta_0+\beta\sum t_i\\
        \sum X_it_i=\beta_0\sum t_i+\beta_1\sum t_i^2
    \end{cases}\\
    &\iff
    \left\{
        \begin{align}
            &\bar{X}=\beta_0+\beta_1\bar{t}\\
            &\frac{1}{n}\sum X_it_i=\beta_0\bar{t}+\beta_1\frac{1}{n}\sum t_i^2
        \end{align}
    \right.\\
\end{align*}
$$

$(2)-(1)\times \bar{t}\implies \frac{1}{n}\sum X_it_i-\bar{X}\bar{t}=\beta_1(\sum t_i^2-\bar{t})$

$$
\begin{align*}
    \implies& \hat{\beta}_1=\frac{\frac{1}{n}\sum X_it_i-\bar{X}\bar{t}}{\frac{1}{n}\sum t_i^2-\bar{t}^2}=
\frac{\sum X_it_i-n\bar{X}\bar{t}}{\sum t_i^2-n\bar{t}^2}=\frac{\sum(X_i-\bar{X})(t_i-\bar{t})}{\sum(t_i-\bar{t})^2} \\

    \xRightarrow{\text{代入(1)}}& \hat{\beta}_0=\bar{X}-\hat{\beta}_1\bar{t}
\end{align*}
$$

因為 $Q(\theta)$ 是係數為正的二次函數，所以一次微分等於 0 的點一定是最小值。以上的推導沒有對分佈做任何假設。

如果進一步假設 $X_i\sim N(\beta_0+\beta_1t_i, \sigma^2)$ indep

$$
\begin{align*}
    L(\beta_0, \beta_1 | \utilde{x}) &= (\frac{1}{\sqrt{2\pi}\sigma})^n\exp\set{-\frac{1}{2\sigma^2}\sum_{i=1}^n(x_i-\beta_0-\beta_1t_i)^2}\\
    &= (\frac{1}{\sqrt{2\pi}\sigma})^n\exp\set{-\frac{1}{2\sigma^2}Q(\beta_0, \beta_1)}
\end{align*}
$$

i.e. $Q(\beta_0, \beta_1)\searrow \implies L(\beta_0, \beta_1 | \utilde{x})\nearrow$， 也就是說這裡的 LSE 也是 MLE。