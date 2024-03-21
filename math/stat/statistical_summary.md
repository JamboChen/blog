# 統計結論

:::info
- $\bar{X}=\frac{1}{n}\sum_{i=1}^n X_i$
- $S^2=\frac{1}{n-1}\sum_{i=1}^n (X_i-\bar{X})^2$
- $S^2_*=\frac{1}{n}\sum_{i=1}^n (X_i-\bar{X})^2$
:::

## Binomial Distribution

$X_1,\ldots,X_n\stackrel{\text{iid}}{\sim} B(1,p), p\in\Omega=(0,1)$ with
$$
f(x;p)=p^x(1-p)^{1-x}, x\in\set{0,1}
$$

## Poisson Distribution

$X_1,\ldots,X_n\stackrel{\text{iid}}{\sim} P(\lambda), \lambda\in\Omega=(0,\infty)$ with
$$
f(x;\lambda)=\frac{e^{-\lambda}\lambda^x}{x!}, x\in\set{0,1,\ldots}
$$

| Type       | Statistic | $\theta$  |
| ---------- | --------- | --------- |
| Sufficient | $\bar{X}$ | $\lambda$ |

## Normal Distribution

$X_1,\ldots,X_n\stackrel{\text{iid}}{\sim} N(\mu,\sigma^2), (\mu, \sigma)\in\Omega=\R\times (0,\infty)$ with
$$
f(x;\mu,\sigma^2)=\frac{1}{\sqrt{2\pi\sigma^2}}e^{-\frac{(x-\mu)^2}{2\sigma^2}}, x\in\R
$$

| Type       | Statistic                   | $\theta$        |
| ---------- | --------------------------- | --------------- |
| Sufficient | $(S^2, \bar{X})$            | $(\mu, \sigma)$ |
| Sufficient | $\bar{X}$                   | $\mu$           |
| Sufficient | $\sum_{i=1}^n(X_i-\mu_0)^2$ | $\sigma^2$      |

### MLE

| Estimator | $\theta$   |
| --------- | ---------- |
| $\bar{X}$ | $\mu$      |
| $S^2_*$   | $\sigma^2$ |

## Uniform Distribution

$X_1,\ldots,X_n\stackrel{\text{iid}}{\sim} U(\alpha,\beta)$

| Type       | Statistic            | $\theta$          |
| ---------- | -------------------- | ----------------- |
| Sufficient | $X_{(1)}$            | $\alpha$          |
| Sufficient | $X_{(n)}$            | $\beta$           |
| Sufficient | $(X_{(1)}, X_{(n)})$ | $(\alpha, \beta)$ |