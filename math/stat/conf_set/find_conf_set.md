# Methods of finding confidence sets

## Inverting tests

Idea: 當我們做假設檢定 $H_0:\theta=\theta_0$ 時，如果我們沒有拒絕 $H_0$，那麼我們可以說這組數據是“支持” $\theta=\theta_0$ 的。因此，我們可以認為 $\theta_0\in C(\utilde{X})$。那麼我們對所有 $\theta_0$ 都作這樣的檢定，如果我們在資料 $\utilde{x}$ 下沒有拒絕這個 $H_0$，我們就把這個 $\theta_0$ 放進 $C(\utilde{x})$。這樣我們就得到了一個 confidence set。

**EX** $X_1, \cdots, X_n\overset{\text{iid}}{\sim}U(0, \theta)$

$$
H_0:\theta=\theta_0\quad\text{vs.}\quad H_1:\theta\neq\theta_0
$$

$\implies$ UMP level $\alpha$ test is rejects $H_0$ if $x_{(n)}>\theta_0$ or $x_{(n)}<\theta_0\alpha^\frac{1}{n}$

$\iff$ Note rejects $H_0$ if $x_{(n)}\le\theta_0$ and $x_{(n)}\ge\theta_0\alpha^\frac{1}{n}$

Note.

$$
\begin{align*}
    &P_{\theta_0}(\text{rej. }H_0) = \alpha \quad \forall\theta_0 > 0\\
    \iff& P_{\theta_0}(\text{Not rej. }H_0)=1-\alpha \quad \forall\theta_0 > 0\\
    \iff& P_{\theta}(X_{(n)}\le\theta\le X_{(n)}\alpha^{-\frac{1}{n}})=1-\alpha \quad \forall\theta > 0
\end{align*}
$$

$\implies [X_{(n)}, X_{(n)}\alpha^{-\frac{1}{n}}]$ is a $1-\alpha$ **UMA** confidence set for $\theta$.

- 我們可以從 UMP $\alpha$ test 得到 UMA conf coef $1-\alpha$ 的 confidence set。

:::info[Definition]
**Inverting tests Method**:

Let a non-randomized level $\alpha$ test

$$
\phi(\utilde{X}) = \begin{cases}
    1 & \text{if } \utilde{X}\in B(\theta_0)=\text{ rejection region}\\
    0 & \text{if } \utilde{X}\in (B(\theta_0))^c\triangleq A(\theta_0)=\text{ acceptance region}
\end{cases}
$$
Def $C(\utilde{X})$ with $\utilde{X}\in A(\theta_0)\iff\theta_0\in C(\utilde{X})$, then

$$
\begin{align*}
    \forall\theta_0\quad P_{\theta_0}(\theta_0\in C(\utilde{X})) &= P_{\theta_0}(\utilde{X}\in A(\theta_0))\\
    &=1-P_{\theta_0}(\utilde{X}\in B(\theta_0))\\
    &=1-\alpha
\end{align*}
$$

i.e. $\forall\theta P_\theta(\theta\in C(\utilde{X}))=1-\alpha\implies C(\utilde{X})$ is a $1-\alpha$ confidence set for $\theta$.
:::

In fact, $\theta_0\in C(\utilde{X})\iff\utilde{X}\in A(\theta_0)$。因此我們可以通過檢定的結果來得到 confidence set，也可以通過 confidence set 來得到檢定的結果。

也就是說，there is Duality(一體兩面) relationship between confidence sets and hypothesis tests. 它們問的是同一個問題，只是從不同的角度來看。假設檢定是給定 $\theta$，看數據是否支持這個 $\theta$；而 confidence set 是給定數據，看這個數據支持哪些 $\theta$。

因此我們可以利用已知結果的檢定來得到 confidence set，並且
- UMP $\implies$ UMA
- UMPU $\implies$ UMAU

**EX** $X_1,\cdots, X_n\overset{\text{iid}}{\sim}U(0, \theta)$. Find UMAU $1-\alpha$ conf lower bound for $\theta$ ($\theta\ge L(\utilde{X})$).

$\implies$ UMP level $\alpha$ test for $H_0:\theta\le\theta_0$ vs. $H_1:\theta>\theta_0$ is rejects $H_0$ if $x_{(n)}\le\theta_0$ 

i.e. **Not reject** $H_0$ if $x_{(n)}\le\theta_0(1-\alpha)^{\frac{1}{n}}$

i.e. $\set{\utilde{x}:x_{(n)}(1-\alpha)^{-\frac{1}{n}}\le\theta_0}=A(\theta_0)$

$\implies C(\utilde{X})=\set{\theta_0:\utilde{X}\in A(\theta_0)}=\set{\theta_0:\theta_0\ge X_{n}(1-\alpha)^{-\frac{1}{n}}}=[X_{(n)}(1-\alpha)^{-\frac{1}{n}}, \infty)$

is UMA(hence UMAU) $1-\alpha$ confidence set for $\theta$.

如果我們關注的是 upper bound，那麼我們同樣做鑒定，得到 reject $H_0$ if $x_{(n)}<\theta_0\alpha^{\frac{1}{n}}$。而他的接受區域是 $\set{\utilde{x}:x_{(n)}\ge\theta_0\alpha^{\frac{1}{n}}}$。因此我們可以得到 UMAU $1-\alpha$ confidence set for $\theta$ is $(-\infty, X_{(n)}\alpha^{-\frac{1}{n}}]$。

---

**EX**

$$
\perp \Big< \begin{align*}
    &X_1, \cdots, X_n\overset{\text{iid}}{\sim}N(\mu, \sigma^2)\\
    &Y_1, \cdots, Y_m\overset{\text{iid}}{\sim}N(\mu, \sigma^2)
\end{align*}
$$

$C(\utilde{X}, \utilde{Y})=\set{\lambda:\lambda\ge L(\utilde{X}, \utilde{Y})}$ with $\theta=(\mu_x, \mu_y, \sigma^2_x, \sigma^2_y)$

$\implies$ UMAU lower bound $1-\alpha$ for $\lambda=\frac{\sigma^2_x}{\sigma^2_y}$

$\implies$ UMPU level $\alpha$ for 

$$
H_0:\frac{\sigma^2_x}{\sigma^2_y}=\lambda=\lambda_0\quad\text{vs.}\quad H_1:\frac{\sigma^2_x}{\sigma^2_y}=\lambda>\lambda_0
$$

rejects $H_0$ if $\frac{S^2_x/\sigma^2_x}{S^2_y/\sigma^2_y}>F_{n-1, m-1}(\alpha)\iff \frac{S^2_x}{S^2_Y}>\lambda_0F_{n-1, m-1}(\alpha)$ where $\lambda_0\overset{H_0}{=}\frac{\sigma^2_x}{\sigma^2_y}$

i.e. Not reject $H_0$ if $\frac{S^2_x}{S^2_y}\le\lambda_0F_{n-1, m-1}(\alpha)\implies\lambda\in C(\utilde{X}, \utilde{Y})=\set{\lambda:\lambda\le\frac{S^2_X}{S^2_Y}F_{n-1,m-1,\alpha}^{-1}}$

i.e. $[\frac{S^2_X}{S^2_Y}\frac{1}{F_{n-1,m-1,\alpha}},\infty)$ is $1-\alpha$ UMAU lower bound for $\lambda=\frac{\sigma^2_x}{\sigma^2_y}$.

## Pivot

:::info[Definition]
A r.v. $K(\utilde{X};\theta)$ is called a *pivot* $\iff K(\utilde{X};\theta)$ 's dist $\perp\theta$
:::

**EX** $X_1,\cdots, X_n\overset{\text{iid}}{\sim}N(\mu, \sigma^2)$, 

$$
\frac{\sqrt{n}(\bar{X}-\mu)}{\sigma}\sim N(0, 1) \perp \mu, \sigma^2
$$

If $K(\utilde{X},\theta)$ is a pivot, then $\forall$ set $B$, $P(K(\utilde{X},\theta)\in B)\perp\theta$

$\implies$ Given $\alpha\in(0, 1), \exist B_\alpha$ s.t. $P(K(\utilde{X},\theta)\in B_\alpha)=1-\alpha$. i.e. $\inf_\theta P(K\in B_\alpha)=1-\alpha$

:::info[Definition]
$$
C(\utilde{X})=\set{\theta:K(\utilde{X},\theta)\in B_\alpha}
$$

we have

$$
\forall\theta\quad P_\theta(\theta\in C(\utilde{X}))=P(K(\utilde{X},\theta)\in B_\alpha)=1-\alpha
$$

$\implies \inf_\theta P_\theta(\theta\in C(\utilde{X}))=1-\alpha$, i.e. $C(\utilde{X})$ is a $1-\alpha$ confidence set for $\theta$.
:::

**EX** $X_1,\cdots, X_n\overset{\text{iid}}{\sim}N(\mu, \sigma^2_0)$ with $\sigma^2_0$ known. Given $\alpha\in(0, 1)$, find $1-\alpha$ confidence set for $\mu$.

$$
\implies \frac{\sqrt{n}(\bar{X}-\mu)}{\sigma_0}\sim N(0, 1)\perp\mu
$$

Let $B_\alpha=[-Z_{\alpha_1}, Z_{\alpha_2}]$ with $\alpha_1+\alpha_2=\alpha\implies P\left(\frac{\sqrt{n}(\bar{X}-\mu)}{\sigma_0}\in B_\alpha\right)=1-\alpha$

$$
\begin{align*}
    C(\utilde{X})&=\set{\mu:-Z_{\alpha_1}\le\frac{\sqrt{n}(\bar{X}-\mu)}{\sigma_0}\le Z_{\alpha_2}}\\
    &=\set{\mu:\bar{X}-\frac{\sigma_0Z_{\alpha_1}}{\sqrt{n}}\le\mu\le\bar{X}+\frac{\sigma_0Z_{\alpha_2}}{\sqrt{n}}}\\
    &=\left[\bar{X}-\frac{\sigma_0Z_{\alpha_1}}{\sqrt{n}}, \bar{X}+\frac{\sigma_0Z_{\alpha_2}}{\sqrt{n}}\right]
\end{align*}
$$

所有 $\alpha_1+\alpha_2=\alpha$ 構建的 confidence set 都是 $1-\alpha$ confidence set，但我們希望找到最好的 confidence set，也就是找到長度最短的 confidence set。

$C(\utilde{X})$ 的長度 $=\frac{\sigma_0}{\sqrt{n}}(Z_{\alpha_2}-Z_{\alpha_1})$。為了使它最短，並且保證 $P(Z_{\alpha_1}\le Z\le Z_{\alpha_2})=1-\alpha$

$\iff$ min $(Z_{\alpha_2}-Z_{\alpha_1})$ s.t. $\Phi(Z_{\alpha_2})-\Phi(Z_{\alpha_1})=1-\alpha$

我們可以使用 Largrange multiplier method 來求解這個問題。如果我們要使 $h(a,b)$ 最小（或最大）並保證限製函數 $g(a,b)=k$，即找到 $a,b$ 使得

$$
\nabla h(a,b)=\lambda\nabla g(a,b)\quad \text{and } g(a,b)=k
$$

這裡 $h(a,b)=b-a, g(a,b)=\Phi(b)-\Phi(a), k=1-\alpha$

$$
\begin{cases}
    \frac{\partial}{\partial a}(b-a)=\lambda\frac{\partial}{\partial a}\Phi(a)\\
    \frac{\partial}{\partial b}(b-a)=\lambda\frac{\partial}{\partial b}\Phi(b)\\
    \Phi(b)-\Phi(a)=1-\alpha
\end{cases}\implies\begin{cases}
    -1=-\lambda\phi(a)\\
    1=\lambda\phi(b)\\
    \Phi(b)-\Phi(a)=1-\alpha
\end{cases}
$$

$$
\implies \phi(a)=\phi(b) \text{ and } \Phi(b)-\Phi(a)=1-\alpha \implies a=-b 
$$

i.e. $Z_{\alpha_1}=Z_{\frac{\alpha}{2}}, Z_{\alpha_2}=Z_{1-\frac{\alpha}{2}}\implies$ the best confidence set for $\mu$ is $\left[\bar{X}\pm \frac{\sigma_0}{\sqrt{n}}Z_{\frac{\alpha}{2}}\right]$ with length $\frac{2\sigma_0}{\sqrt{n}}Z_{\frac{\alpha}{2}}$

同樣，我們也可以用 $\frac{X_1-\mu}{\sigma_0}$ 作為 pivot 來得到 confidence set $\left[X_1\pm\sigma_0Z_{\frac{\alpha}{2}}\right]$，而長度為 $2\sigma_0Z_{\frac{\alpha}{2}}$。

比較兩個 confidence set 的長度，我們可以發現 $\bar{X}$ 的 confidence set 的長度比 $X_1$ 的 confidence set 的長度要短。因此我們會選擇基於 $\bar{X}$ 的 confidence set。

**Remark**: 我們會使用基於 sufficient statistics 的 pivot 來構建 confidence set。

**Note**

1. $X_1, \cdots, X_n\overset{\text{iid}}{\sim}f(x;\theta)=f(x-\theta)$ called location family
   
   Let $Y_i=X_i-\theta\iff X_i=Y_i+\theta$

   $$
    \implies f_{Y_i}(y)=f_{X_i}(y+\theta)\cdot |\frac{d}{dy}(y+\theta)|=f(y+\theta-\theta)=f(y) \perp \theta
   $$

   $\implies Y_1,\cdots,Y_n\overset{\text{iid}}{\sim} f(y)$ are pivots $\implies\forall h, h(Y_1,\cdots, Y_n)$ are also pivots

2. $X_1, \cdots, X_n\overset{\text{iid}}{\sim}f(x;\sigma)=\frac{1}{\sigma}f(\frac{x}{\sigma})$ called scale family
   
   Let $Y_i=\frac{X_i}{\sigma}\iff X_i=Y_i\sigma$

   $$
   \implies f_{Y_i}(y)=f_{X_i}(y\sigma)\cdot |\frac{d}{dy}(y\sigma)|=\frac{1}{\sigma}f(\frac{y\sigma}{\sigma})\cdot\sigma=f(y)\perp\sigma
   $$

   $\implies Y_1,\cdots,Y_n\overset{\text{iid}}{\sim}f(y)$ are pivots $\implies\forall h, h(Y_1,\cdots, Y_n)$ are also pivots

3. $X_1, \cdots, X_n\overset{\text{iid}}{\sim}f(x;\theta, \sigma)=\frac{1}{\sigma}f(\frac{x-\theta}{\sigma})$ called location-scale family
   
   Let $Y_i=\frac{X_i-\theta}{\sigma}\iff X_i=Y_i\sigma+\theta$

   similar $Y_1,\cdots,Y_n\overset{\text{iid}}{\sim}f(y)$ are pivots $\implies\forall h, h(Y_1,\cdots, Y_n)$ are also pivots

---

**EX** $X_1, \cdots, X_n\overset{\text{iid}}{\sim}N(\mu, \sigma^2)$

$$
\frac{1}{\sqrt{2\pi}\sigma}e^{-\frac{1}{2\sigma^2}(x-\mu)^2}=\frac{1}{\sigma}\phi(\frac{x-\mu}{\sigma}) \text{ is a location-scale family}
$$

Note: $(\bar{X},S^2)$ is complete sufficient for $(\mu, \sigma^2)$

對於 $\theta$ 的信賴區間，我們用 $\frac{\sqrt{n}(\bar{X}-\theta)}{S}\sim t_{n-1}\perp (\theta, \sigma^2)$ 作為 pivot

$\forall a<b$ s.t. 

$$
\begin{align*}
    1-\alpha&=P(a\le t_{n-1}\le b)\\
    &=P_{\theta,\sigma^2}(a\le\frac{\sqrt{n}(\bar{X}-\theta)}{S}\le b)\\
    &=P_{\theta,\sigma^2}(aS\le\sqrt{n}(\bar{X}-\theta)\le bS)\\
    &=P_{\theta,\sigma^2}(\bar{X}-\frac{bS}{\sqrt{n}}\le\theta\le\bar{X}-\frac{aS}{\sqrt{n}})\\
    &=P_{\theta,\sigma^2}(\theta\in C_{a,b}(\utilde{X}))\\
\end{align*}
$$

with $C_{a,b}(\utilde{X},S^2)=[\bar{X}-\frac{bS}{\sqrt{n}}, \bar{X}+\frac{aS}{\sqrt{n}}]$ are $1-\alpha$ conf. int. for $\theta$.

To min $(b-a)$ s.t. $a<b$ and $G(b)-G(a)=1-\alpha$ with $G$: cdf of $t_{n-1}$

$$
\begin{cases}
    -1=\lambda(-g(a))\\
    1=\lambda g(b)\\
    G(b)-G(a)=1-\alpha
\end{cases}
\implies a=-b=t_{n-1,\frac{\alpha}{2}}
$$

如果用 $C^*(\bar{X}, S^2)=[\bar{X}-\frac{S}{\sqrt{n}}t_{n-1}, \bar{X}+\frac{S}{\sqrt{n}}t_{n-1}]$ 做假設檢驗

$$
H_0:\theta=\theta_0\quad\text{vs.}\quad H_1:\theta\neq\theta_0
$$

$\implies$ reject $H_0\iff\theta_0\notin C^*(\bar{X}, S^2)\iff \left|\frac{\sqrt{n}(\bar{X}-\theta_0)}{S}\right|>t_{n-1,\frac{\alpha}{2}}$ is UMPU level $\alpha$ test for $\theta$.

$\implies [\bar{X}\pm \frac{S}{\sqrt{n}}t_{n-1,\frac{\alpha}{2}}]$ is $1-\alpha$ UMAU for $\theta$.

$\sigma^2$ 的信賴區間，我們會用 $\frac{S^2}{\sigma^2}$ 作為 pivot。因為 $\frac{(n-1)S^2}{\sigma^2}\sim \chi^2_{n-1}\perp(\theta, \sigma^2)\implies \forall 0\le a<b$ s.t.

$$
\begin{align*}
    1-\alpha&=P(a\le\chi^2_{n-1}\le b)\\
    &=P_{\theta,\sigma^2}(a\le\frac{(n-1)S^2}{\sigma^2}\le b) \quad\forall\theta, \sigma^2\\
    &=P_{\theta,\sigma^2}\left(\frac{(n-1)S^2}{b}\le\sigma^2\le\frac{(n-1)S^2}{a}\right)\\
    &=P_{\theta,\sigma^2}(\sigma^2\in C_{a,b}(\utilde{X}))\\
\end{align*}
$$

with $C_{a,b}(\utilde{X}, S^2)=\left[\frac{(n-1)S^2}{b}, \frac{(n-1)S^2}{a}\right]$ are $1-\alpha$ conf. int. for $\sigma^2$. (Note: $\chi^2$ 不是對稱的)

1. $a=0, b=\chi^2_{n-1,\alpha}\implies C_{a,b}(S^2)=\left[\frac{(n-1)S^2}{\chi^2_{n-1,\alpha}}, \infty\right]$ is $1-\alpha$ UMAU **lower** conf. bound for $\sigma^2$.
2. $a=\chi^2_{n-1,1-\alpha}, b=\infty\implies C_{a,b}(S^2)=\left(0, \frac{(n-1)S^2}{\chi^2_{n-1,1-\alpha}}\right]$ is $1-\alpha$ UMAU **upper** conf. bound for $\sigma^2$.
3. **Usually**, used $a=\chi^2_{n-1,\frac{\alpha}{2}}, b=\chi^2_{n-1,1-\frac{\alpha}{2}}\implies C_{a,b}(S^2)=\left[\frac{(n-1)S^2}{\chi^2_{n-1,1-\frac{\alpha}{2}}}, \frac{(n-1)S^2}{\chi^2_{n-1,\frac{\alpha}{2}}}\right]$ for $\sigma^2$.'

   $\implies$ LRT, equal-tailed test for $H_0:\sigma^2=\sigma^2_0$ vs. $H_1:\sigma^2\neq\sigma^2_0$ (not UMAU)

但對於最佳的 $C_{a,b}(S^2)$，我們同樣要使用 Largrange multiplier method 來解：min $(\frac{1}{a}-\frac{1}{b})$ s.t. $G(b)-G(a)=1-\alpha$ with $G$: cdf of $\chi^2_{n-1}$

$$
\implies\begin{cases}
    -\frac{1}{a^2}=\lambda(-g(a))\\
    \frac{1}{b^2}=\lambda g(b)\\
    G(b)-G(a)=1-\alpha
\end{cases}\implies
\begin{cases}
    a^2g(a)=b^2g(b)\\
    G(b)-G(a)=1-\alpha
\end{cases}
$$

解出來的 $a, b$ 就就能得到 UMAU confidence set。但它只能用電腦得到數值解。

上面找的都是 $\sigma^2$ 的信賴區間，而對於 $\sigma=\sqrt{\sigma^2}$ 

$$
\begin{align*}
    1-\alpha&=P(A\le\sigma^2\le B)\\
    &=P(\sqrt{A}\le\sigma\le\sqrt{B})\\
\end{align*}\implies
\begin{align*}
    1-\alpha&=P_\theta(\theta\in C(\utilde{X}))\\
    &=P_\theta(h(\theta)\in h(C(\utilde{X})))\\
\end{align*}
$$

---

**EX** $X_1,\cdots, X_n\overset{\text{iid}}{\sim}e^{-(x-\theta)}$ with $x\ge\theta$ is a location family $\implies X_{(1)}$ is suff for $\theta$。因此我們用 $X_{(1)}-\theta$ 構建 pivot。

$$
Y=2n(X_{(1)}-\theta)\sim \frac{1}{2}e^{-\frac{y}{2}}, y>0\overset{d}{=}\text{Gamma}(1,2)\overset{d}{=}\chi^2_2\perp\theta
$$

$\implies \forall 0\le a<b$ s.t.

$$
\begin{align*}
    1-\alpha&=\int_a^b\frac{1}{2}e^{-\frac{y}{2}}dy\\
    &=P(a\le Y\le b)\\
    &=P_{\theta}(a\le 2n(X_{(1)}-\theta)\le b)\\
    &=P_{\theta}\left(X_{(1)}-\frac{b}{2n}\le\theta\le X_{(1)}-\frac{a}{2n}\right)\\
    &=P_{\theta}\left(\theta\in C_{a,b}(X_{(1)})\right)\\
\end{align*}
$$

with $C_{a,b}(X_{(1)})=\left[X_{(1)}-\frac{b}{2n}, X_{(1)}-\frac{a}{2n}\right]$ are $1-\alpha$ conf. int. for $\theta$.

為了找到最短的 confidence set，我們要找到最小的 $b-a$ s.t. $G(b)-G(a)=1-\alpha$ with $G$: cdf of $\chi^2_2$。

$$
\begin{cases}
    -1=\lambda(-g(a))\\
    1=\lambda g(b)\\
    G(b)-G(a)=1-\alpha
\end{cases}\implies
g(a)=g(b)\implies a=-b
$$

但注意到 $g(y)=\frac{1}{2}e^{-\frac{y}{2}}$ 是單調遞減的。我們不可能在一個絕對遞減的函數中，得到兩個函數值相同但自變量不同的點。因此最小值發身在邊界點上，即 $a=0, b=\chi^2_{2,\alpha}$

$$
\int_0^b \frac{1}{2}e^{-\frac{y}{2}}dy=1-\alpha\implies 1-e^{-\frac{b}{2}}=1-\alpha\implies b=-2\log(1-\alpha)
$$

i.e. $[X_{(1)}+\frac{2\log(1-\alpha)}{2n}, X_{(1)}]$ is $1-\alpha$ conf. int. for $\theta$. 並且對於假設檢定 $H_0:\theta=\theta_0$ vs. $H_1:\theta\neq\theta_0$，我們會 

$$
\text{reject } H_0\iff\theta_0\notin C_{a,b}(X_{(1)})\iff \theta_0<X_{(1)}-\frac{a}{2n} \text{ or } \theta_0>X_{(1)}-\frac{b}{2n}
$$

事實上這個檢定是 **UMP**，因此這個信賴區間是 **UMA**。

:::danger
信賴區間的長度最短並不代表是 UMA 或 UMAU 的。

要說明一個信賴區間是 UMA 或 UMAU 的，我們需要通過檢定的結果來說明。
:::

**EX** $X_1, \cdots, X_n\overset{\text{iid}}{\sim}U(0, \theta)$. i.e. $f(x;\theta)=\frac{1}{\theta}\cdot I(0\le\frac{x}{\theta}\le 1)$

$T=X_{(n)}$ is suff for $\theta\implies$ use $W=\frac{X_{(n)}}{\theta}\in[0,1]$ as pivot. Let $0\le a<b\le 1$ s.t.

$$
\begin{align*}
    1-\alpha&=P(a\le W\le b)\\
    &=P_\theta(a\le\frac{X_{(n)}}{\theta}\le b)\\
    &=P_\theta(a\theta\le X_{(n)}\le b\theta)=\left(\frac{b\theta}{\theta}\right)^n-\left(\frac{a\theta}{\theta}\right)^n=b^n-a^n\\
    &=P_\theta\left(\frac{X_{(n)}}{b}\le\theta\le\frac{X_{(n)}}{a}\right)\\
\end{align*}
$$

Let $C_{a,b}(X_{(n)})=[\frac{X_{(n)}}{b}, \frac{X_{(n)}}{a}]\implies \forall 0\le a<b\le 1$ with $b^n-a^n=1-\alpha$, $C_{a,b}(X_{(n)})$ are $1-\alpha$ conf. int. for $\theta$.

接下來，為了得到最短的信賴區間，我們要最小化 $\frac{1}{a}-\frac{1}{b}$ s.t. $b^n-a^n=1-\alpha$。而此時 Lagrange multiplier method 就不適用了。

$\because b^n-a^n=1-\alpha\implies a = a(b)$, let $l(b)=\frac{1}{a(b)}-\frac{1}{b}$ 

$$
\begin{align*}
    n\cdot b^{n-1}-n\cdot a^{n-1}\cdot a'(b)&=0\\
    a'(b)&=\frac{b^{n-1}}{a^{n-1}}\\
\end{align*}
\implies 
\begin{align*}
    l'(b)&=-\frac{a'(b)}{a^2}+\frac{1}{b^2}=-\frac{1}{a^2}\frac{b^{n-1}}{a^{n-1}}+\frac{1}{b^2}\\
    &=\frac{a^{n+1}-b^{n+1}}{a^{n+1}b^2}<0
\end{align*}
$$

$\implies l(b) \searrow$ in $b\le 1\implies l(b)$ is min when $b=1\implies a=\alpha^{\frac{1}{n}}$. I.e, the shortest one is $[X_{(n)}, X_{(n)}\alpha^{-\frac{1}{n}}]$.

For testing $H_0:\theta=\theta_0$ vs. $H_1:\theta\neq\theta_0$

$$
\begin{align*}
    \implies\text{reject }H_0 &\iff \theta_0\notin [X_{(n)}, X_{(n)}\alpha^{-\frac{1}{n}}]\\
    &\iff \theta_0<X_{(n)}\text{ or }\theta_0>X_{(n)}\alpha^{-\frac{1}{n}}\\
    &\iff X_{(n)} > \theta_0 \text{ or } X_{(n)} < \theta_0\alpha^{\frac{1}{n}}\\
    &\text{ is UMP level }\alpha\text{ test}
\end{align*}
$$

$\implies [X_{(n)}, X_{(n)}\alpha^{-\frac{1}{n}}]$ is **UMA** $1-\alpha$ confidence set for $\theta$.

---

**EX** $X_1, \cdots, X_n\overset{\text{iid}}{\sim}\frac{1}{\lambda}e^{-\frac{x}{\lambda}}\in$ scale family and $\sum X_i$ is suff for $\lambda$

Note that

$$
\sum_{i=1}^n X_i \sim \text{Gamma}(n, \lambda)\implies \frac{2\sum X_i}{\lambda}\sim\text{Gamma}(n, 2)=\chi^2_{2n}\perp\lambda
$$

$\implies$ Use $\frac{2\sum X_i}{\lambda}\sim \chi^2_{2n}$ as pivot.

$$
\begin{align*}
    \implies 1-\alpha&=P_\lambda(\chi^2_{2n, 1-\frac{\alpha}{2}}\le\frac{2\sum X_i}{\lambda}\le\chi^2_{2n, \frac{\alpha}{2}})\\
    &=P_\lambda\left(\frac{2\sum X_i}{\chi^2_{2n,\frac{\alpha}{2}}}\le\lambda\le\frac{2\sum X_i}{\chi^2_{2n, 1-\frac{\alpha}{2}}}\right)\\
\end{align*}
$$

$\implies \left[\frac{2\sum X_i}{\chi^2_{2n, 1-\frac{\alpha}{2}}}, \frac{2\sum X_i}{\chi^2_{2n, \frac{\alpha}{2}}}\right]$ is $1-\alpha$ conf. int. for $\lambda$.

---

**EX**

$$
\perp\Big<\begin{align*}
    &X_1, \cdots, X_n\overset{\text{iid}}{\sim}\frac{1}{\lambda}e^{-\frac{x}{\lambda}}, x>0\\
    &Y_1, \cdots, Y_m\overset{\text{iid}}{\sim}\frac{1}{\theta}e^{-\frac{y}{\theta}}, y>0
\end{align*}
\quad\implies\quad
\perp\Big<\begin{align*}
    &\frac{2\sum X_i}{\lambda}\sim\chi^2_{2n}\perp\lambda\\
    &\frac{2\sum Y_i}{\theta}\sim\chi^2_{2m}\perp\theta
\end{align*}
$$

$$
\begin{align*}
    &\frac{2\sum X_i}{\lambda(2n)}\Bigg/\frac{2\sum Y_i}{\theta(2m)}\sim F_{2n, 2m}\\
    =&\frac{\bar{X}}{\lambda}\bigg/\frac{\bar{Y}}{\theta}=\frac{\bar{X}}{\bar{Y}}\frac{\theta}{\lambda}
\end{align*}
$$

$$
\begin{align*}
    1-\alpha&=P(F_{2n,2m,1-\frac{\alpha}{2}}\le F_{2n,2m}\le F_{2n,2m,\frac{\alpha}{2}})\\
   &=P_{\theta,\lambda}\left(F_{2n,2m,1-\frac{\alpha}{2}}\le\frac{\bar{X}}{\bar{Y}}\frac{\theta}{\lambda}\le F_{2n,2m,\frac{\alpha}{2}}\right)\\
   &=P_{\theta,\lambda}\left(\frac{\bar{Y}}{\bar{X}}F_{2n,2m,1-\frac{\alpha}{2}}\le\frac{\theta}{\lambda}\le\frac{\bar{Y}}{\bar{X}}F_{2n,2m,\frac{\alpha}{2}}\right)\\
\end{align*}
$$

$\implies \left[\frac{\bar{Y}}{\bar{X}}F_{2n,2m,1-\frac{\alpha}{2}}, \frac{\bar{Y}}{\bar{X}}F_{2n,2m,\frac{\alpha}{2}}\right]$ is $1-\alpha$ conf. int. for $\frac{\theta}{\lambda}$.

$\implies$ To test $H_0:\lambda=\theta$ v.s. $H_1:\lambda\neq\theta$ $\iff$ $H_0:\frac{\theta}{\lambda}=1$ v.s. $H_1:\frac{\theta}{\lambda}\neq 1$  

Reject $H_0$ if $1\notin\left[\frac{\bar{Y}}{\bar{X}}F_{2n,2m,1-\frac{\alpha}{2}}, \frac{\bar{Y}}{\bar{X}}F_{2n,2m,\frac{\alpha}{2}}\right]$

---

**EX**

$$
\Big < \begin{align*}
    &X_1, \cdots, X_n\overset{\text{iid}}{\sim}N(\mu_x, \sigma^2)\\
    &Y_1, \cdots, Y_m\overset{\text{iid}}{\sim}N(\mu_y, \sigma^2)
\end{align*}
$$

Note that $S_p^2\triangleq\frac{(n-1)S^2_X+(m-1)S^2_Y}{n+m-2}$ is the best est for $\sigma^2$ and

$$
\frac{\bar{X}-\bar{Y}-(\mu_x-\mu_y)}{\sqrt{(\frac{1}{n}+\frac{1}{m})S_p^2}}\sim t_{n+m-2}\perp(\mu, \sigma^2)
$$

$\implies [(\bar{X}-\bar{Y})\pm t_{n+m-2,\frac{\alpha}{2}}\sqrt{(\frac{1}{n}+\frac{1}{m})S_p^2}]$ is $1-\alpha$ conf. int. for $\mu_x-\mu_y$.

To test $H_0:\mu_x=\mu_y$ vs. $H_1:\mu_x\neq\mu_y$, reject $H_0$ if $0\notin\left[(\bar{X}-\bar{Y})\pm t_{n+m-2,\frac{\alpha}{2}}\sqrt{(\frac{1}{n}+\frac{1}{m})S_p^2}\right]$ is UMAP test.

$\implies \left[(\bar{X}-\bar{Y})\pm t_{n+m-2,\frac{\alpha}{2}}\sqrt{(\frac{1}{n}+\frac{1}{m})S_p^2}\right]$ is UMAU $1-\alpha$ conf. int. for $\mu_x-\mu_y$.

## Asymptotic

這個方法會讓 $n\to\infty$，因此也稱為 **Large Sample method**。

As $n\to\infty$, if $\frac{W(\utilde{x})-\theta}{V(\utilde{X})}\xrightarrow{D}N(0,1)$, i.e.

$$
P\left(a\le\frac{W(\utilde{X})-\theta}{V(\utilde{X})}\le b\right)\approx P\left(a\le Z\le b\right)
$$

$$
\begin{align*}
    \text{i.e. }1-\alpha&\approx P_\theta\left(-Z_{\frac{\alpha}{2}}\le\frac{W(\utilde{X})-\theta}{V(\utilde{X})}\le Z_{\frac{\alpha}{2}}\right)\\
    &=P_\theta\left(W(\utilde{X})-Z_{\frac{\alpha}{2}}V(\utilde{X})\le\theta\le W(\utilde{X})+Z_{\frac{\alpha}{2}}V(\utilde{X})\right)\\
    &=P_\theta\left(\theta\in [W(\utilde{X})\pm Z_{\frac{\alpha}{2}}V(\utilde{X})]\right)\quad \forall\theta
\end{align*}
$$

$\implies [W(\utilde{X})\pm Z_{\frac{\alpha}{2}}V(\utilde{X})]$ is **approximated** $1-\alpha$ conf. int. for $\theta$.

**Remark**: 當有確定的樣本數時，永遠都是推導**精確**的信賴區間。只有當被要求時，才用 approximated 的信賴區間。


:::info[Theorem]
**C.L.T(中央極限定理)**

$X_1,\cdots,X_n$ are iid with $E(X_i)=\mu, Var(X_i)\in(0, \infty)$, let $\bar{X}=\frac{1}{n}\sum_{i=1}^n X_i$, then

$$
\frac{\sqrt{n}(\bar{X}-\mu)}{\sigma}\xrightarrow[n\to\infty]{D}N(0,1)
$$


$$
\text{i.e. }P\left(\frac{\sqrt{n}(\bar{X}-\mu)}{\sigma}\le t\right)\approx\Phi(t) \quad \forall t\text{ for large enough } n
$$
:::

$$
\begin{align*}
    &\implies \text{ if }\sigma^2\text{ known}\\
    &\implies P\left(-Z_{\frac{\alpha}{2}}\le\frac{\sqrt{n}(\bar{X}-\mu)}{\sigma}\le Z_{\frac{\alpha}{2}}\right)\approx 1-\alpha\\
    &\iff P_\mu\left(\mu\in[\bar{X}-\frac{\sigma}{\sqrt{n}}Z_{\frac{\alpha}{2}}, \bar{X}+\frac{\sigma}{\sqrt{n}}Z_{\frac{\alpha}{2}}]\right)\approx 1-\alpha\quad\forall\mu
\end{align*}
$$
i.e. $[\bar{X}\pm\frac{\sigma}{\sqrt{n}}Z_{\frac{\alpha}{2}}]$ is approximated $1-\alpha$  conf. int. for $\mu$
$$
\begin{align*}
    \text{Also } &P_\mu\left(\frac{\sqrt{n}(\bar{X}-\mu)}{\sigma}\le Z_{\alpha}\right)=P_\mu\left(\mu\ge\bar{X}-\frac{\sigma}{\sqrt{n}}Z_{\alpha}\right)\\
    \approx& P(Z\le Z_{\alpha})=1-\alpha\quad\forall\mu
\end{align*}
$$

i.e. $[\bar{X}-\frac{\sigma}{\sqrt{n}}Z_{\alpha}, \infty)$ is approximated $1-\alpha$ conf lower bound for $\mu$.

但以上結論都是在 $\sigma^2$ 已知的情況下。如果 $\sigma^2$ 未知，上面的結論就是錯的。

:::info[Theorem]
**Slutsky's Theorem**

1. r.v.'s $Y_n\xrightarrow[n\to\infty]{D}W$ : r.v., i.e. $P(Y_n\le t)\xrightarrow[n\to\infty]{}P(W\le t)$ $\forall t$ with $F_w$: conti at $t$.
2. r.v. $T_n\xrightarrow[n\to\infty]{P}c$ : const

$$
\implies T_nY_n\xrightarrow[n\to\infty]{D}cW
$$
:::

**EX** $X_1,\cdots, X_n$ are iid with $E(X_i)=\mu, Var(X_i)=\sigma^2\in(0, \infty)$

If $\sigma^2$ unknown

$$
\frac{\sqrt{n}(\bar{X}-\mu)}{S}=\frac{\frac{\sqrt{n}(\bar{X}-\mu)}{\sigma}\xrightarrow{D}N(0,1)}{\frac{S}{\sigma}\xrightarrow{P}1} \xrightarrow{D}N(0,1)
$$

i.e. If $\sigma^2$ unknown

- $[\bar{X}\pm\frac{S}{\sqrt{n}}Z_{\frac{\alpha}{2}}]$ is approximated $1-\alpha$ conf. int. for $\mu$
- $[\bar{X}-\frac{S}{\sqrt{n}}Z_{\alpha}, \infty)$ is approximated $1-\alpha$ conf lower bound for $\mu$

---

**EX** $X_1, \cdots, X_n\overset{\text{iid}}{\sim}B(1, p)$ with $E(X_i)=p, Var(X_i)=p(1-p)$

$$
\text{By C.L.T}\qquad\frac{\sqrt{n}(\bar{X}-p)}{\sqrt{p(1-p)}}\xrightarrow{D}N(0,1)
$$

但其中分母部分有 $p$，這難以湊成 $[L(\utilde{X}),U(\utilde{X})]$。

$$
\text{L.L.N }\bar{X}\xrightarrow{P}p\implies\bar{X}\xrightarrow{P}p\implies\sqrt{\bar{X}(1-\bar{X})}\xrightarrow{P}\sqrt{p(1-p)}
$$

$\implies \left[\bar{X}\pm Z_{\frac{\alpha}{2}}\sqrt{\bar{X}(1-\bar{X})}/\sqrt{n}\right]$ is approximated $1-\alpha$ conf. int. for $p$.

**Recall**: $\forall a_n\to 1, b_n\to 0$, $a_n\bar{X}+b_n\xrightarrow{P}p$ , i.e. 答案並不唯一。

$$
\begin{align*}
    S^2&=\frac{1}{n-1}\sum_{i=1}^n(X_i-\bar{X})^2\\
    &=\frac{1}{n-1}\sum_{i=1}^nX_i^2-\frac{n}{n-1}\bar{X}^2\\
    &=\frac{1}{n-1}\sum_{i=1}^nX_i-\frac{n}{n-1}\bar{X}^2\quad X_i\in\{0,1\}\text{ in Bernoulli}\\
    &=\frac{n}{n-1}\bar{X}-\frac{n}{n-1}\bar{X}^2\\
    &=\frac{n}{n-1}\bar{X}(1-\bar{X})\\
\end{align*}
$$

$\implies\left[\bar{X}\pm \frac{\sqrt{\bar{X}(1-\bar{X})}}{\sqrt{n-1}}Z_{\frac{\alpha}{2}}\right]$ is also approximated $1-\alpha$ conf. int. for $p$.

**Remark**: 如果 $\bar{X}=1$ or $0$ 時，按照以上方法計算出的信賴區間會只有單點。在實際應用中，可能會偏離正確的信賴區間。

---

**EX** $X_1, \cdots, X_n\overset{\text{iid}}{\sim}P(\lambda)$ with $E(X_i)=\lambda=Var(X_i)$. 我們可以用 $\bar{X}$ 或 $S^2$ 來估計 $\lambda$。

$\implies \left[\bar{X}\pm Z_{\frac{\alpha}{2}}\sqrt{\frac{\bar{X}\text{ or }S^2}{n}}\right]$ are approximated $1-\alpha$ conf. int. for $\lambda$.

我們還可以直接假設區間 $C(\utilde{X})=\set{\lambda: -Z_{\frac{\alpha}{2}}\le\frac{\sqrt{n}(\bar{X}-\lambda)}{\sqrt{\lambda}}\le Z_{\frac{\alpha}{2}}}$，然後分別解出左右兩個不等式。這個區間相比於上面的區間更精確，因為少了一個 $\lambda$ 的估計。

### $\delta$-method

如果我們關注參數 $\mu$ ,可以利用 C.L.T 得到 $\frac{\sqrt{n}(\bar{X}-\mu)}{\sigma}\xrightarrow{D}N(0,1), S\xrightarrow{P}\sigma$，進而得到 $\mu$ 的信賴區間。但如果我們關注的是 $g(\mu)$ ，我們則會使用 $\delta$-method。

:::info[Theorem]
$\delta$-method: As $n\to\infty$, $\forall g$ with $g'$ exists and $g'(\mu)\neq 0$

If $\sqrt{n}(T_n-\mu)\xrightarrow{D}N(0, \sigma^2)$

then $\sqrt{n}(g(T_n)-g(\mu))\xrightarrow{D}N(0, \sigma^2\cdot[g'(\mu)]^2)$
:::

**EX** $X_1,\cdots, X_n\overset{\text{iid}}{\sim}B(1,p)$

我們關心 $\frac{p}{1-p}$。（如果 $p$ 是遊戲的勝率，那麼 $\frac{p}{1-p}$ 就是勝算(odds)）

$$
\begin{align*}
    \text{Let }g(p)&=\log\left(\frac{p}{1-p}\right)\triangleq \text{logit}(p)\\
    &=\log(p)-\log(1-p)\\
\end{align*}
$$

$$
\implies g'(p)=\frac{1}{p}-\frac{1}{1-p}=\frac{1}{p(1-p)}
$$

$$
\begin{align*}
    \frac{\sqrt{n}(\bar{X}-p)}{\sqrt{p(1-p)}}\xrightarrow{D}N(0,1) &\iff\sqrt{n}(\bar{X}-p)\xrightarrow{D}N(0, p(1-p))\\
    \delta\text{-methpd with }g(p)=\log(\frac{p}{1-p})&\implies \sqrt{n}(g(\bar{X})-g(p))\xrightarrow{D}N(0, [p(1-p)]\cdot[\frac{1}{p(1-p)}]^2)\\
\end{align*}
$$

$$
\begin{align*}
    \text{i.e.} &\sqrt{n}\left(\log\left(\frac{T}{n-T}\right)-\log\left(\frac{p}{1-p}\right)\right)\xrightarrow{D}N(0, 1)\qquad\text{with }T=\sum X_i\\
    \iff &\frac{\sqrt{n}\left(\log\left(\frac{T}{n-T}\right)-\log\left(\frac{p}{1-p}\right)\right)}{\sqrt{\frac{1}{p(1-p)}}}\xrightarrow{D}N(0, 1)\\
    \iff &\frac{\log\left(\frac{T}{n-T}\right)-\log\left(\frac{p}{1-p}\right)}{\sqrt{\frac{1}{n}\left(\frac{1}{p}+\frac{1}{1-p}\right)}}\xrightarrow{D}N(0, 1)\\
\end{align*}
$$

分母部分仍有 $p$，我們可以用 $\bar{X}$ 來估計 $p$，並且因為 **Slutsky's Theorem**，收斂性不會受到影響。

$$
\begin{align*}
    \implies& \frac{\log\left(\frac{T}{n-T}\right)-\log\left(\frac{p}{1-p}\right)}{\sqrt{\frac{1}{\cancel{n}}\left(\frac{\cancel{n}}{T}+\frac{\cancel{n}}{n-T}\right)}}\xrightarrow{D}N(0, 1)\\
    \implies& P_p\left(\log\left(\frac{p}{1-p}\right)\in\left[\log\left(\frac{T}{n-T}\right)\pm Z_{\frac{\alpha}{2}}\sqrt{\frac{1}{T}+\frac{1}{n-T}}\right]\right)\approx 1-\alpha
\end{align*}
$$

$\implies$ $[\log\left(\frac{T}{n-T}\right)\pm Z_{\frac{\alpha}{2}}\sqrt{\frac{1}{T}+\frac{1}{n-T}}]$ is approximated $1-\alpha$ conf. int. for $\log\left(\frac{p}{1-p}\right)$

$\implies$ $\left[\exp\left(\log\left(\frac{T}{n-T}\right)\pm Z_{\frac{\alpha}{2}}\sqrt{\frac{1}{T}+\frac{1}{n-T}}\right)\right]$ is approximated $1-\alpha$ conf. int. for $\frac{p}{1-p}$


因為在 $B(1,p)$ 中，$\sum X_i=T$ 就会是成功的次数，$n-T$ 则是失败的次数。因此 $\frac{T}{n-T}$ 就是成功的次数与失败的次数的比值，即 $\frac{p}{1-p}$。

---

for large $n$

$$
\begin{align*}
    &\log\left(\frac{T_x}{n-T_x}\right)-\log\left(\frac{p_1}{1-p_1}\right)\approx \sqrt{\frac{1}{T_x}+\frac{1}{n-T_x}}Z_1\\
    &\log\left(\frac{T_y}{m-T_y}\right)-\log\left(\frac{p_2}{1-p_2}\right)\approx \sqrt{\frac{1}{T_y}+\frac{1}{m-T_y}}Z_2
\end{align*}\qquad\text{with }Z_1, Z_2\sim N(0,1) \text{ iid}
$$


$$
\begin{align*}
    \log\left(\frac{T_x}{n-T_x}\frac{m-T_y}{T_y}\right)-\log\left(\frac{p_1}{1-p_1}\frac{p_2}{1-p_2}\right)&\approx \sqrt{\frac{1}{T_x}+\frac{1}{n-T_x}}Z_1+\sqrt{\frac{1}{T_y}+\frac{1}{m-T_y}}Z_2\\
    &=N(0,\frac{1}{T_x}+\frac{1}{n-T_x}+\frac{1}{T_y}+\frac{1}{m-T_y})
\end{align*}
$$

如果我們關注的是勝算比（odds ratio）$\frac{p_1}{1-p_1}\big/\frac{p_2}{1-p_2}\triangleq\theta$

$$
\implies P_{p_1,p_2}\left(\log(\theta)\in\left[\log\left(\frac{T_x}{n-T_x}\frac{m-T_y}{T_y}\right)\pm Z_{\frac{\alpha}{2}}\sqrt{\frac{1}{T_x}+\frac{1}{n-T_x}+\frac{1}{T_y}+\frac{1}{m-T_y}}\right]\right)\approx 1-\alpha
$$

|     | 正面  | 反面    | 總和 |
| --- | ----- | ------- | ---- |
| X   | $T_x$ | $n-T_x$ | n    |
| Y   | $T_y$ | $m-T_y$ | m    |

---

**EX**：對於阿司匹林是否能預防心臟病這一問題收集到以下數據：

| 是否有心臟病 | 有  | 沒有  |
| ------------ | --- | ----- |
| 阿司匹林     | 104 | 10933 |
| 安慰劑       | 189 | 10845 |

我們關心 odds ratio $\theta=\frac{p_1}{1-p_1}\big/\frac{p_2}{1-p_2}$，其中 $p_2$ 是阿司匹林組的心臟病發生率，$p_1$ 是安慰劑組的心臟病發生率。

$$
\begin{align*}
    &\implies \left[\ln\left(\frac{189\times 10933}{10845\times 104}\right)\pm 1.96\sqrt{\frac{1}{189}+\frac{1}{10845}+\frac{1}{104}+\frac{1}{10933}} \right]=[0.365, 0.846]\text{ is } 95\%\text{ conf. int. for }\ln(\theta)\\
    &\implies \left[e^{0.365}, e^{0.846}\right]=[1.44, 2.33]\text{ is } 95\%\text{ conf. int. for }\theta
\end{align*}
$$

如果阿司匹林明顯有預防心臟病的效果，那麼 $p_2$ 應該顯著小於 $p_1$；如果沒有效果，那麼吃阿司匹林或安慰劑的產生心臟病的幾率應該相同。因此做檢定：

$$
\begin{align*}
    & H_0: p_1=p_2\text{ vs. }H_1: p_1>p_2\\
    \iff& H_0: \theta=1\text{ vs. }H_1: \theta>1\\
\end{align*}
$$

$1\notin [1.44, 2.33]\implies$ reject $H_0$ at $5\%$ level. I.e. 阿司匹林在預防心臟病方面有顯著效果。

---

**EX**:

$$
\perp\Big<\begin{align*}
    &X\sim B(n,p)\\
    &Y\sim B(m,p)
\end{align*}\qquad\theta=\frac{p_1(1-p_2)}{(1-p_1)p_2}
$$

將獲得的 Binomal 數據整理成以下 $2\times 2$ 列聯表（contingency table）

| Data | S         | F               | Total |
| ---- | --------- | --------------- | ----- |
|      | $x_{p_1}$ | $n-x_{(1-p_1)}$ | n     |
|      | $y_{p_2}$ | $m-x_{(1-p_2)}$ | m     |

$$
\begin{align*}
    &\ln\theta\in \left[\ln\left(\frac{x(m-y)}{y(n-x)}\pm Z_{\frac{\alpha}{2}}\sqrt{\frac{1}{x}+\frac{1}{n-x}+\frac{1}{y}+\frac{1}{m-y}} \right)\right]=[A,B] \text{ with conf }\approx 1-\alpha\\
    \implies& \theta\in [e^A, e^B] \text{ with conf }\approx 1-\alpha
\end{align*}
$$

**Remark**：這個結論對於從 multinomial / Poisson 獲得樣本所組成的列聯表也是成立的。

---

**EX**:

收集到以下數據。A, B 是關心的兩個不同的事件（e.g. 天氣、路口），中間表格則是交集事件發生的次數，以及發生幾率。這組數據服從 multinomial distribution。

|       | $B_1$            | $B_2$            |
| ----- | ---------------- | ---------------- |
| $A_1$ | $X_{11}, P_{11}$ | $X_{12}, P_{12}$ |
| $A_2$ | $X_{21}, P_{21}$ | $X_{22}, P_{22}$ |

$\sum_{i=1}^2\sum_{j=1}^2 X_{ij}=n\quad \sum_{i=1}^2\sum_{j=1}^2 P_{ij}=1$

$$
\begin{align*}
    &\utilde{X}=(X_{11}, X_{12}, X_{21}, X_{22})\\
    &\utilde{P}=(P_{11}, P_{12}, P_{21}, P_{22})
\end{align*}\sim \text{Multinomial}_n(k, \utilde{P})\quad k=\text{ col }\times \text{ row }=4
$$

$$
\theta\triangleq \frac{P_{11}P_{22}}{P_{12}P_{21}}: \text{ odds ratio}
$$

如果 $A\perp B$，那麼不管在 $A_1$ 或 $A_2$ 條件下，$B_1$ 和 $B_2$ 的發生概率應該是不變的。因此

$$
\begin{align*}
    & H_0: A\perp B\\
    \iff& H_0: \frac{P_{11}}{P_{12}}=\frac{P_{21}}{P_{22}}\\
    \iff& H_0: \frac{P_{11}P_{22}}{P_{12}P_{21}}=\theta=1\\
    \iff& H_0: \ln\theta=0
\end{align*}
$$

$$
\begin{align*}
    &\implies \ln\theta\in \left[\ln\left(\frac{X_{11}X_{22}}{X_{12}X_{21}}\right)\pm Z_{\frac{\alpha}{2}}\sqrt{\frac{1}{X_{11}}+\frac{1}{X_{12}}+\frac{1}{X_{21}}+\frac{1}{X_{22}}}\right]=[A,B] \text{ with conf }\approx 1-\alpha\\
    &\implies \theta\in [e^A, e^B] \text{ with conf }\approx 1-\alpha
\end{align*}
$$

廣泛來說，$\utilde{X}\sim \text{Multinomial}_n(k,\utilde{P})$ with $k=I\times J$ 可以做成一張 $I\times J$ 的列聯表

| A\B      | $B_1$    | $B_2$    | $\cdots$ | $B_J$    |
| -------- | -------- | -------- | -------- | -------- |
| $A_1$    | $X_{11}$ | $X_{12}$ | $\cdots$ | $X_{1J}$ |
| $A_2$    | $X_{21}$ | $X_{22}$ | $\cdots$ | $X_{2J}$ |
| $\vdots$ | $\vdots$ | $\vdots$ | $\ddots$ | $\vdots$ |
| $A_I$    | $X_{I1}$ | $X_{I2}$ | $\cdots$ | $X_{IJ}$ |

而關注的問題通常是 $H_0:A\perp B$ v.s. $H_1:A\not\perp B$。進一步可能會關注關鍵的地方在哪，因此會在 A, B 中各挑選 2 個事件組成 $2\times 2$ 的列聯表，並計算 odds ratio 的信賴區間。

### Multivariate

#### C.L.T

$$
\utilde{X}_1, \cdots, \utilde{X}_n \text{ iid with } E(\utilde{X}_i)=\utilde{\theta}, Var(\utilde{X}_i)=\bcancel{\utilde{\Sigma}}
$$

如果每個隨機向量 $\utilde{X}_i$ 的維度都是 $m$，那麼 $\bar{X}=\frac{1}{n}\sum\utilde{X}_i$ 也會是一個 $m$ 維的隨機向量。

$$
\implies \sqrt{n}(\bar{X}-\utilde{\theta})\xrightarrow[n\to\infty]{D}N_m(\utilde{0}, \bcancel{\utilde{\Sigma}})
$$

**EX**: $\utilde{Y}\sim \text{Multinomial}_n(k,\utilde{P})$

|         | 1     | 2     | $\cdots$ | k     | total |
| ------- | ----- | ----- | -------- | ----- | ----- |
| data    | $Y_1$ | $Y_2$ | $\cdots$ | $Y_k$ | n     |
| w) prob | $P_1$ | $P_2$ | $\cdots$ | $P_k$ | 1     |

$\implies$ est $P_j$ by $\frac{Y_j}{n}$, i.e. $\utilde{\hat{P}}=\frac{\utilde{Y}}{n}$

如果令 $\utilde{X}_i$ 是第 $i$ 次丟一個球的結果。因為一個球只能進一個籃子裡，所以每個 $\utilde{X}_i$ 向量中只有一個元素是 1，其他都是 0。

如果單看 $\utilde{X}_i$ 中的每個 $X_{ij}$，那麼 $X_{ij}\sim B(1,P_j)$ with $E[X_{ij}\cdot X_{ij'}]=0\quad Var(X_{ij})=P_j(1-P_j)\implies\forall j\neq j'\quad Cov(X_{ij},X_{ij'})=-P_jP_{j'}$

I.e. $\utilde{X}_i\sim \text{Multinomial}_1(k, \utilde{P})$ with

$$
\begin{align*}
    E(X_i)=\utilde{P}\quad \sigma^2\set{\utilde{X}_i}&=\begin{bmatrix}
      P_1(1-P_1) & -P_1P_2 & \cdots & -P_1P_k\\
      -P_2P_1 & P_2(1-P_2) & \cdots & -P_2P_k\\
      \vdots & \vdots & \ddots & \vdots\\
      -P_kP_1 & -P_kP_2 & \cdots & P_k(1-P_k)
  \end{bmatrix}\\
  &=\text{diag}(\utilde{P})-\utilde{P}\utilde{P}^t=\bcancel{\Sigma}
\end{align*}
$$

$$
\implies \utilde{Y}=\sum \utilde{X}_i\implies \utilde{\hat{P}}=\frac{\utilde{Y}}{n}=\bar{X}
$$

$$
\xRightarrow{\text{C.L.T}} \sqrt{n}(\utilde{\hat{P}}-\utilde{P})\xrightarrow{D}N_m(\utilde{0}, \bcancel{\Sigma})
$$

#### $\delta$-method

$$
\begin{align*}
    &\sqrt{n}(\bar{X}-\utilde{\theta})\xrightarrow{D}N_m(\utilde{0}, \bcancel{\Sigma})\\
    \implies& \sqrt{n}(g(\bar{X})-g(\utilde{\theta}))\xrightarrow{D}N_m(\utilde{0},\nabla g(\utilde{\theta})^t\bcancel{\Sigma}\nabla g(\utilde{\theta}))
\end{align*}
$$

$$
\forall g:\R^m\to \R \quad\text{with } \nabla g(\utilde{\theta})=\begin{pmatrix}
    \frac{\partial g}{\partial \theta_1}(\utilde{\theta})\\
    \vdots\\
    \frac{\partial g}{\partial \theta_m}(\utilde{\theta})
\end{pmatrix}\text{ exists and }\neq \utilde{0}
$$

---

$$
k\ge 4\quad \sqrt{n}(\utilde{\hat{P}}-\utilde{P})\xrightarrow{D}N_k(\utilde{0}, \bcancel{\Sigma})\quad \bcancel{\Sigma}=\text{diag}(\utilde{P})-\utilde{P}\utilde{P}^t
$$

$$
\begin{align*}
    \text{Take}&\quad g(\utilde{P})=\ln\left(\frac{P_1P_2}{P_3P_4}\right)=\ln(P_1) + \ln(P_2) - \ln(P_3) - \ln(P_4)\\
    \implies& \nabla g(\utilde{P})=(\frac{1}{P_1}, \frac{1}{P_2}, -\frac{1}{P_3}, -\frac{1}{P_4}, 0, \cdots, 0)^t: k\times 1
\end{align*}
$$

$$
\begin{align*}
    &(\nabla g(\utilde{P}))^t\bcancel{\Sigma}\nabla g(\utilde{P})\\
    =&(\nabla g(\utilde{P}))^t\left(\text{diag}(\utilde{P})-\utilde{P}\utilde{P}^t\right)\nabla g(\utilde{P})\\
    =&(\nabla g(\utilde{P}))^t\text{diag}(\utilde{P})\nabla g(\utilde{P}) - (\nabla g(\utilde{P}))^t\utilde{P}\utilde{P}^t\nabla g(\utilde{P})\\
    =&\sum_{i=1}^kP_i(\nabla g(\utilde{P}))_i^2 - \left[(\nabla g(\utilde{P}))^t \utilde{P} \right]^2\\
    =&\frac{1}{P_1} + \frac{1}{P_2} + \frac{1}{P_3} + \frac{1}{P_4} - (1+1-1-1+0+\cdots+0)^2\\
    =&\frac{1}{P_1} + \frac{1}{P_2} + \frac{1}{P_3} + \frac{1}{P_4}
\end{align*}
$$

$$
\begin{align*}
    \xRightarrow{\delta\text{-method}}& \sqrt{n}(g(\utilde{\hat{P}})-g(\utilde{P}))\xrightarrow{D}N\left(0, \frac{1}{P_1} + \frac{1}{P_2} + \frac{1}{P_3} + \frac{1}{P_4}\right)\\
    \implies& \frac{g(\utilde{\hat{P}})-g(\utilde{P})}{\sqrt{\frac{1}{n}\left(\frac{1}{P_1}+\frac{1}{P_2}+\frac{1}{P_3}+\frac{1}{P_4} \right)}} \xrightarrow{D}N(0,1)\\
    \xRightarrow{\text{slutsky}}& \frac{g(\utilde{\hat{P}})-g(\utilde{P})}{\sqrt{\frac{1}{Y_1}+\frac{1}{Y_2}+\frac{1}{Y_3}+\frac{1}{Y_4}}} \xrightarrow{D}N(0,1)\quad \hat{P}_j=\frac{Y_j}{n}\\
    \implies& \frac{\ln\left(\frac{Y_1Y_2}{Y_3Y_4} \right)-\ln\left(\frac{P_1P_2}{P_3P_4} \right)}{\sqrt{\frac{1}{n}\left(\frac{1}{P_1}+\frac{1}{P_2}+\frac{1}{P_3}+\frac{1}{P_4} \right)}} \xrightarrow{D}N(0,1)
\end{align*}
$$

因此如果 $\theta=\ln\frac{P_1P_2}{P_3P_4}$ 是我們關心的參數

$$
P\left(\theta\in\left[\ln\left(\frac{Y_1Y_2}{Y_3Y_4} \right)\pm Z_{\frac{\alpha}{2}}\sqrt{\frac{1}{Y_1}+\frac{1}{Y_2}+\frac{1}{Y_3}+\frac{1}{Y_4}} \right] \right)\approx 1-\alpha\quad\text{for large enough }n
$$

### Sample size determination with Binomial sample

我們通常會在收集資料之前就定下目標，然後為了驗證目標而收集資料。因此我們會需要知道需要多少樣本才能驗證是否達到目標。而不是收集資料後發現樣本數不夠達到足夠的信心水平。

$X_1,\cdots, X_n\overset{\text{iid}}{\sim}B(1,p)$

$$
\xRightarrow{\text{C.L.T}} p\in \left[\bar{X}\pm \frac{\sqrt{\bar{X}(1-\bar{X})}}{\sqrt{n}}Z_{\frac{\alpha}{2}} \right] \text{ with conf }\approx 1-\alpha
$$

$$
\begin{align*}
    \text{i.e. }\forall p\in(0,1)\quad 1-\alpha&\approx P_p\left(\bar{X}-\frac{\sqrt{\bar{X}(1-\bar{X})}}{\sqrt{n}}Z_{\frac{\alpha}{2}}\le p\le\bar{X}+\frac{\sqrt{\bar{X}(1-\bar{X})}}{\sqrt{n}}Z_{\frac{\alpha}{2}} \right)\\
    &=P_p\left(|\bar{X}-p|\le\frac{\sqrt{\bar{X}(1-\bar{X})}}{\sqrt{n}}Z_{\frac{\alpha}{2}} \right)
\end{align*}
$$

其中 $\bar{X}$ 是 $p$ 的估計。意思是在 $1-\alpha$ 的信心水平下，我們需要確保 $\bar{X}$ 與 $p$ 的誤差不超過某個範圍 $E$。

因為這是在收集數據之前確定要收集的樣本量，因此我們還不知道 $\bar{X}$ 的值。但我們知道 $\bar{X}\in[0,1]$ 並且 $\sqrt{\bar{X}(1-\bar{X})}\le \frac{1}{2}$。因此我們可以用這個上界來計算一個保守的樣本量。

$$
\begin{align*}
    &\sqrt{\frac{\bar{X}(1-\bar{X})}{n}} Z_{\frac{\alpha}{2}}\le \frac{1}{2}Z_{\frac{\alpha}{2}}=E\\
    \iff& \sqrt{n}=\frac{1}{2E}Z_{\frac{\alpha}{2}}\\
    \iff& n=\left(\frac{1}{2E}Z_{\frac{\alpha}{2}}\right)^2
\end{align*}
$$

e.g. $\alpha=0.05, E=0.03\implies n\ge \frac{(1.96)^2}{4\cdot(0.03)^2}=1068$