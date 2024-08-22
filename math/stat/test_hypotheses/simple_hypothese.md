---
sidebar_position: 1
---

# 簡單假設（Simple Hypothese）

:::info[Definition]
如果 $\sup_{\theta\in\omega_0}E_\theta \phi^*(\utilde{X})=\alpha$ 是 level $\alpha$ 的檢定，並且對於所有的 level $\alpha$ 的檢定 $\phi(\utilde{X})$ 有

$$
E_\theta\phi^*(\utilde{X})\ge E_\theta\phi(\utilde{X})\quad \forall \underbrace{\theta\in\omega_1}_{\text{uniformly}}
$$

則 $\phi^*(\utilde{X})$ 被稱爲 level $\alpha$ 的 *最強檢定（Uniformly Most Powerful Test, UMP Test）*。

如果 $H_1$ 是 simple，則 UMP 被稱為 *MP* 檢定。
:::

## MP 檢定（MP test）

我們獲得單筆數據 $X$，並且有以下分佈：

| $X=$       | 1    | 2    | 3    | 4    | 5    | 6    | 7    |
| ---------- | ---- | ---- | ---- | ---- | ---- | ---- | ---- |
| $\theta_0$ | 0.01 | 0.01 | 0.02 | 0.01 | 0.01 | 0.01 | 0.93 |
| $\theta_1$ | 0.09 | 0.08 | 0.07 | 0.06 | 0.05 | 0.04 | 0.61 |

在顯著水平 $\alpha=0.05$ 下檢定 $H_0: \theta=\theta_0$ v.s. $H_1: \theta=\theta_1$。

$$
\phi_1(X)=\begin{cases}
    1 & \text{if } X=1,2,4,5,6\\
    0 & \text{o.w.}
    \end{cases}
$$

with level $=E_{\theta_0}\phi_1(X)=P_{\theta_0}(X=1 \text{ or } X=2 \text{ or } X=4 \text{ or } X=5 \text{ or } X=6)=0.05$ 

power of $\phi_1=0.32$

$$
\phi_2(X)=\begin{cases}
    1 & \text{if } X=1,2,3,4\\
    0 & \text{o.w.}
    \end{cases}
$$

with level $=E_{\theta_0}\phi_2(X)=P_{\theta_0}(X=1 \text{ or } X=2 \text{ or } X=3 \text{ or } X=4)=0.05$

power of $\phi_2=0.3$

$$
\phi_3(X)=\begin{cases}
    1 & \text{if } X=3,4,5,6\\
    0 & \text{o.w.}
    \end{cases}
$$

with level $=E_{\theta_0}\phi_3(X)=P_{\theta_0}(X=3 \text{ or } X=4 \text{ or } X=5 \text{ or } X=6)=0.05$

power of $\phi_3=0.22$

$$
\phi_4(X)=\begin{cases}
    \frac{0.05}{0.93} & \text{if } X=7\\
    0 & \text{o.w.}
    \end{cases}
$$

with level $=E_{\theta_0}\phi_4(X)=\frac{0.05}{0.93}P_{\theta_0}(X=7)=0.05$

power of $\phi_4=\frac{0.05}{0.93}\cdot 0.61$

當我們想從這 4 中檢定中選擇一個最好的時，我們需要比較他們的 power。通過比較發現 $\phi_1$ 是最好的檢定。

但為什麼 $\phi_1$ 能得到最好的 power 呢？因為他選擇拒絕 $H_0$ 的 $X$ 是在 $\theta_1$ 下概率最大而在 $\theta_0$ 下概率最小的。

| $X=$                                  | 1    | 2    | 3    | 4    | 5    | 6    | 7               |
| ------------------------------------- | ---- | ---- | ---- | ---- | ---- | ---- | --------------- |
| $\theta_0$                            | 0.01 | 0.01 | 0.02 | 0.01 | 0.01 | 0.01 | 0.93            |
| $\theta_1$                            | 0.09 | 0.08 | 0.07 | 0.06 | 0.05 | 0.04 | 0.61            |
| $\frac{f(x;\theta_1)}{f(x;\theta_0)}$ | 9    | 8    | 3.5  | 6    | 5    | 4    | $\frac{61}{93}$ |

最下面一行可以認為是“CP 值”，我們只要選擇讓 CP 值最大的 $X$ 來拒絕 $H_0$，就能得到最好的 power。比如在這個例子中，$\phi_1$ 就選擇了 "CP 值" 大於等於 4 的 $X$，i.e.:

$$
\phi_1(X)=\begin{cases}
    1 & \text{if } f(x;\theta_1) \ge 4f(x;\theta_0)\\
    0 & \text{o.w.}
    \end{cases}
$$

:::tip[Theorem]
**Neyman-Pearson Fundamental Lemma**:
Let $\utilde{X}\sim f(\utilde{x};\theta)$ for testing

$$
\text{Simple } H_0: \theta=\theta_0 \text{ v.s. Simple } H_1: \theta=\theta_1
$$

with a given $0<\alpha<1$，除非存在一個 power=1 & sig. level $<\alpha$ 的檢定函數

我們會有以下結論：
1. **存在性**
   $\exist$ a test $\phi(\utilde{X})$ and a constant $c>0$ s.t.

   1. $E_{\theta_0}\phi(\utilde{X})=\alpha$
   2. 
      $$
      \phi(\utilde{X})=\begin{cases}
        1 & \text{if } f(\utilde{x};\theta_1) > cf(\utilde{x};\theta_0)\\
        0 & \text{if } f(\utilde{x};\theta_1) < cf(\utilde{x};\theta_0)
      \end{cases}
      $$

2. **充分條件**
   如果 $\phi^*$ 滿足存在性的兩個條件，則 $\phi^*$ 在所有 level $\le \alpha$ 檢定下都是 MP 檢定。

3. **必要條件**
   如果 $\phi^*$ 是 level $\alpha$ 的 MP 檢定，則 $\phi^*$ 滿足存在性的兩個條件。
:::

**Remark**:

1. 根據 Lemma 的第 1 點，通常可以得到唯一的 MP $\alpha$ 檢定（反例：$U(0, \theta)$）
2. 滿足以下條件的檢定函數 $\phi$ 被稱爲 N-P 檢定函數：
   $$
    \phi(\utilde{X})=\begin{cases}
        1 & \text{if } f(\utilde{x};\theta_1) > cf(\utilde{x};\theta_0)\\
        0 & \text{if } f(\utilde{x};\theta_1) < cf(\utilde{x};\theta_0)\\
        \gamma & \text{if } f(\utilde{x};\theta_1) = cf(\utilde{x};\theta_0)
    \end{cases}
    \quad \text{with }c>0\quad \gamma\in[0,1]
   $$

   使得 $E_{\theta_0}\phi(\utilde{X})=P_{\theta_0}(f(\utilde{x};\theta_1) > cf(\utilde{x};\theta_0))+\gamma P_{\theta_0}(f(\utilde{x};\theta_1) = cf(\utilde{x};\theta_0))=\alpha$

    因此，$\phi$ 滿足 Lemma 的兩個條件，所以 N-P 檢定函數是 MP 檢定函數。

:::tip[Corollary]
如果 $\phi$ 滿足 N-P Lemma 的兩個條件（因此 $\phi$ 是 MP level $\phi$ 檢定）

則 $E_{\theta_1}\phi(\utilde{X})\ge \alpha = E_{\theta_0}\phi(\utilde{X})$
:::

**Proof**: 令 $\phi_\alpha(\utilde{X})=\alpha\implies E_\theta\phi_\alpha(\utilde{X})=\alpha$

$\because \phi$ is MP level $\alpha$ $\implies E_{\theta_1}\phi(\utilde{X})\ge E_{\theta_1}\phi_\alpha(\utilde{X})$

如果等於號成立，則 $\phi_\alpha(\utilde{X})$ 也會是 MP level $\alpha$ 檢定。

**Remark**:事實上，除非 $\utilde{X}$ 在 $\theta_1$ 和 $\theta_0$ 的 cdf 相同，否則等號不會成立。

by N-P Lemma, $\exist c>0$ s.t.:

$$
\alpha=\phi_\alpha\utilde{X}=\begin{cases}
    1 & \text{if } f(\utilde{x};\theta_1) > cf(\utilde{x};\theta_0)\\
    0 & \text{if } f(\utilde{x};\theta_1) < cf(\utilde{x};\theta_0)\\
    \gamma & \text{if } f(\utilde{x};\theta_1) = cf(\utilde{x};\theta_0)
\end{cases}
$$

$$
\implies \gamma=\alpha
$$

因此
1. 當$\theta_1>\theta_0$,  $E_\theta\phi(\utilde{X})$ 是沿着 $\theta$ 遞增 $\implies \sup_{\theta\le\theta_0}E_\theta\phi(\utilde{X})=E_{\theta_0}\phi(\utilde{X})=\alpha$。因此我們可以把 H_0 擴產爲 $H_0: \theta\le\theta_0$。
2. 當$\theta_1<\theta_0$,  $E_\theta\phi(\utilde{X})$ 是沿着 $\theta$ 遞減 $\implies \sup_{\theta\ge\theta_0}E_\theta\phi(\utilde{X})=E_{\theta_0}\phi(\utilde{X})=\alpha$。因此我們可以把 H_0 擴產爲 $H_0: \theta\ge\theta_0$。

---

**EX**: 我們得到單個數據 $X$，並且作以下檢定：

$$
H_0: X\sim U(0,1) \text{ v.s. } H_1: X\sim f(x;\theta_1)=2x\quad x\in(0,1)
$$

根據 N-P Lemma，我們可以在 $f(x;\theta_1) > cf(x;\theta_0)$ 時拒絕 $H_0$。而 $f(x;\theta_1) > cf(x;\theta_0) \iff x>k$，i.e.:

$$
\phi(X)=\begin{cases}
    1 & \text{if } X>k\\
    0 & \text{if } X<k
\end{cases}
\quad \text{with } \alpha=E_{\theta_0}\phi(X)=P_{\theta_0}(X>k)=1-k
$$

$\implies k=1-\alpha$。因此我們可以得到 MP：

$$
\phi(X)=\begin{cases}
    1 & \text{if } X>1-\alpha\\
    0 & \text{if } X<1-\alpha
\end{cases}
$$

而它的 power 是：

$$
E_{\theta_1}\phi(X)=P_{\theta_1}(X>1-\alpha)=1-(1-\alpha)^2
$$

但假如我們現在獲得了 $n\ge 2$ 筆數據，i.e. $X_1,\cdots,X_n\overset{\text{iid}}{\sim}f(x;\theta)$，並且有以下檢定：

$$
H_0: X\sim U(0,1) \text{ v.s. } H_1: X\sim f(x;\theta_1)=2x\quad x\in(0,1)
$$

根據 N-P Lemma，$\phi(\utilde{X})=1$ if $\prod_{i=1}^nX_i > k \iff \sum_{i=1}^n(-2\ln X)<k'=-2\ln k$。

$$
\begin{align*}
    \implies \alpha=E_{\theta_0}\phi(\utilde{X})&=P_{\theta_0}\left(\sum_{i=1}^n(-2\ln X_i)<k'\right)\\
    &=P_{\theta_0}\left(\chi^2_{2n}<k'\right) \quad \because -2\ln U(0,1)\sim \chi^2_2\\
    &=P_{\theta_0}\left(w<k'\right) \quad \text{where } w\sim \chi^2_{2n}
\end{align*}
$$

我們可以定義一個數字 $\chi^2_{m,\alpha}$ 使得 $P(w>\chi^2_{m,\alpha})=\alpha$。因此我們可以得到 MP：

$$
\phi(\utilde{X})=1 \text{ if } \sum_{i=1}^n(-2\ln X_i)<\chi^2_{2n,1-\alpha}
$$

---

**EX**: $X_1, \cdots, X_n \overset{\text{iid}}{\sim} U(0, \theta)$

給定 $\theta_1\neq\theta_0\in\R^+$，做以下檢定：

$$
H_0: \theta=\theta_0 \text{ v.s. } H_1: \theta=\theta_1
$$

根據 N-P Lemma，我們可以得到 level $\alpha$ 的 MP 檢定是：

$$
\phi(\utilde{X})=\begin{cases}
    1& \text{if } f(\utilde{x};\theta_1)\ge cf(\utilde{x};\theta_0)\\
    0& \text{if } f(\utilde{x};\theta_1)<cf(\utilde{x};\theta_0)
\end{cases}
\quad \text{with } c>0 \text{ s.t. } E_{\theta_0}\phi(\utilde{X})=\alpha
$$

因為 $X_{(n)}$ 是 sufficient statistic，我們可以得到：

$$
\iff \phi(\utilde{X})=\begin{cases}
    1 & \text{if } X_{(n)}\ge k\\
    0 & \text{if } X_{(n)}<k
\end{cases}
$$

with

$$
\alpha=E_{\theta_0}\phi(\utilde{X})=P_{\theta_0}(X_{(n)}\ge k)=1-\left(\frac{k}{\theta_0}\right)^n \quad\implies k=\theta_0(1-\alpha)^{1/n}<\theta_0
$$

而它的 power 是：

$$
E_{\theta_1}\phi(\utilde{X})=P_{\theta_1}(X_{(n)}\ge k)=1-\left(\frac{k}{\theta_1}\right)^n=1-\left(\frac{\theta_0(1-\alpha)^{1/n}}{\theta_1}\right)^n=1-\left(\frac{\theta_0}{\theta_1}\right)^n(1-\alpha)
$$

但對於這種情況，可以發現如果有 $X$ 落在 $\theta_0$ 和 $\theta_1$ 之間，那我們可以立馬得知 $\theta_1$ 是真的。因此我們可以造另一個檢定函數：

$$
\phi_1 = \begin{cases}
    1 & \text{if } X_{(n)}> \theta_0\\
    \alpha & \text{if } 0<X_{(n)}\le \theta_0\\
\end{cases}
$$

with

$$
E_{\theta_0}\phi_1(\utilde{X})=P_{\theta_0}(X_{(n)}>\theta_0)+\alpha P_{\theta_0}(0<X_{(n)}\le \theta_0)=0+\alpha\cdot 1=\alpha
$$

而它的 power 同樣會是：

$$
\begin{align*}
    E_{\theta_1}\phi_1(\utilde{X})&=P_{\theta_1}(X_{(n)}>\theta_0)+\alpha P_{\theta_1}(0<X_{(n)}\le \theta_0)\\
    &=1-P_{\theta_1}(X_{(n)}\le \theta_0)+\alpha P_{\theta_1}(0<X_{(n)}\le \theta_0)\\
    &=1-\left(\frac{\theta_0}{\theta_1}\right)^n+\alpha\left(\frac{\theta_0}{\theta_1}\right)^n\\
    &=1-\left(\frac{\theta_0}{\theta_1}\right)^n(1-\alpha)
\end{align*}
$$

因此我們造出了兩個不同但都是 MP 的檢定函數。
