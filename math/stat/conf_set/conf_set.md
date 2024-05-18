# 信賴集合估計（Confidence Sets Estimation）

我們有 $n$ 個數據 $\utilde{X}=(X_1, \cdots, X_n)\sim f(\utilde{x};\theta)$  with $\theta\in\Omega\subset\R^r, r\ge 1$，並且我們對 $\eta(\theta):\Omega\to\R^m,m\le r$ (通常 $m=1$) 感興趣。

e.g. $N(\mu, \sigma^2), \theta=(\mu, \sigma^2)$

$$
\begin{alignat*}{3}
    \implies &\eta(\theta)=\mu&\qquad&\eta(\theta)=\sigma &\qquad& \eta(\theta)=\log|\mu|\\
    &\eta(\theta)=\sigma^2&\qquad&\eta(\theta)=\frac{\mu}{\sigma} &\qquad& \cdots
\end{alignat*}
$$

在數據 $\utilde{X}$ (r.v) 下，$\eta(\theta)$ 的集合估計是指在 $\R^m(=\eta(\Omega))$ 下找到一個子集 $C(\utilde{X})$ 使得

$$
\forall \theta \quad P_\theta\left(\eta(\theta)\in C(\utilde{X})\right)=r\in[0,1]
$$

而當得到實際數據 $\utilde{X}=\utilde{x}$ 時，我們稱有 r 的信心，未知量 $\eta(\theta)\in C(\utilde{x})$ 。因為當數據確定下來時，$\eta(\theta)$ 是否在 $C(\utilde{x})$ 也是確定的，只是我們不知道。

---

**EX** $X_1,\cdots, X_n\overset{\text{iid}}{\sim}N(\mu, \sigma^2_0)$

$$
\implies\frac{\sqrt{n}(\bar{X}-\mu)}{\sigma_0}\sim N(0,1)
$$

$$
\begin{align*}
    \implies \forall \mu\in\R\quad 1-\alpha&=P_\mu\left(-z_{\alpha/2}<\frac{\sqrt{n}(\bar{X}-\mu)}{\sigma_0}<z_{\alpha/2}\right)\\
    &=P_\mu\left(\bar{X}-\frac{z_{\alpha/2}\sigma_0}{\sqrt{n}}<\mu<\bar{X}+\frac{z_{\alpha/2}\sigma_0}{\sqrt{n}}\right)
    &=P_\mu\left(\mu\in(\bar{X}-\frac{z_{\alpha/2}\sigma_0}{\sqrt{n}}, \bar{X}+\frac{z_{\alpha/2}\sigma_0}{\sqrt{n}})\right)
\end{align*}
$$

$$
\implies P_\mu(\mu\in C(\utilde{X}))=1-\alpha\quad \forall\mu\text{ where }C(\utilde{X})=[\bar{X}\pm\frac{z_{\alpha/2}\sigma_0}{\sqrt{n}}]
$$

:::info[Definition]
$\utilde{X}\sim f(\utilde{x};\theta)$ where $\theta$ is the true parameter.

$$
P_\theta(\eta(\theta)\in C(\utilde{X}))
$$

is the *coverage probability(涵蓋幾率)* of $C(\utilde{X})$ for $\eta(\theta)$.
:::

我們當然會希望 conv. prob. 越大越好，但按照這個想法 $C(\utilde{X})=\eta(\Omega)$ 一定會是最好的.但 $\eta(\Omega)$ 在實踐中是沒用的，因此我們需要另一種評判標準。我們希望在 conv. prob. 相同的情況下，$C(\utilde{X})$ 越小越好。

我們計算 $C(\utilde{X})$ 會覆蓋所有錯誤點的幾率，即 

$$
P_\theta(\eta(\theta^*)\notin C(\utilde{X})) \quad \forall \theta^*\neq\theta
$$

這個幾率越小越好，我們稱之為 *false cov. prob*。而當 $C(\utilde{X})=\eta(\Omega)$ 時，error prob. = 1.

Remark:

我們會希望 cov. prob. 越大越好，而覆蓋到**在意的** $\forall \theta^*\neq\theta$ 的幾率越小越好。其中：
- 對於雙邊區間 $C(\utilde{X})=[L(\utilde{X}), U(\utilde{X})]$，我們在意 $\forall \theta^*\neq\theta$
- 單邊區間 $C(\utilde{X})=[L(\utilde{X}), \infty)$，我們在意 $\forall \theta^*\neq\theta$ with $\eta(\theta^*)<\eta(\theta)$
- 單邊區間 $C(\utilde{X})=(-\infty, U(\utilde{X})]$，我們在意 $\forall \theta^*\neq\theta$ with $\eta(\theta^*)>\eta(\theta)$

因為 cov. prob. 和 false cov. prob. 是互斥的關係。想要 cov. prob. 最大化，那麼我們會取 $C(\utilde{X})=\eta(\Omega)$。想要 false cov. prob. 最小化，那麼我們會取 $C(\utilde{X})=\eta(\empty)$。因此我們需要一個平衡點。

我們首先會希望 cov. prob 至少要大於某個*信賴係數*，然後再盡可能讓 false cov. prob. 最小化。

:::info[Definition]
$C(\utilde{X})$ is a conf. set for $\eta(\theta)$

1. $P_\theta(\eta(\theta)\in C(\utilde{X})), \forall\theta\in\Omega$ is the *coverage probability(涵蓋幾率)* of $C(\utilde{X})$
2. $C(\utilde{X})$ is called a $1-\alpha$ conf. set for $\eta(\theta)$ if
   $$
    \begin{align*}
        \inf_{\theta\in\Omega}P_\theta(\eta(\theta)\in C(\utilde{X})) &= \text{ conf. coef of }C(\utilde{X})\\
        &= 1-\alpha, \quad \alpha\in [0,1]
    \end{align*}
   $$
3. A $1-\alpha$ conf. set $C^*(\utilde{X})$ is called a *uniformly most accurate (UMA)* $1=\alpha$ for $\eta(\theta)\iff$
   1. $\inf_{\theta\in\Omega}P_\theta(\eta(\theta)\in C^*(\utilde{X}))=1-\alpha$ 
   2. $P_\theta(\eta(\theta^*)\in C^*(\utilde{X})\le P_\theta(\eta(\theta^*))\in C(\utilde{X})), \forall\theta^*\neq\theta$ relevant
   
      $\forall 1-\alpha$ conf. set $C(\utilde{X})$ for $\eta(\theta)$
4. A $1-\alpha$ conf. set $C(\utilde{X})$ for $\eta(\theta)$ is *unbiased* $\iff 1-\alpha\ge$ relevant false cov. prob.
   
   i.e. $1-\alpha\ge P_\theta(\eta(\theta^*)\in C(\utilde{X})), \forall\theta^*\neq\theta$ relevant 
5. A $1-\alpha$ conf. set $C^*(\utilde{X})$ for $\eta(\theta)$ is *UMAU* $1-\alpha$ conf. set if $C^*(\utilde{X})$ is **UMA** among **unbiased** $1-\alpha$ conf. set.
:::

**EX** $X_1, \cdots, X_n\overset{\text{iid}}{\sim}N(\mu, \sigma^2_0)$ of interest $\mu$

recall: pointest for $\mu$: $\bar{X}$(UMVUE, MLE, MOME, Minimax)

But $P_\mu(\bar{X}=\mu)=0, \forall\mu$ $\implies$ idea: $\mu\in[\bar{X}\pm c]=C(\utilde{X}), c>0$ given, with positive prob. of being correct.

$$
\begin{align*}
    \text{cov. prob of }[\bar{X}\pm c]&=P_\mu(\mu\in[\bar{X}\pm c])\\
    &=P_\mu(\mu-c\le\bar{X}\le\mu+c)\\
    &=P(\frac{\sqrt{n}(\mu-c\mu)}{\sigma_0}\le Z\le\frac{\sqrt{n}(\mu+c\mu)}{\sigma_0})\\
    &=\Phi(\frac{c\sqrt{n}}{\sigma_0})-\Phi(-\frac{c\sqrt{n}}{\sigma_0})\\
    &=2\Phi(\frac{c\sqrt{n}}{\sigma_0})-1\quad\forall\mu
\end{align*}
$$

$$
\implies \inf_{\mu\in\R}P_\mu(\mu\in[\bar{X}\pm c])=2\Phi(\frac{c\sqrt{n}}{\sigma_0})-1
$$

is the conf. coef. of $[\bar{X}\pm c]$

e.g. $n=4, \sigma_0=1, c=1$

$\implies 2\Phi(\frac{1\cdot 2}{1})-1=2\Phi(2)-1=2\cdot 0.9772-1=0.9544$

i.e. $[\bar{X}\pm 1]$ is a 95.44% conf. set for $\mu$

If want to have conf. coef. = $1-\alpha$

$\because 2\Phi(\frac{c\sqrt{n}}{\sigma_0})-1=1-\alpha\implies c=\frac{\sigma_0}{\sqrt{n}}z_{\alpha/2}$

$\implies [\bar{X}\pm\frac{\sigma_0}{\sqrt{n}}z_{\alpha/2}]$ is a $1-\alpha$ conf. set for $\mu$

If now, $\sigma^2_0$ is unknown, $\theta=(\mu, \sigma^2_0), \eta(\theta)=\theta$

$$
\begin{align*}
    \text{Conf. coef. of }[\bar{X}\pm c]&=\inf_{\theta\in\Omega}P_\theta(\mu\in[\bar{X}\pm c])\\
    
    &=\inf_{\sigma_0>0}[2\Phi(\frac{c\sqrt{n}}{\sigma_0})-1]=0
\end{align*}

$$

i.e. $[\text{Good point}\pm c]$ 可能並不能得到一個好的結果。

---

**EX** $X_1, \cdots, X_n\overset{\text{iid}}{\sim}N(\mu, \sigma^2_0)$. Given $C(\utilde{X})=[\bar{X}-\frac{\sigma_0}{\sqrt{n}}Z_\alpha, \infty]$

$$
\begin{align*}
    P_\mu(\mu\in C(\utilde{X}))&=P_\mu(\mu\ge\bar{X}-\frac{\sigma_0}{\sqrt{n}}Z_\alpha)\\
    &=P_\mu(\frac{\sqrt{n}(\bar{X}-\mu)}{\sigma_0}\le \frac{\sqrt{n}(\mu+\frac{\sigma_0}{\sqrt{n}}Z_\alpha-\mu)}{\sigma_0})\\
    &=P(Z\le Z_\alpha)\\
    &=1-\alpha
\end{align*}
$$

$\implies C(\utilde{X})$ is $1-\alpha$ conf. lower limit for $\mu$

Is it unbiased?

$$
\begin{align*}
    &P_\mu(\mu^*\in C(\utilde{X}))\quad\forall\mu^*<\mu\\
    =&P_\mu(\mu^*\ge\bar{X}-\frac{\sigma_0}{\sqrt{n}}Z_\alpha)\\
    =&P(Z\le Z_\alpha+\underbrace{\frac{n}{\sigma_0}(\mu^*-\mu)}_{<0})\\
    <&1-\alpha
\end{align*}
$$

