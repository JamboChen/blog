# 點估計（Point Estimation）

我們已經有 $n$ 個數據 $\utilde{X}=(X_1,\ldots,X_n)\stackrel{\text{iid}}{\sim} f(\utilde{x};\theta)$ with $\theta\in\Omega=\R^r$ 是未知的。假設我們對 $\eta(\theta)\in\R$ 感興趣。我們希望能夠用 $\utilde{X}$ 來估計 $\eta(\theta)$。

:::info[Definition]
Any function $\delta :\R^n\to\R$ is called a point estimator of $\eta(\theta)$.

i.e. $\delta(\utilde{X})=\hat{\eta(\theta)}$ 是一種估計 $\eta(\theta)$ 的**方法**。
:::

**EX**: Data from $N(\mu, \sigma^2)$ with $\theta=(\mu, \sigma^2)\in\Omega=\R\times(0,\infty), r=2$，有兩個參數。

1. $\eta(\theta)=\mu$
   
   $\hat{\mu}=X_1$ or $\bar{X}$ or $X_{(n)}$ 都是估計 $\mu$ 的方法。

2. $\eta(\theta)=\sigma^2$
   
   $\hat{\sigma^2}=S=\sqrt{\frac{1}{n-1}\sum_{i=1}^n(X_i-\bar{X})^2}$ or $S_*=\sqrt{\frac{1}{n}\sum_{i=1}^n(X_i-\bar{X})^2}$ 都是估計 $\sigma^2$ 的方法。

:::note
已經觀察到數據 $\utilde{X}=\utilde{x}=(x_1,\ldots,x_n)$

$\implies \delta(\utilde{X})\mid_{\utilde{X}=\utilde{x}}$ 是 $\eta(\theta)$ 的估計值。
:::

> Q: 如何從眾多的估計方法中選擇一個好的估計方法？
> 
> i.e. 如何衡量一個估計方法的好壞？

我們可以用誤差的平方來衡量一個估計方法的好壞i.e. $(\delta(\utilde{X})-\eta(\theta))^2$.

$\implies$ 最佳的估計方法，是能夠在所有可能的数据情况下，平均的误差平方最小的方法。

$\implies E[(\delta(\utilde{X})-\eta(\theta))^2]$


:::info[Definition]
$MSE(\delta(\utilde{X}), \eta(\theta))\triangleq E_\theta[(\delta(\utilde{X})-\eta(\theta))^2]$ is function of $\theta$.
:::

$$
\begin{align*}
   MSE(\delta(\utilde{X}), \eta(\theta))&=E_\theta[(\delta(\utilde{X})-\eta(\theta))^2]\\
   &=E_\theta[(\delta(\utilde{X})-E_\theta[\delta(\utilde{X})]+E_\theta[\delta(\utilde{X})]-\eta(\theta))^2]\\
   &=E_\theta[(\delta(\utilde{X})-E_\theta[\delta(\utilde{X})])^2]+E_\theta[(E_\theta[\delta(\utilde{X})]-\eta(\theta))^2]+2E_\theta[(\delta(\utilde{X})-E_\theta[\delta(\utilde{X})])(E_\theta[\delta(\utilde{X})]-\eta(\theta))]\\
   &\text{Since } E_\theta[\delta(\utilde{X})]=E_\theta(E_\theta[\delta(\utilde{X})]) \implies 2E_\theta[\delta(\utilde{X})-E_\theta[\delta(\utilde{X})]]=0\\
   &=E_\theta[(\delta(\utilde{X})-E_\theta[\delta(\utilde{X})])^2]+E_\theta[(E_\theta[\delta(\utilde{X})]-\eta(\theta))^2]\\
   &=Var_\theta(\delta(\utilde{X}))+[E_\theta[\delta(\utilde{X})]-\eta(\theta)]^2
\end{align*}
$$

:::info[Definition]
1. $E_\theta[\delta(\utilde{X})]-\eta(\theta)\triangleq Bias(\delta(\utilde{X}),\eta(\theta))$
2. $E_\theta\delta(\utilde{X})=\eta(\theta) \implies \delta(\utilde{X})$ is *unbiased* for $\eta(\theta)$
:::

:::note
$MES(\delta(\utilde{X}), \eta(\theta)) = Var_\theta(\utilde{X})+[Bias(\utilde{X}, \eta(\theta))]^2$
:::

我們希望針對 $\eta(\theta)$ 找到一個最好的估計方法 $\delta^*$ ，使得在任何可能的參數值下，它的 $MSE$ 比其他任何估計方法都要小，i.e. $MSE(\delta^*, \eta(\theta))\le MSE(\delta, \eta(\theta)), \forall \theta\in\Omega, \forall \delta$

**但這是不可能的**

我們可以定義白目估計量 $\delta_c(\utilde{X})=c, \forall\utilde{X}$ with $c=\eta(\theta_c)$ is a constant.

$$
\begin{align*}
   \implies MSE(\delta_c, \eta(\theta)) &= Var_\theta(\delta_c)+[Bias(\delta_c, \eta(\theta))]^2\\
   &=0+[c-\eta(\theta)]^2\\
   &=
   \begin{cases}
      0 & \text{if } c=\eta(\theta)\\
      (c-\eta(\theta))^2 & \text{if } c\neq\eta(\theta)
   \end{cases}

\end{align*}
$$

如果最佳的估計方法 $\delta^*$ 存在
$$
\begin{align*}
   \implies& MSE(\delta^*, \eta(\theta))\le MSE(\delta_c, \eta(\theta))\\
   \implies& MSE(\delta^*, \eta(\theta_c))=0, \forall \theta_c\\
   \implies& MSE(\delta^*, \eta(\theta))=0, \forall \theta
\end{align*}
$$

是不可能的。因此我们无法通过 $MSE$ 来得到最佳的估计方法，我们需要做一些取舍（限制）。