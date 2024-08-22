---
sidebar_position: 2
---

# 複合假設（Composite Hypothese）

## 單邊問題（One-Sided Problem）

在做 simple $H_0$ 和 simple $H_1$ 的檢定時，N-P Lemma 可以直接給我們一個 MP 檢定。而它還可以幫我們找到某些 composite $H_0$ 和 composite $H_1$ 的 UMP 檢定。

對於 $H_0: \theta\in\omega_0$ v.s. $H_1: \theta\in\omega_1$。因爲 N-P lemma 告訴我們，在 level $\alpha$ 下找到的 MP test，對於  level $<\alpha$ 時也會是 MP test。那麼如果還有 $\forall \theta\in\omega_1$ 可以得到同一個 $\phi$，那麼就可以把複雜的檢定變成簡單的檢定，並使用 N-P Lemma。

記得 $f(\utilde{X};\theta)=g(T:\theta)h(\utilde{X})$ with $T=T(\utilde{X})$ 是 $\theta$ 的 sufficient statistic，並且我們可以讓 $g(T;\theta)$ 是 pdf。

因此如果一個滿足 N-P Lemma 的檢定函數，i.e. 拒絕 $H_0\iff f(\utilde{x};\theta_1)>cf(\utilde{x};\theta_0)\iff g(t;\theta_1)>cg(t;\theta_0)$

---

**EX**: $X_1, \cdots, X_n \overset{\text{iid}}{\sim}N(\theta, \sigma^2_0)$

$$
H_0:\theta=\theta_0 \text{ v.s. } H_1:\theta=\theta_1
$$

$\implies$ MP level $\alpha$ 檢定是：拒絕 $H_0$ if $\frac{\bar{X}-\theta_0}{\sigma_0/\sqrt{n}}>Z_\alpha$。這對於任何 $\theta_1>\theta_0$ 都適用。

注意到

$$
\begin{align*}
    &E_\theta\phi(\utilde{X})=P_\theta\left(\frac{\bar{X}-\theta_0}{\sigma_0/\sqrt{n}}>Z_\alpha\right)=1-\Phi(\frac{\sqrt{n}}{\sigma^2}(\theta_0-\theta)+Z_\alpha)\\
    \implies& \frac{d}{d\theta} E_\theta\phi(\utilde{X})=\frac{\sqrt{n}}{\sigma_0}\phi(\frac{\sqrt{n}}{\sigma_0}(\theta_0-\theta)+Z_\alpha)>0\\
    \implies& \sup_{\theta\le\theta_0}E_\theta \phi(\utilde{X})=E_{\theta_0}\phi(\utilde{X})=\alpha\\
    & \text{i.e. }\phi\text{ is level }\alpha\text{ test for }H^*_0:\theta\le\theta_0\\
    \implies& \text{For testing }H_0:\theta\le\theta_0\text{ v.s. }H_1:\theta>\theta_0\text{, we can use the same }\phi
\end{align*}
$$

:::tip[Theorem]
$\utilde{X}\sim f(\utilde{x};\theta), \theta\in\Omega$, let $T=T(\utilde{X})$ be suff for $\theta$ and $g(t;\theta)$ be its pdf.

Given $\omega_0\subset\Omega, \omega_1\subset\Omega$ with $\omega_0\cap\omega_1=\empty$.

For tesing $H_0: \theta\in\omega_0$ v.s. $H_1: \theta\in\omega_1$. Suppose a test $\phi(T)$ with:
1. $\sup_{\theta\in\omega_0}E_\theta\phi(t)=\alpha$
2. $\exist \theta_0\in\omega_0$  s.t. $E_{\theta_0}\phi(T)=\alpha$ and $\forall \theta\in\omega_1, \exist c>0$ s.t.
   $$
    \phi(T)=\begin{cases}
         1 & \text{if } g(t;\theta_1)>cg(t;\theta_0)\\
         0 & \text{if } g(t;\theta_1)<cg(t;\theta_0)
    \end{cases}
    $$
$\implies$ $\phi(T)$ is UMP level $\alpha$ test.

Note: $\forall \theta\in\omega_1$ find the same $\phi$
:::

## MLR

:::info[Definition]
**Monotone Likelihood Ratio (MLR)**:

$\utilde{X}\sim f(\utilde{x};\theta), \theta\in\omega\subset\R$ and $T=T(\utilde{X})$ is suff for $\theta$ with pdf $g(t;\theta)$.

Suppose $\forall \theta_2>\theta_1$
$$
\frac{f(\utilde{x};\theta_2)}{f(\utilde{x};\theta_1)}=\frac{g(t;\theta_2)}{g(t;\theta_1)} \quad \text{is monotone in } v(t)
$$

$\implies f(\utilde{x};\theta)$ (or $g(t;\theta)$) has MLR.
:::

假設 $f$ 有 MLR, $f(\utilde{x};\theta_1)>cf(\utilde{x};\theta_0) \iff f(\utilde{x};\theta_1)/f(\utilde{x};\theta_0)>c$

1. 如果 $\theta_1>\theta_0$，則 $g(t;\theta_1)/g(t;\theta_0)$ 是單調遞增的，i.e. $v(t)$ 越大，$g(t;\theta_1)/g(t;\theta_0)$ 越大。
2. 如果 $\theta_1<\theta_0$，則 $g(t;\theta_1)/g(t;\theta_0)$ 是單調遞減的，i.e. $v(t)$ 越小，$g(t;\theta_1)/g(t;\theta_0)$ 越大。

$\exist k$ s.t.

$$
\iff \begin{cases}
    v(t)>k & \text{if } \theta_1>\theta_0\\
    v(t)<k & \text{if } \theta_1<\theta_0
\end{cases}
$$

因此當在 $v(t)$ 下具有 MLR 時，作檢定：
1. $H_0:\theta=\theta_0$ v.s. $H_1:\theta=\theta_1$ ($\theta_1>\theta_0$)

   拒絕 $H_0$ if $v(T)>k$ 會是 MP 檢定。
2. $H_0:\theta=\theta_0$ v.s. $H_1:\theta=\theta_1$ ($\theta_1<\theta_0$)

   拒絕 $H_0$ if $v(T)<k$ 會是 MP 檢定。

:::tip[Theorem]
$\utilde{X}\sim f(\utilde{x};\theta), T=T(\utilde{X})$ is suff for $\theta$ with pdf $g(t;\theta)$ has MLR in $v(t)$. Then for test:

1. $H_0:\theta=\theta_0$ v.s. $H_1:\theta=\theta_1$ ($\theta_1>\theta_0$)

   $$
    \phi(\utilde{X})=\phi(T)=\begin{cases}
        1 & \text{if } v(T)>k\\
        0 & \text{if } v(T)<k
    \end{cases}
    \quad \text{with } E_{\theta_0}\phi(T)=\alpha
    $$
    is UMP level $\alpha$ test.

2. $H_0: \theta=\theta_0$ v.s. $H_1:\theta>\theta_0$
   UMP level $\alpha$ test is also $\phi(T)$ in (1).

3. $H_0: \theta\le\theta_0$ v.s. $H_1:\theta>\theta_0$
   UMP level $\alpha$ test is also $\phi(T)$ in (1)

    and its power function $\nearrow$ in $\theta$.

4. $H_0: \theta\ge\theta_0$ v.s. $H_1:\theta<\theta_0$
   
   UMP level $\alpha$ test is
   $$
    \phi(\utilde{X})=\begin{cases}
        1 & \text{if } v(T)<k\\
        0 & \text{if } v(T)>k
    \end{cases}
   $$

    and its power function $\searrow$ in $\theta$.
:::

**EX**: $X_1, \cdots, X_n\overset{\text{iid}}{\sim}e^{-(x-\theta)}, x\ge\theta$

UMP level $\alpha$ test for $H_0:\theta\le\theta_0$ v.s. $H_1:\theta>\theta_0$

Note:

$$
f(\utilde{x};\theta)=\prod_{i=1}^ne^{-(x_i-\theta)}I(x_i\ge\theta)=e^{-\sum x_i+n\theta}I(x_{(1)}\ge\theta)
$$

$$
\implies \forall\theta_2>\theta_1, \frac{f(\utilde{x};\theta_2)}{f(\utilde{x};\theta_1)}=e^{n(\theta_2-\theta_1)}\frac{I(x_{(1)}\ge\theta_2)}{I(x_{(1)}\ge\theta_1)}\text{ is monotone in } x_{(1)}
$$

i.e. $f(\utilde{x};\theta)$ has MLR in $x_{(1)}$. UMP level $\alpha$ test is:

$$
\phi(\utilde{x})=\utilde{x_{(1)}}=\begin{cases}
    1 & \text{if } x_{(1)}>k\\
    r & \text{if } x_{(1)}=k\\
    0 & \text{if } x_{(1)}< k
\end{cases}
$$

$$
\begin{align*}
    \text{s.t. } \alpha&=E_{\theta_0}\phi(\utilde{x})=P_{\theta_0}(x_{(1)}>k)+r\cdot P_{\theta_0}(x_{(1)}=k)\\
    &=P_{\theta_0}(x_i>k, \forall i)\xlongequal{\text{iid}}\left[P_{\theta_0}(x_1>k)\right]^n\\
    &=\left[\int_k^\infty e^{-(x-\theta_0)}dx\right]^n=\left[e^{\theta_0}(-e^{-x}|_k^\infty)\right]^n\\
    &=e^{n\theta_0-nk}\\
    \implies k&=\theta_0-\frac{\ln \alpha}{n}
\end{align*}
$$

因此 UMP level $\alpha$ test 是：拒絕 $H_0$ if $x_{(1)}>\theta_0-\frac{\ln \alpha}{n}$

---

當假設檢定的對立假設是 $H_1:\theta>\theta_0$ 或 $H_1:\theta<\theta_0$ 時，稱為單邊（One-Sided）檢定。

而對立假設是 $H_1:\theta\neq\theta_0$ 或 $H_1:\theta<\theta_1$ or $\theta>\theta_2$ 時，稱為雙邊（Two-Sided）檢定。

對於單邊檢定問題，如果有 MLR 性質，我們可以直接得到 UMP 檢定。並且拒絕 $H_0$ 的範圍與 $H_1$ 是“同方向”的。i.e.:

- $H_1:\theta>\theta_0$，拒絕 $H_0$ if $v(T)>k$
- $H_1:\theta<\theta_0$，拒絕 $H_0$ if $v(T)<k$

## 1-par exp family

如果 $f(\utilde{x};\theta)\in$ 1-par exp family，i.e. $f(\utilde{x};\theta)=c(\theta)\exp(Q(\theta)T(\utilde{\theta}))h(\utilde{x})$ with $\mathscr{X}=\set{\utilde{x};f(\utilde{x};\theta)>0}\perp\theta$

$$
\implies \forall\theta_2>\theta_1, \frac{f(\utilde{x};\theta_2)}{f(\utilde{x};\theta_1)}=\frac{c(\theta_2)}{c(\theta_1)}\exp\left(T(\utilde{x})(Q(\theta_2)-Q(\theta_1))\right)
$$

注意到，如果對於所有 $\theta_2>\theta_1$ 都有 $Q(\theta_2)-Q(\theta_1)>0$，這代表 $Q$ 是單調遞增的，反之亦然。因此：

- 如果 $Q$ 遞增，那麼 $f$ 在 $T$ 下具有 MLR。
- 如果 $Q$ 遞減，那麼 $f$ 在 $-T$ 下具有 MLR。

:::tip[Theorem]
$f\in$ 1-par exp family

1. $Q$ 沿著 $\theta$ 遞增 $\implies$ $f$ 在 $T$ 下具有 MLR
   1. $H_0:\theta\overset{(\le)}{=}\theta_0$ v.s. $H_1:\theta>\theta_0$.
      $$
      \phi(\utilde{X})=\begin{cases}
          1 & \text{if } T(\utilde{X})>k\\
          r & \text{if } T(\utilde{X})=k\\
          0 & \text{if } T(\utilde{X})<k
      \end{cases}
      $$
    2. $H_0:\theta\overset{(\ge)}{=}\theta_0$ v.s. $H_1:\theta<\theta_0$.
      $$
      \phi(\utilde{X})=\begin{cases}
          1 & \text{if } T(\utilde{X})<k\\
          r & \text{if } T(\utilde{X})=k\\
          0 & \text{if } T(\utilde{X})>k
      \end{cases}
      $$
    
    s.t. $E_{\theta_0}\phi(\utilde{X})=\alpha$ is UMP level $\alpha$ test.

2. $Q$ 沿著 $\theta$ 遞減 $\implies$ $f$ 在 $-T$ 下具有 MLR
   1. $H_0:\theta\overset{(\le)}{=}\theta_0$ v.s. $H_1:\theta>\theta_0$.
      $$
      \phi(\utilde{X})=\begin{cases}
          1 & \text{if } T(\utilde{X})<k\\
          r & \text{if } T(\utilde{X})=k\\
          0 & \text{if } T(\utilde{X})>k
      \end{cases}
      $$
    2. $H_0:\theta\overset{(\ge)}{=}\theta_0$ v.s. $H_1:\theta<\theta_0$.
      $$
      \phi(\utilde{X})=\begin{cases}
          1 & \text{if } T(\utilde{X})>k\\
          r & \text{if } T(\utilde{X})=k\\
          0 & \text{if } T(\utilde{X})<k
      \end{cases}
      $$
    
    s.t. $E_{\theta_0}\phi(\utilde{X})=\alpha$ is UMP level $\alpha$ test.

3. $H_0:\theta\le\theta_1$ or $\theta\ge\theta_2$ v.s. $H_1:\theta_1<\theta<\theta_2$
   
   UMP level $\alpha$ test is
   $$
    \phi(t)=\begin{cases}
        1 & \text{if } k_1<t<k_2\\
        r_1 & \text{if } t=k_1\\
        r_2 & \text{if } t=k_2\\
        0 & \text{if } t<k_1\text{ or } t>k_2
    \end{cases}
   $$

    s.t. $E_{\theta_i}\phi(T)=\alpha$
:::

**EX**: $X_1,\cdots, X_n\overset{\text{iid}}{\sim}B(1, \theta)$

$$
\begin{align*}
    \implies f(\utilde{x};\theta)&=\prod_{i=1}^n\theta^{x_i}(1-\theta)^{1-x_i}\\
    &=\theta^t(1-\theta)^{n-t}\quad \text{with } t=\sum x_i\\
    &=(1-\theta)^n\left(\frac{\theta}{1-\theta}\right)^t\\
    &=c(\theta)\exp\left(t\ln\frac{\theta}{1-\theta}\right)\in \text{ 1-par exp family}\\
    \text{with }Q(\theta)&=\ln\frac{\theta}{1-\theta}\quad Q'(\theta)=\frac{1}{\theta}+\frac{1}{1-\theta}>0
\end{align*}
$$

因此 $f$ 在 $T=\sum X_i$ 下具有 MLR。對於檢定：

$$
H_0:\theta\overset{(=)}{\le}\theta_0\text{ v.s. }H_1:\theta>\theta_0
$$

UMP level $\alpha$ test 是：

$$
\phi(t)=\begin{cases}
    1 & \text{if } t>k\\
    r & \text{if } t=k\\
    0 & \text{if } t<k
\end{cases}
$$
$$
\text{ with } \alpha=E_{\theta_0}\phi(T)=P_{\theta_0}(T>k)+r\cdot P_{\theta_0}(T=k)
$$

在樣本數比較大的情況下，我們難以計算 $P_{\theta_0}(T>k)$。我們可以用中央極限定理來近似到標準常態分佈，這樣就可以查表得到 $k$。

$$
\begin{align*}
    \alpha&=P_{\theta_0}(T>k)+r\cdot P_{\theta_0}(T=k)\\
    &=P\left(\frac{T-n\theta_0}{\sqrt{n\theta_0(1-\theta_0)}}>\frac{k-n\theta_0}{\sqrt{n\theta_0(1-\theta_0)}}\right)+r\cdot \underbrace{P\left(\frac{T-n\theta_0}{\sqrt{n\theta_0(1-\theta_0)}}=\frac{k-n\theta_0}{\sqrt{n\theta_0(1-\theta_0)}}\right)}_{\text{連續分佈的單點幾率為0}}\\
    &\approx P(Z>\frac{k-n\theta_0}{\sqrt{n\theta_0(1-\theta_0)}})\\
    &\implies \frac{k-n\theta_0}{\sqrt{n\theta_0(1-\theta_0)}}=Z_\alpha\\
    &\implies k=n\theta_0+Z_\alpha\sqrt{n\theta_0(1-\theta_0)}
\end{align*}
$$

- 假設 $n=25, \theta_0=0.5, \alpha=0.01$

$k=25\cdots\frac{1}{2}+Z_{0.01}\sqrt{25\cdot\frac{1}{2}\cdot\frac{1}{2}}\approx 25\cdot\frac{1}{2}+2.33\approx 18.3$

$$
\implies \phi(t)=\begin{cases}
    1 & \text{if } t>18.3\\
    0 & \text{if } t<18.3
\end{cases}=\begin{cases}
    1 & \text{if } t=19, 20, \cdots, 25\\
    0 & \text{o.w.}
\end{cases}
$$

## 雙邊問題（Two-Sided Problem）

$$
H_0:\theta=\theta_0\text{ v.s. }H_1:\theta\neq\theta_0
$$

or

$$
H_0:\theta_1\le\theta_\le\theta_2 \text{ v.s. }H_1:\theta<\theta_1\text{ or }\theta>\theta_2
$$

對於雙邊問題，UMP 檢定**通常**不存在（反例：$U(0, \theta)$）

**EX**: For 1-par exp family

$$
f(\utilde{x}\theta)=c(\theta)\exp(Q(\theta)T(\utilde{x}))h(\utilde{x})\quad\text{ with } Q(\theta)\nearrow \text{ in }\theta
$$

$\implies$ UMP level $\alpha$ test for $H_0:\theta=\theta_0$ v.s. $H_1:\theta>\theta_0$ is: reject $H_0$ if $T>k_1$ with $E_{\theta_0}\phi_1(T)=\alpha$

UMP level $\alpha$ test for $H_0:\theta=\theta_0$ v.s. $H_1:\theta<\theta_0$ is: reject $H_0$ if $T<k_2$ with $E_{\theta_0}\phi_2(T)=\alpha$

$\implies$ UMP level $\alpha$ test for $H_0:\theta=\theta_0$ v.s. $H_1\theta\neq\theta_0$ **does not exist**.

**Idea**: 假設 UMP test 存在，令 $\phi^*$ 是 UMP test，並且 $E_{\theta_0}\phi^*=\alpha$

因為是 UMP test，那麼 $\phi^*$ 的 power 應該比其他任何 $\alpha$ test 都大，i.e. $\forall \phi$ with $E_{\theta_0}\phi=\alpha$，有 $E_{\theta}\phi^*\ge E_{\theta}\phi, \forall\theta\neq\theta_0$。

$H_1$ 所假設的範圍可以拆成 $\theta>\theta_0$ 和 $\theta<\theta_0$。因為 $\phi^*$ 是 UMP test，那麼對於這兩個範圍的檢定，$\phi^*$ 也應該是 UMP test。

這兩個範圍的 UMP test 分別是 $\phi_1$ 和 $\phi_2$。而 UMP test **通常** 是唯一的，因此 $\phi^*=\phi_1=\phi_2$。

但 $\phi_1\neq\phi_2$ 是矛盾的，因此 UMP test 不存在。

---

**EX** $X_1,\cdots,X_n\overset{\text{iid}}{\sim}U(0,\theta)$

UMP level $\alpha$ test for $H_0:\theta=\theta_0$ v.s. $H_1:\theta\neq\theta_0$ exists and is given

1. UMP level $\alpha$ test for $H_0:\theta=\theta_0$ v.s. $H_1:\theta>\theta_0$ is
   $$
    \phi_1(X_{(n)})=\begin{cases}
        1 & \text{if }  x_{(n)}>k_1\\
        0 & \text{o.w.}
    \end{cases} \quad \text{with } \begin{align*}
        \alpha&=P_{\theta_0}(X_{(n)}>k_1)\\
        &=1-P_{\theta_0}(X_{(n)}\le k_1)\\
        &=1-\left(\frac{k_1}{\theta_0}\right)^n
    \end{align*}
   $$
   $$
   \implies k_1=\theta_0(1-\alpha)^{1/n}
   $$

   and its power, $\forall\theta>\theta_0$

   $$
   \begin{align*}
       E_\theta\phi_i(X_{(n)})&=P_\theta(X_{(n)}>\theta_0(1-\alpha)^{1/n})\\
       &=1-P_\theta(X_{(n)}\le\theta_0(1-\alpha)^{1/n})\\
       &=1-\left(\frac{\theta_0(1-\alpha)^{1/n}}{\theta}\right)^n\\
       &=1-\left(\frac{\theta_0}{\theta}\right)^n(1-\alpha)\quad\forall\theta>\theta_0\\
       &\ge E_\theta\phi(X_{(n)})\quad\forall\phi\text{ with }E_{\theta_0}\phi=\alpha
   \end{align*}
   $$
2. UMP level $\alpha$ test $H_0:\theta=\theta_0$ v.s. $H_1: \theta<\theta_0$ is
   $$
   \phi_2(X_{(n)})=\begin{cases}
       1 & \text{if }  x_{(n)}<k_2\\
       0 & \text{o.w.}
    \end{cases} \quad\text{with }\begin{align*}
        \alpha&=E_{\theta_0}\phi_2(X_{(n)})\\
        &=P_{\theta_0}(X_{(n)}<k_2)\\
        &=\left(\frac{k_2}{\theta_0}\right)^n
    \end{align*}
   $$
   $$
   \implies k_2=\theta_0\alpha^{1/n}
   $$

   with power, $\forall\theta<\theta_0$
   $$
    \begin{align*}
       E_\theta\phi_2(X_{(n)})&=P_\theta(X_{(n)}<\theta_0\alpha^{1/n})\\
       &=\begin{cases}
         1&\text{if }\theta\le\theta_0\alpha^{1/n}\\
         \left(\frac{\theta_0}{\theta}\right)^n\alpha&\text{if }\theta>\theta_0\alpha^{1/n}
       \end{cases}\\
       &\ge E_\theta\phi(X_{(n)})\quad\forall\phi\text{ with }E_{\theta_0}\phi=\alpha
    \end{align*}
   $$

3. UMP level $\alpha$ test for $H_0:\theta_1\le\theta\le\theta_2$ v.s. $H_1:\theta\neq\theta_2$ is
   $$
    \phi(X_{(n)})=\begin{cases}
        1 & \text{if } x_{(n)}>\theta_0\text{ or } x_{(n)}<\theta_0\alpha^{1/n}\\
        0 & \text{o.w.}
    \end{cases}
   $$
   $$
   \implies E_{\theta_0}\phi(X_{(n)})=\underbrace{P_{\theta_0}(X_{(n)}>\theta_0)}_{=0}+P_{\theta_0}(X_{(n)}<\theta_0\alpha^{1/n})=\left(\frac{\theta_0\alpha^{1/n}}{\theta_0}\right)^n=\alpha
   $$
   and its power
   $$
   \begin{align*}
       E_\theta\phi(X_{(n)})&=P_\theta(X_{(n)}>\theta_0)+P_\theta(X_{(n)}<\theta_0\alpha^{1/n})\\
       &=\begin{cases}
         1-\left(\frac{\theta_0}{\theta} \right)^n&\text{if }\theta>\theta_0\\
         E_\theta\phi_2(X_{(n)})&\text{if }\theta<\theta_0\alpha^{1/n}
       \end{cases}\\
       &\ge E_\theta\phi(X_{(n)})\quad\forall\phi\text{ with }E_{\theta_0}\phi=\alpha
   \end{align*}
   $$
**EX**: 

## UMPU Test

:::info[Definition]
1. A level $\alpha$ test $\phi$ for $H_0:\theta\in\omega_0$ v.s. $H_1:\theta\in\omega_1$ is said to be *unbiased* if
   $$
    E_\theta\phi(\utilde{x})\ge\alpha\quad\forall\theta\in\omega_1
   $$
2. $\phi^*$ is *UMPU* levet $\alpha$ test $\iff\phi^*$ is UMP level $\alpha$ test among *unbiased* test
:::

我們可以找一個一定無偏的 test $\phi_\alpha=\alpha$，使得 $E_\theta[\phi_\alpha]=\alpha, \forall\theta$。因為 UMP test $\phi^*$ 的 power 比任何 $\alpha$ test 都大，i.e. $\forall\theta\in\omega_1, E_\theta[\phi^*]\ge E_\theta[\phi_\alpha]=\alpha$。因此 UMP test 一定是 UMPU test。

而在單邊問題下，如果有 MLR，那麼我們可以直接得到 UMP test，也就是 UMPU test。所以 UMPU 的理論結果只討論在雙邊問題下。

:::tip[Theorem]
Let $T=T(\utilde{X})$ be sufficient for $\eta\in\R$ with p.d.f. $g(t;\eta)=c(\eta)\exp(\eta\cdot t)h(t)\in$ 1-par exp family.

1. $H_0:\eta=\eta_0$ v.s. $H_1:\eta\neq\eta_0$

   UMPU level $\alpha$ test exists and is given
   $$
    \phi_1(T)=\begin{cases}
        1 & \text{if } t>k_1\text{ or } t<k_2\\
        r_i & \text{if } t=k_i, i=1,2,\cdots\\
        0 & \text{o.w.}
    \end{cases}\quad \text{with} k_1>k_2, r_i\in[0,1],i=1,2,\cdots
   $$
   s.t. $E_{\eta_0}\phi(T)=\alpha$ and $E_{\eta_0}[T\phi(T)]=\alpha E_{\eta_0}[T]$ （$E_{\eta_0}\phi(T)$ 在 $\eta_0$ 時有最小值）。

2. $H_0:\eta_1\le\eta\le\eta_2$ v.s. $H_1:\eta<\eta_1$ or $\eta>\eta_2$

   UMPU level $\alpha$ test exists and is given
   $$
    \phi_2(T)=\begin{cases}
        1 & \text{if } t>k_1\text{ or } t<k_2\\
        r_i & \text{if } t=k_i, i=1,2,\cdots\\
        0 & \text{o.w.}
    \end{cases}\quad \text{with} k_1>k_2, r_i\in[0,1],i=1,2,\cdots
   $$
   s.t. $E_{\eta_1}\phi(T)=\alpha$ and $E_{\eta_1}\phi_2(T)=\alpha=E_{\eta_2}\phi_2(T)$
:::

**EX**: $X_1, \cdots, X_n\overset{\text{iid}}{\sim}N(\theta ,\sigma^2_0)$ with $\sigma^2_0$ known $\implies T=\bar{X}\sim N(\theta, \tau^2)$ where $\tau^2=\sigma^2_0/n$.i.e.

$$
\begin{align*}
    g(t;\theta)&=\frac{1}{\sqrt{2\pi\tau^2}}\exp\left(-\frac{(t-\theta)^2}{2\tau^2}\right)\\
    &=\underbrace{\frac{1}{\sqrt{2\pi}\tau}\exp\left(-\frac{t^2}{2\tau^2}\right)}_{h(t)}\exp\left(\underbrace{\frac{\theta}{\tau^2}}_{\eta}\cdot t\right)\exp\left(-\frac{\theta^2}{2\tau^2}\right)\\
    &=g(t;\eta)
\end{align*}
$$

Note $\tau^2$ is known, $\eta=\frac{\theta}{\tau^2}, \eta_0=\frac{\theta_0}{\tau^2}$

$$
\implies H_0:\theta=\theta_0\iff H_0:\frac{\theta}{\tau^2}=\frac{\theta_0}{\tau^2}\iff H_0:\eta=\eta_0
$$

For testing $H_0:\theta=\theta_0$ v.s. $H_1:\theta\neq\theta_0$, UMPU level $\alpha$ test is

$$
\phi(t)=\begin{cases}
    1 & \text{if } t>k_1\text{ or } t<k_2\quad(k_1<k_2)\\
    r_i & \text{if } t=k_i, i=1,2,\cdots\\
    0 & \text{o.w.}
\end{cases}
$$

$$
\begin{align*}
    \text{with }\alpha&=E_{\eta_0}\phi(T)=E_{\theta_0}\phi(T)\\
    &=P_{\theta_0}(T<k_1)+P_{\theta_0}(T>k_2)\quad\text{單點機率為0}\\
    &=P(Z<\frac{k_1-\theta_0}{\tau})+P(Z>\frac{k_2-\theta_0}{\tau})\\
\end{align*}
$$

$$
\implies k_1=\theta_0-\frac{\sigma_0}{\sqrt{n}}Z_{\alpha/2}\quad k_2=\theta_0+\frac{\sigma_0}{\sqrt{n}} Z_{\alpha/2}
$$

$$
\begin{align*}
    \text{i.e. }\phi(t)&=\begin{cases}
    1 & \text{if } t>\theta_0+\frac{\sigma_0}{\sqrt{n}}Z_{\alpha/2}\text{ or } t<\theta_0-\frac{\sigma_0}{\sqrt{n}}Z_{\alpha/2}\\
    0 & \text{o.w.}
   \end{cases}\\
   &=\begin{cases}
    1&\text{if } \left|\frac{\sqrt{n}(\bar{X}-\theta_0)}{\sigma_0}\right|>Z_{\alpha/2}\\
    0&\text{o.w.}
   \end{cases}\\
   &=I\left(\left|\frac{\sqrt{n}(\bar{X}-\theta_0)}{\sigma_0}\right|>Z_{\alpha/2}\right)
\end{align*}
$$

and 

$$
\begin{align*}
    E_{\eta_0}[T\phi(T)]&=E_{\theta_0}\left[T\cdot I\left(\left|\frac{\sqrt{n}(\bar{X}-\theta_0)}{\sigma_0}\right|>Z_{\alpha/2}\right)\right]\quad T\sim N(\theta_0, \frac{\sigma^2_0}{n})\\
    &=E_{\theta_0}\left[(\theta_0+\frac{\sigma^2_0}{\sqrt{n}}Z)I(|Z|>Z_{\alpha/2})\right]\\
    &=\theta_0E_{\theta_0}\left[I(|Z|>Z_{\alpha/2})\right]+\frac{\sigma_0}{\sqrt{n}}E_{\theta_0}\left[Z\cdot I(|Z|>Z_{\alpha/2})\right]\\
    &=\theta_0\alpha+0
\end{align*}
$$

$$
\begin{align*}
    \because E[Z\cdot I(|Z|>Z_{\alpha/2})]&=\int_{|Z|>Z_{\alpha/2}}z\cdot\frac{1}{\sqrt{2\pi}}e^{-z^2/2}dz\\
    &=\int_{Z_{\alpha/2}}^\infty \frac{z}{\sqrt{2\pi}}e^{-z^2/2}dz+\int_{-\infty}^{-Z_{\alpha/2}}\frac{z}{\sqrt{2\pi}}e^{-z^2/2}dz\\
    &=0 \quad\because \text{odd function}
\end{align*}
$$

---

**Remark**:

$$
g(t;\theta)=c(\theta)\exp(Q(\theta)\cdot t)h(t)\xRightarrow[\theta=Q^{-1}(\eta)]{\eta=Q(\theta)}c_0(\eta)\exp(\eta\cdot t)h(t)=g(t;\eta)
$$

$$
\implies \theta=\theta_0 \iff \eta=\eta_0\qquad \theta_1\le\theta\le\theta_2\iff \eta_1\le\eta\le\eta_2
$$

:::info[Theorem]
UMPU test exist when p.d.f is of this form

$$
c(\theta,\underbrace{\xi_1,\xi_2,\cdots,\xi_k}_{\text{nuisance param}})\exp\left(\theta T(\utilde{x})+\sum_{i=1}^k\xi_iU_i(\utilde{x})\right)h(\utilde{x})\in\text{ (k+1)-par exp family}
$$

of interest is $\theta\in\R$ and nulls are

$$
\begin{alignat*}{2}
    &H_0^1:\theta\le\theta_0 &\quad\text{ v.s. }\quad &H_1^1:\theta>\theta_0\\
    &H_0^2:\theta\ge\theta_0 &\quad\text{ v.s. }\quad &H_1^2:\theta<\theta_0\\
    &H_0^3:\theta_1\le\theta\le\theta_2 &\quad\text{ v.s. }\quad &H_1^3:\theta<\theta_1\text{ or }\theta>\theta_2\\
    &H_0^4:\theta\le\theta_1\text{ or }\theta\ge\theta_2 &\quad\text{ v.s. }\quad &H_1^4:\theta_1<\theta<\theta_2\\
    &H_0^5:\theta=\theta_0 &\quad\text{ v.s. }\quad &H_1^5:\theta\neq\theta_0
\end{alignat*}
$$

基本上，$H_0$ 的拒絕區域取決於 $H_1$ 的範圍。並通過計算顯著水準來確定拒絕的分界點。
:::

**EX**: $X_1,\cdots,X_n\overset{\text{iid}}{\sim}N(\theta, \sigma^2)\implies(\bar{X}, S^2)$: sufficient for $(\theta, \sigma^2)$ with $\theta, \sigma^2$ unknown.

$\implies H_0:\theta\le\theta_0$ v.s. $H_1:\theta>\theta_0$, UMPU level $\alpha$ test is reject $H_0$ if $\underbrace{\sqrt{n}(\bar{x}-\theta_0)/S}_{\sim t_{n-1}}>t_{n-1,\alpha}$
