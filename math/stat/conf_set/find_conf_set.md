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
   &=P_{\theta,\lambda}\left(\frac{\bar{Y}}{\bar{X}}F_{2n,2m,1-\frac{\alpha}{2}}\le\frac{\lambda}{\theta}\le\frac{\bar{Y}}{\bar{X}}F_{2n,2m,\frac{\alpha}{2}}\right)\\
\end{align*}
$$

$\implies \left[\frac{\bar{Y}}{\bar{X}}F_{2n,2m,1-\frac{\alpha}{2}}, \frac{\bar{Y}}{\bar{X}}F_{2n,2m,\frac{\alpha}{2}}\right]$ is $1-\alpha$ conf. int. for $\frac{\lambda}{\theta}$.

$\implies$ To test $H_0:\lambda=\theta$ v.s. $H_1:\lambda\neq\theta$ $\iff$ $H_0:\frac{\lambda}{\theta}=1$ v.s. $H_1:\frac{\lambda}{\theta}\neq 1$  

Reject $H_0$ if $1\notin\left[\frac{\bar{Y}}{\bar{X}}F_{2n,2m,1-\frac{\alpha}{2}}, \frac{\bar{Y}}{\bar{X}}F_{2n,2m,\frac{\alpha}{2}}\right]$

## Asymptotic