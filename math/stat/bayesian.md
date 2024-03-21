# Bayesian Estimation & Minimax Estimation

## 損失函數

之前提到過，我們的目標是針對 $\eta(\theta)$ 找到一個好的點估計。而我們用 $MSE(\delta, \theta)\triangleq E_\theta[(\delta(\utilde{X})-\eta(\theta))^2]$ 來衡量一個點估計的好壞。但對於不同的情景和需求，我們可能會需要不同的損失函數，所以我們需要一個更一般的定義。

:::tip[Definition]
Define a loss function $L(\delta,\theta)$ for $\eta(\theta)$ $ with
1. $L(\delta(\utilde{X},\theta))\ge 0, \forall \utilde{x}, \theta$
2. $L(\eta(\theta), \theta)=0, \forall\theta$
:::

E.g. 
- $L(\delta, \theta)=(\delta-\theta)^2$
- $L(\delta, \theta)=|\delta-\theta|$
- $L(\delta, \theta)=w(\theta)|\delta-\eta(\theta)|^k, \forall k>0, w(\theta)>0$, 其中 $w(\theta)$ 是一個權重函數（weight function）

一個估計方法得到的損失函數的值越小，代表這個估計方法越好。但 $L(\delta(\utilde{X}),\theta)$ 沒有辦法比較，因為我們每次獲得的數據都不一樣，因此我們就計算損失函數的期望值，來獲得這個估計方法平均而言的表現。

## 風險函數

:::tip[Definition]
The risk function of $\delta$ is defined as
$$
R(\delta, \theta)\triangleq E_\theta[L(\delta(\utilde{X}), \theta)]
$$
:::

因此好的估計方法，可以得到較小的風險函數。但與 $MSE$ 一樣，我們同樣無法得到一個可以讓 $R(\delta, \theta)$ 最小的估計方法。

在比較不同風險函數時，對於不同的 $\theta$，更好的風險函數可能是不同的，所以我們希望用一個數字來概括 $R(\delta, \theta)$，而數字最小的那個就是最好的估計方法。

> Q: 如何概括 $R(\delta, \theta)$？

1. 我們比較所有風險函數的最大值，取最小的那個作為 $\delta^*$
   
   i.e. $R(\delta^*, \theta)=\inf_\delta[\sup_{\theta\in\Omega}R(\delta, \theta)]$

   我們稱這個估計方法為 *minmax est.*

2. 在一些情景中，$\theta$ 在某些區域內發生的幾率更高，而在其他地方不怎麼發生。此時我們需要的估計方式是在這些區域內表現得更好。因此我們需要對這些區間進行加權。這就是 *Bayesian Estimation* 的核心思想。

## Bayes estimator

## 先驗分佈

:::tip[Definition]
**Bayes estimator**:

$\pi(\theta)$: $\Omega$ 上的先驗分佈（prior distribution），是一個 pdf。

$\delta_\pi(\utilde{X})$ is called the Bayes estimator of $\eta(\theta)$ iff $r_\pi(\delta_\pi)\le r_\pi(\delta), \forall \delta$, where

$$
r_\pi(\delta)\triangleq \int_\Omega R(\delta, \theta)\pi(\theta)d\theta
$$

is called the Bayes risk of $\delta$.
:::

:::tip[Definiton]
$\delta_0(\utilde{X})$ is admissible
$\iff \not \exist \delta$ s.t. $\delta$ dom $\delta_0$, i.e. $R(\delta, \theta)\le R(\delta_\theta, \theta), \forall \theta$ and 

$\delta$ 不比 $\delta$ 糟糕，在某些方面還比 $\delta_0$ 好

$$
\begin{align*}
   &R(\delta, \theta) \le R(\delta_\theta, \theta) \quad \forall \theta \\
   &R(\delta, \theta) < R(\delta_\theta, \theta) \quad \text{some } \delta
\end{align*}
$$
:::

:::tip[Thorem]
Any unique Bayes est is adm
:::

**Proof**:let $\delta_\pi$ be the unique Bayes est with respect to prior $\pi$

Suppose that $\delta_\pi$ is *Not* adm, i.e. $\exist\delta$ s.t.

$$
\begin{align*}
   &R(\delta, \theta) \le R(\delta_\theta, \theta) \quad \forall \theta \\
   &R(\delta, \theta) < R(\delta_\theta, \theta) \quad \text{some } \delta
\end{align*}
$$

$\because \delta_\pi$ is Bayes est $\implies \int_\Omega R(\delta_\pi, \theta)\pi(\theta)d\theta \le \int_\Omega R(\delta, \theta)\pi(\theta)d\theta \iff r_\pi(\delta_\pi)\le r_\pi(\delta)$

$\because \delta$ dom $\delta_\pi \implies$ $r_\pi(\delta)=r_\pi(\delta_\pi)$, i.e. $\delta$ is also Bayes est and $\delta \neq \delta_\pi$ 

But $\delta\pi$ is unique, contradiction.

---

**EX**: Let $\pi(\theta)=P(\theta=\theta_c)=1$, with $\theta_c\in\Omega$ given, i.e. 認爲 $\theta$ 一定會是 $\theta_c$  

$\implies$ Bayes risk $r_\pi(\delta)\triangleq\int_\Omega R(\delta, \theta)\pi(\theta)d\theta=R(\delta, \theta_c)$

$\implies$ Bayes est w.r.t. $\pi$ is to $\min R(\delta, \theta_c)\iff \min E_{\theta_c}[L(\delta(\utilde{X}), \theta_c)]$

$\implies \delta_\pi(\utilde{X})=\eta(\theta_c)$ is admissible 。既然認爲 $\theta$ 一定會是 $\theta_c$，那麽就用 $\theta_c$ 作爲 $\theta$ 的估計。

---

Now let $L(\delta, \theta)=w(\theta)(\delta(\utilde{X})-\eta(\theta))^2$ and given a prior dist (pdf) $\pi(\theta)$

> Q: 如何計算 $\eta(\theta)$ 關於 $\pi(\theta)$ 的 Bayes est?

i.e. to min the $r_\pi(\delta)$

$$
\begin{align*}
   \implies r_\pi(\delta) =& \int_\Omega R(\delta, \theta)\pi(\theta) d\theta\\
   =&\int_\Omega E_\theta[L(\delta(\utilde{x}), \theta)]\pi(\theta)d\theta\\
   =&\int_\Omega\int_{\R^n} L(\delta(\utilde{x}), \theta)f(\utilde{x};\theta)d\utilde{x}\cdot\pi(\theta)d\theta\\
   \xlongequal{\text{Fubini}}&\int_{\R^n}\int_\Omega w(\theta)[\delta(\utilde{x})-\eta(\theta)]^2f(\utilde{x};\theta)\pi(\theta)d\theta d\utilde{x}
\end{align*}
$$

為了找到 $r_\pi(\delta)$ 的最小值，我們只需要找到裡面積分的最小值，因此要找到他微分是 0 的點。

$$
 \frac{d}{d\theta} \int_\Omega w(\theta)[\delta(\utilde{x})-\eta(\theta)]^2f(\utilde{x};\theta)\pi(\theta)d\theta = 0
$$

$$
2\int_\Omega w(\theta)[\delta(\utilde{x})-\eta(\theta)]f(\utilde{x};\theta)\pi(\theta)d\theta = 0
$$

$$
\int_\Omega w(\theta)\delta(\utilde{x})f(\utilde{x};\theta)\pi(\theta)d\theta = \int_\Omega w(\theta)\eta(\theta)f(\utilde{x};\theta)\pi(\theta)d\theta
$$

$$
\delta = \frac{\int_\Omega w(\theta)\eta(\theta)f(\utilde{x};\theta)\pi(\theta)d\theta}{\int_\Omega w(\theta)f(\utilde{x};\theta)\pi(\theta)d\theta}
$$

因此，$\eta(\theta)$ 的 Bayes est 是：

$$
\delta_\pi(\utilde{X}) = \frac{\int_\Omega w(\theta)\eta(\theta)f(\utilde{X};\theta)\pi(\theta)d\theta}{\int_\Omega w(\theta)f(\utilde{X};\theta)\pi(\theta)d\theta}
$$


### 後驗分佈

之前說 $\utilde{X}\sim f(\utilde{x};\theta)$ 實際上是在給定 $\theta$ 的情況下 $X$ 的 pdf。但我們現在認為 $\theta$ 同樣也是一個隨機變量，所以準確的寫法是 $\utilde{X}|_\theta\sim f(\utilde{x};\theta)$。

如果 $\pi(\theta)$ 是 $\theta$ 在 $\Omega$ 上的一個先驗分佈，就是在看到數據之前，對 $\theta$ 做主觀的猜測，那麽 $f(\utilde{x};\theta)\pi(\theta)$ 就是 $\utilde{X}$ 和 $\theta$ 的聯合分佈。

而 $\utilde{X}$ 的邊際密度函數就是：

$$
m_\pi(\utilde{x})\triangleq\int_\Omega f(\utilde{x};\theta)\pi(\theta)d\theta
$$

根據這個，我們就可以算出給定數據 $\utilde{X}=\utilde{x}$ 是，$\theta$ 的條件分佈：

$$
\pi(\theta|\utilde{x}) = \frac{f(\utilde{x};\theta)\pi(\theta)}{m_\pi(\utilde{x})}
$$

這是我們得到數據後，對 $\theta$ 推測的改進，就是*後驗*（posterior）分佈。

:::tip[Definition]

$X_1,\cdots, X_n \stackrel{\text{iid}}{\sim} f(\utilde{x};\theta)$, and $\pi(\theta)$ is the prior dist of $\theta$

The posterior dist of $\theta$ given $\utilde{X}=\utilde{x}$ is

$$
\pi(\theta|\utilde{x}) = \frac{f(\utilde{x};\theta)\pi(\theta)}{m_\pi(\utilde{x})}
$$

where $m_\pi(\utilde{x})\triangleq\int_\Omega f(\utilde{x};\theta)\pi(\theta)d\theta$ is the marginal dist of $\utilde{x}$

:::

---

**Note**: Suppose that $T=T(\utilde{X})$ 是 $\theta$ 的充分統計量，那麼根據分割定理，我們可以得到 $f(\utilde{x};\theta)=g(t;\theta)h(\utilde{x})$，這裡我們可以通過調整係數的方式，讓 $g(t;\theta)$ 是一個 pdf。則我們有：

$$
\pi(\theta|\utilde{x}) = \frac{f(\utilde{x};\theta)\pi(\theta)}{\int_\Omega f(\utilde{x};\theta)\pi(\theta)d\theta}=\frac{h(\utilde{x})g(t;\theta)\pi(\theta)}{h(\utilde{x})\int_\Omega g(t;\theta)\pi(\theta)d\theta}=\frac{g(t;\theta)\pi(\theta)}{\int_\Omega g(t;\theta)\pi(\theta)d\theta}=\pi(\theta|t)
$$

在 loss function 是 $L(\delta, \theta)=w(\theta)(\delta(\utilde{X})-\eta(\theta))^2$ 的情況下，並且我們已經拿到數據 $\utilde{X}=\utilde{x}$，我們可以得到：

$$
\begin{align*}
   \delta_\pi(\utilde{x})&=\frac{\int_\Omega w(\theta)\eta(\theta)f(\utilde{x};\theta)\pi(\theta)d\theta}{\int_\Omega w(\theta)f(\utilde{x};\theta)\pi(\theta)d\theta}\\
   &=\frac{\int_\Omega w(\theta)\eta(\theta)\frac{f(\utilde{x};\theta)\pi(\theta)}{m_\pi(\utilde{x})}d\theta}{\int_\Omega w(\theta)\frac{f(\utilde{x};\theta)\pi(\theta)}{m_\pi(\utilde{x})}d\theta}\\
   &=\frac{\int_\Omega w(\theta)\eta(\theta)\pi(\theta|\utilde{x})dx}{\int_\Omega w(\theta)\pi(\theta|\utilde{x})d\theta}=\frac{E[w(\theta)\eta(\theta)|\utilde{x}]}{E[w(\theta)|\utilde{x}]}\\
   &=\frac{E[w(\theta)\eta(\theta)|t]}{E[w(\theta)|t]} \qquad \text{where } t=T(\utilde{x}) \text{ is suff. for } \theta
\end{align*}
$$

也就是說：
$$
\delta_\pi(\utilde{X})=\frac{E[w(\theta)\eta(\theta)|\utilde{X}]}{E[w(\theta)|\utilde{X}]}=\frac{E[w(\theta)\eta(\theta)|T]}{E[w(\theta)|T]}\xlongequal{w(\theta)=1}E[\eta(\theta)|T]
$$

因此在 MSE 作為風險函數時，Bayes est 沒有辦法再通過 *Rao-Blackwell Theorem* 進行改進，因為 Bayes est 已經是充分統計量的函數。

:::tip[Theorem]
1. Under loss function $L(\delta, \theta)=w(\theta)(\delta(\utilde{X})-\eta(\theta))^2$, the **unique** Bayes est with respect to prior $\pi$ is 
   $$
   \delta_\pi(\utilde{X})=\frac{E[w(\theta)\eta(\theta)|\utilde{X}]}{E[w(\theta)|\utilde{X}]}=\frac{E[w(\theta)\eta(\theta)|T]}{E[w(\theta)|T]}
   $$
2. Under loss function $L(\delta, \theta)=(\delta(\utilde{X})-\eta(\theta))^2$ with respect to prior $\pi$, the unique Bayes est is 
   $$
   \delta_\pi(\utilde{X})=E[\eta(\theta)|\utilde{X}]=E[\eta(\theta)|T]
   $$ 
:::

---

### Minimax Estimation

Bayes est. 可以幫助我們找到 Minimax est.

:::tip[Theorem]
在給定損失函數 $L(\delta, \theta)$ 之下，如果 $\delta_\pi(\utilde{X})$ 滿足以下條件

1. $\delta_\pi(\utilde{X})$ 是 $\eta(\theta)$ 關於先驗分佈 $\pi(\theta)$ 的 Bayes est

2.  $R(\delta_\pi, \theta)$ 是常數（與 $\theta$ 無關）

則 $\delta_\pi(\utilde{X})$ 是 $\eta(\theta)$ 的 minmax est.

如果進一步 $\delta_\pi(\utilde{X})$ 是**唯一**的 Bayes est，則 $\delta_\pi(\utilde{X})$ 同樣是 $\eta(\theta)$ 的**唯一**的 minmax est.
:::

**Proof**: 令 $\delta_\pi$ 是 $\eta(\theta)$ 的 Bayes est. 使得 $R(\delta_\pi, \theta)$ 是常數。

$$
\begin{align*}
   r_\pi(\delta_\pi) \triangleq& \int_\Omega R(\delta_\pi, \theta)\pi(\theta)d\theta\\
   =& R(\delta_\pi, \theta)\int_\Omega\pi(\theta)d\theta \qquad \because R(\delta_\pi, \theta) \text{ is const.}\\
   =& \sup_{\theta\in\Omega}R(\delta_\pi, \theta) \qquad \because \pi(\theta) \text{ is pdf}
\end{align*}
$$

$\forall \delta \neq \delta_\pi$

$$
\begin{align*}
   r_\pi(\delta) \triangleq& \int_\Omega R(\delta, \theta)\pi(\theta)d\theta\\
   \le& \int_\Omega \sup_{\theta\in\Omega}R(\delta_\pi, \theta)\pi(\theta)d\theta\\
   =& \sup_{\theta\in\Omega}R(\delta_\pi, \theta)\int_\Omega\pi(\theta)d\theta\\
   =& \sup_{\theta\in\Omega}R(\delta_\pi, \theta) \qquad \because \pi(\theta) \text{ is pdf}
\end{align*}
$$

因為 $\delta_\pi$ 是 Bayes est：

$$
\begin{align*}
   \sup_{\theta\in\Omega}R(\delta_\pi, \theta) = r_\pi(\delta_\pi) \stackrel{(<) \text{ if unique}}{\le} r_\pi(\delta) \le \sup_{\theta\in\Omega}R(\delta_\pi, \theta)
\end{align*}
$$

因為 $\delta$ 是任意的，i.e. $\sup_{\theta\in\Omega}R(\delta_\pi, \theta) \le \inf_\delta\sup_{\theta\in\Omega}R(\delta_\pi, \theta) \implies \delta_\pi(\utilde{X})$ 是 $\eta(\theta)$ 的 minmax est.

:::info[Remark]
因為 minimax est 不在意 $\theta$ 的 prior dist，所以如果我們要從 Bayes est 得到 $\eta$ 的 minimax est，只需要找到一個先驗分佈，使得風險函數是常數。
:::

:::info[Remark]
$f$ 和 $g$ 都是 pdf，且成正比，则 $f=g$
:::

**EX**: $X_1, \cdots, X_n \stackrel{\text{iid}}{\sim} B(1,\theta)$ with $\theta\sim Beta(\alpha, \beta)$ 

i.e. $\theta\in\Omega=(0,1)$ and 
$$
\begin{align*}
   \pi(\theta)&=\frac{\Gamma(\alpha+\beta)}{\Gamma(\alpha)\Gamma(\beta)}\theta^{\alpha-1}(1-\theta)^{\beta-1}\\
   &\propto \theta^{\alpha-1}(1-\theta)^{\beta-1}
\end{align*}
$$

e.g. $\alpha=1=\beta,\implies \pi(\theta)=1, \theta\in(0,1)$ i.e. $\theta\sim U(0,1)$ 也就是說用戶沒有意見。

$$
\begin{align*}
   &\begin{align*}
      \implies \pi(\theta|\utilde{x}) &= \frac{f(\utilde{x};\theta)\pi(\theta)}{\int_\Omega f(\utilde{x};\theta)\pi(\theta)d\theta}\\
      &\propto f(\utilde{x};\theta)\pi(\theta) \quad \because\text{分母的 } \theta \text{ 被積分掉，且} \utilde{x} \text{ 給定} \\
      & \propto  \theta^{t+\alpha-1}(1-\theta)^{n-t+\beta-1} \qquad \text{with } t=\sum x_i \\
      & \propto \text{pdf of } Beta(t+\alpha, n-t+\beta)
   \end{align*}\\

   &\implies \theta|_{\utilde{x}} \sim Beta(t+\alpha, n-t+\beta)
\end{align*}
$$

1. 在損失函數 $L(\delta, \theta)=[\delta(\utilde{X})-\theta]^2$ 的情況下，我們可以得到 $\delta_\pi(\utilde{X})=E[\theta|\utilde{X}]=\frac{T+\alpha}{n+\alpha+\beta}$
   
   $$
   \delta_\pi(X)=\frac{T+\alpha}{n+\alpha+\beta} = \frac{\alpha+\beta}{n+\alpha_\beta}\frac{\alpha}{\alpha+\beta}+\frac{n}{n+\alpha+\beta}\frac{T}{n}=\lambda\frac{\alpha}{\alpha+\beta}+(1-\lambda)\bar{X}
   $$

   我們希望找到 minimax est，為此我們需要找到一組適合的 $(\alpha, \beta)$ 使得風險函數是常數。

   $$
   \begin{align*}
      R(\delta_\pi, \theta) &= E_\theta[(\delta_\pi(\utilde{X})-\theta)^2]\\
      &= Var_\theta(\delta_\pi(\utilde{X}))+Bias^2(\delta_\pi(\utilde{X})) \quad \text{MSE 的分解}\\
      &=Var_\theta(\frac{T+\alpha}{n+\alpha+\beta})+[E_\theta(\frac{T+\alpha}{n+\alpha+\beta})-\theta]^2\\
      &=\frac{n\theta(1-\theta)}{n+\alpha+\beta}+\left[\frac{n\theta(1-\theta)}{n+\alpha+\beta}-\theta\right]^2 \quad \text{where } T=\sum_{i=1}^n X_i\sim B(n, \theta)\\
      &=\frac{1}{(n+\alpha+\beta)^2}[((\alpha+\beta)^2-n)\theta+(n-2\alpha(\alpha+\beta))\theta+\alpha]\\
   \end{align*}
   $$

   為了使 $\theta$ 的係數為 0：

   $$
   \left\{
   \begin{align*}
      (\alpha+\beta)^2-n &=0\\
      n-2\alpha(\alpha+\beta) &=0
   \end{align*}
   \right.
   $$

   $\implies \alpha=\beta=\frac{\sqrt n}{2}, \delta_\pi(\utilde{X})=\frac{\frac{\sqrt n}{2}+T}{n+\sqrt{n}}$ 是唯一的 minmax est.
   
   這與我們傳統上用 $\bar{X}$ 估計 $\theta$ 的結果不同。

2. 令損失函數為 $L(\delta, \theta)=\frac{[\delta(\utilde{X})-\theta]^2}{\theta(1-\theta)}$ ，i.e. $w(\theta)=\frac{1}{\theta(1-\theta)}=\frac{1}{Var(T)}$。這樣可以將單位統一（unit-free），也就是將方差都變成 1。

   $$
   \delta_\pi(\utilde{X})=\frac{E[\frac{1}{1-\theta}|\utilde{X}]}{E[\frac{1}{\theta(1-\theta)}|\utilde{X}]}
   $$

   因為之前已經知道 $\theta|_{\utilde{x}} \sim B(\alpha+t, n-t+\beta)$，因此可以算出兩個期望值

   $$
   \implies \delta_\pi(\utilde{X})=\frac{\alpha+T-1}{n+\alpha+\beta-2}\xlongequal[U(0,1)]{\alpha=\beta=1} \frac{T}{n}= \bar{X}
   $$

   也就是說，當 $\theta$ 的先驗分佈是 $U(0,1)$ 時，$\bar{X}$ 是 $\theta$ 唯一的 Bayes est.

   並且它的風險函數

   $$
   R(\bar{X}, \theta)=E_\theta[\frac{(\bar{X}-\theta)^2}{\theta(1-\theta)}]=\frac{Var(\bar{X})}{\theta(1-\theta)}=\frac{1}{n} \quad \text{ is const.}
   $$

   因此 $\bar{X}$ 是 $\theta$ 的 minmax est.


:::info[Remark]
一個估計方法 $\delta$ 如果在任意一個 prior 下是 Bayes est，且風險函數是常數，那麽 $\delta$ 就是 minmax est.

但假如 $\delta$ 不可能會是 Bayes est，就無法用這個方法找到 minmax est.

我們需要通過逼近 Bayes 的方法來找到 minmax est.
:::

:::tip[Theorem]
在給定損失函數之下，如果 $\delta_0$ 使得 $\sup_\theta R(\delta_0\theta)=r$ 。

假設 $(\pi_m)$ 是一系列的先驗分佈，它們對應的 Bayes est $(\delta_{\pi_m})$ 有 Bayes rist $(r_{\pi_m}(\delta_{\pi_m}))$，且 $\lim_{m\to\infty}r_{\pi_m}(\delta_{\pi_m})=r$。

則 $\delta_0$ 是 $\eta(\theta)$ 的 minmax est.
:::

**Proof**: 對於任何 $\delta$，我們有

$$
\begin{align*}
   \sup_\theta R(\delta, \theta) \ge \int_\Omega R(\delta, \theta)\pi_m(\theta)d\theta 
   &= r_{\pi_m}(\delta) \\
   \because \delta_pi \text{ is Bayes est.} \quad &\ge r_{\pi_m}(\delta_{\pi_m}) = r_m \xrightarrow[m\to\infty]{} r = \sup_\theta R(\delta_0, \theta)
\end{align*}
$$

i.e. $\forall \delta, \sup_\theta R(\delta, \theta) \ge \sup_\theta R(\delta_0, \theta)$， 因此 $\delta_0$ 是 minmax est.

---


**EX**: $X_1, \cdots, X_n \overset{\text{iid}}{\sim} P(\theta)$ with loss function $L(\delta, \theta)=\frac{1}{\theta}(\delta(\utilde{X})-\theta)^2$. Proof that $\delta(\utilde{X})=\bar{X}$ is minmax est for $\theta$.

考慮 $\theta$ 的先驗分佈，$\theta\sim Exp(\tau)$

$$
\begin{align*}
   \implies & \pi(\theta|\utilde{x}) \propto f(\utilde{x};\theta)\pi(\theta) \propto \theta^t e^{-(n+\tau)\theta} \quad \text{where } t=\sum x_i\\
   \implies &  \theta|\utilde{x} \sim Gamma(t+1, \frac{1}{n+\tau})\\
   \implies & \delta_\pi(\utilde{X}) = \frac{1}{E[\frac{1}{\theta}|\utilde{X}]} = \frac{T}{n+\tau} \xrightarrow[\tau\to\infty]{} \frac{T}{n} = \bar{X}
\end{align*}
$$

$$
\begin{align*}
   R(\delta_\pi, \theta) &=E_\theta\left[\frac{(\delta_\pi(\utilde{X})-\theta)^2}{\theta}\right]\\
   &=\frac{1}{\theta}E_\theta\left[(\frac{T}{n+\tau}-\theta)^2\right] \quad \text{where } T=\sum X_i\sim P(n\theta)\\
   &=\frac{1}{\theta}\left[Var(\frac{T}{n+\tau}-\theta) + \left(E_\theta(\frac{T}{n+\tau}-\theta) \right)^2 \right]\\
   &=\frac{1}{\theta}(\frac{1}{\theta+\tau})^2n\theta + \frac{1}{\theta}(\frac{n\theta}{n+\tau}-\theta)^2\\
   &=\frac{n}{(n+\tau)^2} + \frac{\tau^2\theta}{(n+\tau)^2}
\end{align*}
$$

$$
\begin{align*}
   \implies r_\pi(\delta_\pi) &= \int_\Omega R(\delta_\pi, \theta)\pi(\theta)d\theta\\
   &=\int_0^\infty \left[\frac{n}{(n+\tau)^2} + \frac{\tau^2\theta}{(n+\tau)^2}\right]\pi(\theta)d\theta\\
   &=E_\theta\left[\frac{n}{(n+\tau)^2} + \frac{\tau^2\theta}{(n+\tau)^2}\right] \quad \text{where } \theta\sim Exp(\tau) \overset{d}{=} Gamma(1, \frac{1}{\tau})\\
   &=\frac{n}{(n+\tau)^2} + \frac{\tau^2}{(n+\tau)^2}E_\theta(\theta)\\
   &=\frac{1}{n+\tau} \xrightarrow[\tau\to\infty]{} \frac{1}{n} = r
\end{align*}
$$

$\implies \sup_\theta R(\delta_\pi, \theta)\pi(\theta) = r = \frac{1}{n}$

---

**EX**: $X_1, \cdots, X_n\overset{\text{iid}}{\sim} N(\theta, \sigma^2_0)$

$\implies T=\bar{X}$ is suff. for $\theta$, and $T\sim N(\theta, \frac{\sigma^2_0}{n}) = N(\theta, \sigma^2)$ where $\sigma^2=\frac{\sigma^2_0}{n}$

:::info[Remark]
在損失函數 $L(\delta, \theta)=(\delta(\utilde{X})-\theta)^2$ 之下，任何無偏估計都不會是 $\theta$ 的 minmax est.
:::

Let prior $\pi$ s.t. $\theta\sim N(\mu, \tau)$

$$
\begin{align*}
   \implies \pi(\theta|\utilde{x}) & \propto e^{-\frac{(t-\theta)^2}{\sigma^2}}e^{-\frac{(\theta-\mu)^2}{\tau^2}}\\
\end{align*}
$$