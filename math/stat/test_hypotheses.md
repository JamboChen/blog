# 假設檢定（Testing Hypotheses）

在得到數據 $\utilde{X}=(X_1, \cdots, X_n)\sim f(\utilde{x};\theta)$ with $\theta\in\Omega\subseteq\R^n$ 時，假設檢定問題旨在決定是否要拒絕 $H_0$ 虛無假設（null hypothese）。

## 假設

:::info[Definition]
1. 假設是關於 $\theta$ 的敘述。$\iff$ 對於母體分佈的敘述。
2. 如何假設只描述了一種分佈，那麽稱之爲*簡單假設（Simple Hypothese）*。否則成爲*複合假設(Composite Hypothese)*。
:::

**EX**: $X\sim N(\mu, \sigma^2)$
- $H: \mu=20, \sigma^2=1 \iff H: X\sim N(20,1)$ is **Simple**。
- $H: \mu\ge 60, \sigma^2:$ unkonwn $\iff H: X\sim N(\mu, \sigma^2)$ with $\mu\ge 60, \sigma^2$ unknow is **Composite**。

通常來説，我們關注的是參數的範圍，i.e. $H: \theta\in \omega \subseteq \Omega$。
- $|\omega|=1\implies H$ is simple.
- $|\omega|>1\implies H$ is composite.

而我們檢定的是 $H_0: \theta\in\omega_0$ v.s. $H_1: \theta\in\omega_1$，并且 $\omega_0\cap \omega_1=\empty$

## 假設檢定

在真實的情況中，我們永遠無法知道 $\theta$ 的值是什麽。而我們同樣無法知道 $H: \theta\in\omega, \forall\omega$ 是真的還是假的。

所以再給檢定結果下結論時，我們不會説我們接受了某個假設。而是拒絕或不拒絕某個假設。

因此檢定 $H_0: \theta\in \omega_0$ v.s. $H_1: \theta\in\omega_1$ 時
- $H_0$ 是默認的假設，稱爲*虛無假設（null hypothese）*。
- $H_1$ 是*對立假設（attenative hypothese）*，是我們想要説明的假設。

**E.G.**:
- $H_0:$ 無罪 v.s. $H_1:$ 有罪
- $H_0:$ 及格 v.s. $H_1:$ 不及格
- $H_0:$ 不及格 v.s. $H_1:$ 及格

## 檢定函數（Test Function）

當我們拿到一組數據 $\utilde{X}=\utilde{x}$ ，希我們會需要在這組數據的基礎上利用檢定函數 $\phi$ 來決定是否拒絕 $H_0$。

:::info[Definition]
任何函數 $\phi$ 滿足

$$
\phi:\R^n\to[0,1]
$$

都可以稱爲*檢定函數（Test Function）*。

並且 $\forall \utilde{x}\in \R^n, \phi(\utilde{x})$ 是拒絕 $H_0$ 的 pdf。
:::

**e.g.**:
1. $\phi(\utilde{x})=1 \iff$ 在 $\utilde{x}$ 下拒絕 $H_0$。
2. $\phi(\utilde{x})=1 \iff$ 在 $\utilde{x}$ 下不拒絕 $H_0$。
3. $\phi(\utilde{x})=\frac{1}{2} \iff$ 丟一枚公平硬幣決定是否拒絕 $H_0$，i.e. 拒絕 $H_0$ 的概率是 $\frac{1}{2}$。

## 型 I 和型 II 錯誤

在檢定問題中，有 4 中可能的情況：

|                                 | reject $H_0$     | not reject $H_0$ |
| ------------------------------- | ---------------- | ---------------- |
| $\theta\in\omega_0,H_0$ is true | Type I Error     | Correct Decision |
| $\theta\in\omega_1,H_1$ is true | Correct Decision | Type II Error    |

**E.G.**:
- Type I Error : 無辜的人被判有罪。
- Type II Error: 有罪的人被判無罪。

我們的目標是最小化 Type I Error 和 Type II Error 發生的概率。

- 假設我們不能容忍 Type I Error $\implies$ 我們永不拒絕 $H_0$ $\implies$ 一定會發生 Type II Error。
- 假設我們不能容忍 Type II Error $\implies$ 我們永遠拒絕 $H_0$ $\implies$ 一定會發生 Type I Error。

:::danger[Note]
$$
P(\text{Type I eror})+P(\text{Type II Error})\neq 1
$$

它們的參數是不同的。Type I Error 是在 $H_0$ 是真的情況下發生的概率，而 Type II Error 是在 $H_1$ 是真的情況下發生的概率。
:::

通常，我們會認為 Type I error 更嚴重。因此我們或設定一個底線 $\alpha$，即發生 Type I error 的機率不超過 $\alpha$。而在不越過底線的前提下，找可以使 Type II error 最小的決定策略。

**Note**: test function $\phi$ 是在得到數據 $\utilde{X}=\utilde{x}$ 下，拒絕 $H_0$ 的概率,i.e.:

$$
\phi(\utilde{X})=P(\text{rej }H_0|\utilde{X}=\utilde{x})=E[I(\text{rej }H_0)|\utilde{X}=\utilde{x}]
$$

$$
\implies E_\theta\phi(\utilde{X})=E_\theta(E[I(\text{rej }H_0)|\utilde{X}])=E_\theta[I(\text{rej }H_0)]=P_\theta(\text{rej }H_0)
$$

$$
\begin{align*}
    
  &\begin{align*}
      \implies& E_\theta\phi(\utilde{X}) \quad \theta\in\omega_0 \iff H_0 \text{ is true} \\
      =&P_\theta(\text{rej }H_0)=P(\text{rej }H_0|H_0\text{ is true})\\
      =&P(\text{Type I Error})
  \end{align*}\\
  &\text{and}\\
  &\begin{align*}
      \implies& E_\theta\phi(\utilde{X}) \quad \theta\in\omega_1 \iff H_1 \text{ is true} \\
      =&P_\theta(\text{rej }H_0)=P(\text{rej }H_0|H_1\text{ is true})=1-P(\text{not rej }H_0|H_1\text{ is true})\\
      =&1-P(\text{Tpye II Error})
  \end{align*}
\end{align*}
$$

:::info[Definition]
$$
\text{Power Function: } \beta_\phi(\theta)=E_\theta\phi(\utilde{X})\quad \theta\in\omega_1
$$
:::

我們設定一個底線 $\alpha$。我們希望在保證 $\forall \theta\in\omega_0, E_\theta\phi(\utilde{X})\le \alpha$ 的前提下，最大化 $E_\theta\phi(\utilde{X}), \forall \theta\in\omega_1$。

i.e. $\max E_\theta\phi(\utilde{X}), \theta\in\omega_1$ s.t. $\sup_{\theta\in\omega_0}E_\theta\phi(\utilde{X})= \alpha$

:::info[Definition]
$\sup_{\theta\in\omega_0}E_\theta\phi(\utilde{X})=\alpha$ 被稱為檢定函數 $phi(\utilde{X})$ 的顯著水平（Significance Level）。而 $\phi(\utilde{X})$ 被稱為 level $\alpha$ 的檢定。
:::

令 $\theta_0$ 使得 $E_{\theta_0}\phi(\utilde{X})=\sup_{\theta\in\omega_0}E_\theta\phi(\utilde{X})=\alpha$，$\theta_0$ 通常會在 $\omega_0$ 的邊界。為了使 $\theta_0\in\omega_0$，我們需要讓 $\omega_0$ 是閉區間。因此檢定假設的等號會放在 $H_0$ 上。

## MP 檢定

:::info[Definition]
如果 $\sup_{\theta\in\omega_0}E_\theta \phi^*(\utilde{X})=\alpha$ 是 level $\alpha$ 的檢定，並且對於所有的 level $\alpha$ 的檢定 $\phi(\utilde{X})$ 有

$$
E_\theta\phi^*(\utilde{X})\ge E_\theta\phi(\utilde{X})\quad \forall \underbrace{\theta\in\omega_1}_{\text{uniformly}}
$$

則 $\phi^*(\utilde{X})$ 被稱爲 level $\alpha$ 的 *最強檢定（Uniformly Most Powerful Test, UMP Test）*。

如果 $H_1$ 是 simple，則 UMP 被稱為 *MP* 檢定。
:::

### Neyman-Pearson Lemma

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

## UMP 檢定

### 單邊問題（One-Sided Problem）

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

### MLR

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

### 1-par exp family

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

### 雙邊問題（Two-Sided Problem）

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

### UMPU Test

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

## Likelihood Ratio Test(LRT)

**Recall**: $L(\theta;\utilde{x})=f(\utilde{x};\theta)$ a function of $\theta$ given $\utilde{x}$. 在 $\theta$ 下出現當前樣本的機率。

**Note**: MP test rejects $H_0=\theta=\theta_0$ if

$$
\begin{align*}
    &f(\utilde{x};\theta_1)>c\cdot f(\utilde{x};\theta_0)\\
    \iff &L(\theta_1;\utilde{x})>c\cdot L(\theta_0;\utilde{x})\\
    \iff &\frac{L(\theta_0;\utilde{x})}{L(\theta_1;\utilde{x})}<\frac{1}{c}=c'\\
    \iff &\frac{L(\theta_0;\utilde{x})}{\max\set{L(\theta_0;\utilde{x}), L(\theta_1;\utilde{x})}}<k\quad\text{some } k\in[0,1]\\
    &=\begin{cases}
        \frac{L(\theta_0;\utilde{x})}{L(\theta_1;\utilde{x})} &\text{if } L(\theta_1;\utilde{x}) \ge L(\theta_0;\utilde{x})\\
        1 &\text{if } L(\theta_1;\utilde{x})<L(\theta_0;\utilde{x})
    \end{cases}\\
    \iff &\frac{\sup_{\theta\in\omega_0 L(\theta;\utilde{x})}}{\sup_{\theta\in\Omega L(\theta;\utilde{x})}}<k \quad \Omega=\omega_0\cup\omega_1\text{ and } \omega_0=\set{\theta_0}, \omega_1=\set{\theta_1}
\end{align*}
$$

:::info[Definition]
1. For testing $H_0:\theta\in\omega_0$ v.s. $H_1:\theta\in\omega_1$, a LRT is a test which $\phi$ rejects $H_0$ if $\lambda(\utilde{x})<k$
   $$
   \text{with }k\in[0,1]\text{ and }\lambda(\utilde{x})=\frac{\sup_{\theta\in\omega_0}L(\theta;\utilde{x})}{\sup_{\theta\in\Omega}L(\theta;\utilde{x})}\text{ where }\Omega=\omega_0\cup\omega_1
   $$
2. A level $\alpha$ LRT $\implies$ k is determined by $\sup_{\theta\in\omega_0}E_\theta\phi(\utilde{x})=\alpha$
:::


**EX** $X\sim f(x;\theta)$

$$
H_0:\theta\in\set{\theta_1,\theta_2} \text{ v.s. }H_1:\theta\in\set{\theta_3,\theta_4}
$$

with level 0.05

| x          | 2              | 3              | 4             | 5              | 6             | 7             | 8             | 9             | 10            | 11            | 12        |
| ---------- | -------------- | -------------- | ------------- | -------------- | ------------- | ------------- | ------------- | ------------- | ------------- | ------------- | --------- |
| $\theta_1$ | (0.01)         | (0.01)         | (0.01)        | (0.01)         | (0.01)        | (0.01)        | (0.01)        | (0.01)        | (0.01)        | (0.01)        | 0.90      |
| $\theta_2$ | (0.01)         | 0.009          | 0.008         | 0.007          | 0.006         | 0.005         | 0.006         | 0.007         | 0.008         | 0.009         | [(0.925)] |
| $\theta_3$ | 0.20           | [0.10]         | [0.09]        | 0.08           | 0.06          | 0.05          | [0.07]        | 0.05          | 0.05          | [0.05]        | 0.20      |
| $\theta_4$ | [0.26]         | [0.10]         | [0.09]        | [0.11]         | [0.07]        | [0.08]        | [0.07]        | [0.06]        | [0.06]        | [0.05]        | 0.05      |
| $\lambda=$ | $\frac{1}{26}$ | $\frac{1}{10}$ | $\frac{1}{9}$ | $\frac{1}{11}$ | $\frac{1}{7}$ | $\frac{1}{8}$ | $\frac{1}{7}$ | $\frac{1}{6}$ | $\frac{1}{6}$ | $\frac{1}{5}$ | $1$       |

$$
\lambda(x)=\frac{\sup_{\theta\in\omega_0}L(\theta;x)}{\sup_{\theta\in\Omega}L(\theta;x)}<k\implies k=\frac{1}{7}\quad\text{i.e. }x=2,3,4,5,7
$$

$$
P_{\theta_1}(x=2\text{ or }3\text{ or }4\text{ or }5\text{ or }7)=0.05
$$

---

**Note**: 

1. Let $\widehat{\theta_\Omega}=\arg\sup_{\theta\in\Omega}L(\theta;\utilde{x}), \widehat{\theta_{\omega_0}}=\arg\sup_{\theta\in\omega_0}L(\theta;\utilde{x})$

   $$
   \implies\lambda(\utilde{x})=\frac{\sup_{\theta\in\omega_0}L(\theta;x)}{\sup_{\theta\in\Omega}L(\theta;x)}=\frac{L(\widehat{\theta_{\omega_0}};\utilde{x})}{L(\widehat{\theta_\Omega};\utilde{x})}
   $$
2. $T=T(\utilde{X})$ is sufficient for $\theta$
   $\implies f(\utilde{x};\theta)=g(t;\theta)h(\utilde{x})$

   如果最大值發生在 $\omega_0$，那麼 $\lambda(\utilde{x})=1$；如果最大值發生在 $\omega_1$，那麼 $\omega_0$ 的最大值會發生在邊界 $\partial(\omega_0)$ 上。

   $$
    \begin{align*}
       \lambda(\utilde{x})&=\frac{g(t;\widehat{\theta_{\omega_0}})}{g(t;\widehat{\theta_\Omega})}\\
       &=\begin{cases}
         1 &\text{if } \widehat{\theta_\Omega}\in\omega_0\\
         \frac{g(t;\partial(\omega_0))}{g(t;\widehat{\theta_\Omega})} &\text{if } \widehat{\theta_\Omega}\in\omega_1
       \end{cases}
    \end{align*}
   $$

---

**EX**: $X_1, \cdots, X_n\overset{\text{iid}}{\sim} N(\theta, \sigma^2_0)$, to test

$$
H_0:\theta=\theta_0\quad\text{ v.s. }\quad H_1:\theta\ne\theta_0
$$

note that $\omega_0=\set{\theta_0}\implies\widehat{\theta_{\omega_0}}=\theta_0$ and $\Omega=\set{\theta_0}\cup\set{\theta:\theta\neq\theta}=\R\implies\widehat{\theta_\Omega}=\bar{x}$ 因為 $\bar{x}$ 是平均的 MLE。

$\implies$ reject $H_0$

$$
\begin{align*}
    \iff &\lambda(\utilde{x})=\frac{L(\theta_0;\utilde{x})}{L(\bar{x};\utilde{x})}<k\\
    \iff&\exp\left(-\frac{n(\bar{x}-\theta_0)^2}{2\sigma^2_0}\right)<k'\\
    \iff&-\frac{n(\bar{x}-\theta_0)^2}{2\sigma^2_0}<\ln k'\\
    \iff&\frac{n(\bar{x}-\theta_0)^2}{2\sigma^2_0}>C\\
    \iff& \left|\frac{\sqrt{n}(\bar{x}-\theta_0)}{\sigma_0}\right|>D
\end{align*}
$$

and

$$
\alpha=P_{\theta_0}\left(\left|\frac{\sqrt{n}(\bar{x}-\theta_0)}{\sigma_0}\right|>D\right)=P(|Z|>D)
$$

I.e., A level $\alpha$ LRT is reject $H_0$ if $|\frac{\sqrt{n}(\bar{x}-\theta_0)}{\sigma_0}|>Z_{\alpha/2}$

---

**EX**: $X_1, \cdots, X_n\overset{\text{iid}}{\sim}N(\theta, \sigma^2)$ with $\theta, \sigma^2$ are both unknown.

To test$H_0:\theta=\theta_0$ v.s. $H_1:\theta\neq\theta_0$

$$
\omega_0=\set{(\theta, \sigma^2):\theta=\theta_0, \sigma^2>0}\implies \widehat{\theta_{\omega_0}}=\theta_0, \widehat{\sigma^2_{\omega_0}}=\frac{1}{n}\sum_{i=1}^n(x_i-\theta_0)^2
$$

$$
\Omega=\set{(\theta, \sigma^2):\theta\neq\theta_0, \sigma^2>0}\implies \widehat{\theta_\Omega}=\bar{x}, \widehat{\sigma^2_\Omega}=\frac{1}{n}\sum_{i=1}^n(x_i-\bar{x})^2
$$

$$
\begin{align*}
    &\begin{align*}
        L(\widehat{\theta_{\omega_0}}, \widehat{\sigma^2_{\omega_0}};\utilde{x})&=\left(\frac{1}{2\pi\widehat{\sigma^2_{\omega_0}}}\right)^{n/2}\exp\left(-\frac{1}{2\widehat{\sigma^2_{\omega_0}}}\sum_{i=1}^n(x_i-\theta_0)^2\right)\\
        &=\left(\frac{1}{2\pi}\right)^{n/2}\left(\frac{1}{\widehat{\sigma^2_{\omega_0}}}\right)^{n/2}\exp\left(-\frac{n}{2}\right)
    \end{align*}\\
    &\begin{align*}
        L(\widehat{\theta_\Omega}, \widehat{\sigma^2_\Omega};\utilde{x})&=\left(\frac{1}{2\pi\widehat{\sigma^2_\Omega}}\right)^{n/2}\exp\left(-\frac{1}{2\widehat{\sigma^2_\Omega}}\sum_{i=1}^n(x_i-\bar{x})^2\right)\\
        &=\left(\frac{1}{2\pi}\right)^{n/2}\left(\frac{1}{\widehat{\sigma^2_\Omega}}\right)^{n/2}\exp\left(-\frac{n}{2}\right)
    \end{align*}
\end{align*}
$$

$$
\begin{align*}
    \text{reject }H_0&\iff\lambda(\utilde{x})=\frac{L(\widehat{\theta_{\omega_0}}, \widehat{\sigma^2_{\omega_0}};\utilde{x})}{L(\widehat{\theta_\Omega}, \widehat{\sigma^2_\Omega};\utilde{x})}=\left(\frac{\widehat{\sigma^2_\Omega}}{\widehat{\sigma^2_{\omega_0}}}\right)^{n/2}<k\\
    &\iff\left(\frac{\sum_{i=1}^n(x_i-\bar{x})^2}{\sum_{i=1}^n(x_i-\theta_0)^2}\right)^{n/2}<k\\
    &\iff\frac{\sum(x_i-\bar{x})^2}{\sum(x_i-\bar{x}+\bar{x}-\theta_0)^2}=\frac{\sum(x_i-\bar{x})^2}{\sum(x_i-\bar{x})^2+n(\bar{x}-\theta_0)^2}=\frac{1}{1+\frac{n(\bar{x}-\theta_0)^2}{\sum(x_i-\bar{x})^2}}<k'\\
    &\iff\frac{n(\bar{x}-\theta_0)^2}{\frac{\sum(x_i-\bar{x})^2}{n-1}}>C\\
    &\iff\frac{n(\bar{x}-\theta_0)^2}{s^2}>C\qquad \bar{X}\sim N(\theta, \sigma^2/n)\perp \frac{(n-1)S^2}{\sigma^2}\sim\chi^2_{n-1}\\
    &\iff\left|\frac{\sqrt{n}(\bar{x}-\theta_0)}{s}\right|>D \qquad \left|\frac{\sqrt{n}(\bar{x}-\theta_0)}{s}\right| \sim t_{n-1}
\end{align*}
$$

I.e. a level $\alpha$ LRT is reject $H_0$ if $|\frac{\sqrt{n}(\bar{x}-\theta_0)}{s}|>t_{n-1,\alpha/2}$

---

**EX**: $X_1, \cdots, X_n\overset{\text{iid}}{\sim}f(x;\theta)=e^{-(x-\theta)}, x\ge\theta$

A level $\alpha$ for $H_0:\theta\le\theta_0$ v.s. $H_1:\theta>\theta_0$

Note $L(\theta;\utilde{x})=\prod_{i=1}^n\left(e^{-(x_i-\theta)}I(x_i\ge\theta)\right)=e^{-\sum x_i}e^{n\theta}I(x_{(1)}\ge\theta)$.

在 $\theta\le x_{(1)}$ 時，$L(\theta;\utilde{x})$ 是沿著 $\theta$ 遞增的，而在 $\theta>x_{(1)}$ 時，$L(\theta;\utilde{x})=0$。如果 $\theta_0>x_{1}$，那麼在 $\theta<\theta_0$ 的範圍內，$L(\theta=x_{(1)},\utilde{x})$ 會是最大值。如果 $\theta_0\le x_{(1)}$，那麼 $L(\theta_0,\utilde{x})$ 會是最大值。

$$
\begin{align*}
    &\implies\sup_{\theta\le\theta_0}L(\theta;\utilde{x})=\begin{cases}
        e^{-\sum x_i}e^{nx_{(1)}} &\text{if }\theta_0 > x_{(1)}\\
        e^{-\sum x_i}e^{n\theta_0} &\text{if }\theta_0\le x_{(1)}
    \end{cases}\\
    &\implies\sup_{\theta\in\Omega}L(\theta;\utilde{x})=e^{-\sum x_i}e^{nx_{(1)}}\\
\end{align*}
$$

$\implies$ LRT is reject $H_0$ if

$$
\begin{align*}
    &\lambda(\utilde{x})=\frac{\sup_{\theta\le\theta_0}L(\theta;\utilde{x})}{\sup_{\theta\in\Omega}L(\theta;\utilde{x})}=\begin{cases}
        1 &\text{if }\theta_0>x_{(1)}\\
        e^{n(\theta_0-x_{(1)})} &\text{if }\theta_0\le x_{(1)}
    \end{cases}<k\in(0,1)\\
    \iff & e^{n(\theta_0-x_{(1)})}<k\quad\text{and }\theta_0\le x_{(1)}\\
    \iff & \theta_0\le c < x_{(1)}
\end{align*}
$$

$$
\begin{align*}
    \alpha&=P_{\theta_0}(X_(1)>c)\\
    &=\left(P_{\theta_0}(X_1>c)\right)^n\\
    &=\left(\int_c^\infty e^{-(x-\theta_0)}dx\right)^n\\
    &=\left(e^{-(c-\theta_0)}\right)^n\\
    &=e^{n(\theta_0-c)}\\
\end{align*}
$$

$$
\implies c = \theta_0-\frac{\ln\alpha}{n} > \theta_0
$$

這與我們用 MLR 得到的 UMP test 是一樣的。

---

**EX**: 

$$
\perp\Big < \begin{align*}
    X_1, \cdots, X_n\overset{\text{iid}}{\sim}N(\theta_1, \sigma^2)\\
    Y_1, \cdots, Y_m\overset{\text{iid}}{\sim}N(\theta_2, \sigma^2)
\end{align*}
$$

$$
L(\theta_1, \theta_2;\utilde{x}, \utilde{y})=f_{\utilde{X}}(\utilde{x};\theta_1)\cdot f_{\utilde{Y}}(\utilde{y};\theta_2)=\theta_1^ne^{-\theta_1\sum x_i}\cdot\theta_2^me^{-\theta_2\sum y_i}
$$

$$
\omega_0=\set{(\theta_1, \theta_2);\theta_1=\theta_2>0}\qquad\Omega=\set{(\theta_1, \theta_2);\theta_1>0, \theta_2>0}
$$

Note

$$
\begin{align*}
    &W\sim\theta^ke^{-\theta \sum w_i}\qquad\ln W\sim k\ln\theta-\theta\sum w_i\\
    \implies&\frac{\partial}{\partial\theta}(k\ln\theta-\theta\sum w_i)=0\implies\frac{k}{\theta}-\sum w_i=0\implies\theta=\frac{k}{\sum w_i}=\frac{1}{\bar{W}}
\end{align*}
$$

在 $\omega_0$ 下

$$
\begin{align*}
    &L(\theta_1, \theta_2, \utilde{x}, \utilde{y})=L(\theta, \theta;\utilde{x}, \utilde{y})=\theta^{n+m}e^{-\theta(\sum x_i+\sum y_i)}\\
    \implies&\widehat{\theta_{\omega_0}}=\frac{n+m}{\sum_{i=1}^{n}x_i+\sum_{i=1}^{m}y_i}\\
    \implies&L(\widehat{\theta_{\omega_0}}, \widehat{\theta_{\omega_0}};\utilde{x}, \utilde{y})=\left(\frac{n+m}{\sum_{i=1}^{n}x_i+\sum_{i=1}^{m}y_i}\right)^{n+m}e^{-(n+m)}\\
\end{align*}
$$

在 $\Omega$ 下

$$
L(\widehat{\theta_{\Omega}}, \widehat{\theta_{\Omega}};\utilde{x}, \utilde{y})=\left(\frac{n}{\sum_{i=1}^{n}x_i}\right)^n\left(\frac{m}{\sum_{i=1}^{m}y_i}\right)^me^{-n}e^{-m}
$$

$\implies$ LRT is reject $H_0$ if

$$
\begin{align*}
    &\begin{align*}
        k>\lambda(\utilde{x}, \utilde{y})&=\frac{L(\widehat{\theta_{\omega_0}}, \widehat{\theta_{\omega_0}};\utilde{x}, \utilde{y})}{L(\widehat{\theta_{\Omega}}, \widehat{\theta_{\Omega}};\utilde{x}, \utilde{y})}\\
        &=\left(\frac{n+m}{\sum x_i+\sum y_i}\right)^{n+m}\left(\frac{\sum x_i}{n}\right)^n\left(\frac{\sum y_i}{m}\right)^me^{n+m}\\
        &=\left(\underbrace{\frac{\sum x_i}{\sum x_i+\sum y_i}}_{=T\in(0,1)}\right)^n\left(\underbrace{\frac{\sum y_i}{\sum x_i+\sum y_i}}_{=1-T}\right)^m(n+m)^{n+m}\frac{1}{n^n}\frac{1}{m^m}\\
    \end{align*}\\
    \iff& T^n(1-T)^m<c\\
    \iff& T<k_1\text{ or }T>k_2
\end{align*}
$$

Note $X\sim \exp(\theta_1)\xlongequal{\text{d}}\text{Gamma}(1, \frac{1}{\theta_1})\implies\sum^n x_i\sim \text{G}(n,\frac{1}{\theta_1})\implies 2\theta_1\sum^n x_i\sim\text{G}(n,2)\xlongequal{\text{d}}\chi^2_{2n}$

$$
\begin{align*}
    T&=\frac{\sum x_i}{\sum x_i+\sum y_i}=\frac{2\theta\sum x_i}{2\theta\sum x_i+2\theta\sum y_i}\\
    &=\frac{1}{1+\frac{2\theta\sum^m y/2m}{2\theta\sum^n x/2n}}\quad\text{where }\frac{2\theta\sum^m y/2m}{2\theta\sum^n x/2n}\xlongequal{\text{d}}\frac{\chi^2_{2m}/2m}{\chi^2_{2n}/2n}\sim F_{2m, 2n}
\end{align*}
$$

$$
\implies T<k_1\text{ or }T>k_2\iff \frac{\bar{Y}}{\bar{X}}<C_1 \text{ or }\frac{\bar{Y}}{\bar{X}}>C_2
$$

I.e. LRT is reject H_0 at level $\alpha$ if $\frac{\bar{Y}}{\bar{X}}<F_{2m,2n,1-\alpha/2}$ or $\frac{\bar{Y}}{\bar{X}}>F_{2m,2n,\alpha/2}$

## 單 normal distribution 的參數檢定

$$
X_1,\cdots, X_n\overset{\text{iid}}{\sim}N(\theta, \sigma^2)\in\text{ 2-par exp family with }\theta, \sigma^2\text{ unknown}
$$

**Recall**

$$
\perp\Bigg < \begin{align*}
    \hat{\theta}=\bar{X}&=\frac{1}{n}\sum_{i=1}^nX_i\sim N(\theta, \frac{\sigma^2}{n})\\
    \hat{\sigma^2}=S^2&=\frac{1}{n-1}\sum_{i=1}^n(X_i-\bar{X})^2\sim\chi^2_{n-1}\quad\text{with }\frac{(n-1)S^2}{\sigma^2}=\frac{\sum(X_i-\bar{X})^2}{\sigma^2}\sim\chi^2_{n-1}
    \end{align*}
$$

---

$$
H_0:\theta\overset{=}{\le}\theta_0\quad\text{ v.s. }\quad H_1:\theta>\theta_0
$$

UMPU level $\alpha$ test is rejects $H_0$ if 

$$
\underbrace{\frac{\bar{x}-\theta_0}{\sqrt{S^2/n}}}_{\overset{\theta=\theta_0}{\sim}t_{n-1}}=\frac{\sqrt{n}(\bar{x}-\theta_0)}{S}>t_{n-1,\alpha}
$$

事實上，這就是 LRT。

Note

$$
t_k=\frac{N(0,1)}{\sqrt{\chi^2_k/k}}\Big>\perp\text{ and }\chi^2_k=\sum^k \chi^2_1
$$

$$
\xRightarrow{\text{LLN}}\bar{X}\xrightarrow[k\to\infty]{P}1\implies t_k\xrightarrow[k\to\infty]{P}N(0,1)\implies t_{k,\alpha}\xrightarrow[k\to\infty]{P}Z_{\alpha}
$$

---

$$
H_0:\theta\ge\theta_0 \text{ v.s. } H_1:\theta<\theta_0
$$

UMPU level $\alpha$ test is reject $H_0$ if

$$
\frac{\sqrt{n}(\bar{x}-\theta_0)}{S}<-t_{n-1,\alpha}=t_{n-1,1-\alpha}
$$

事實上，這就是 LRT。

---

$$
H_0:\theta=\theta_0\quad\text{ v.s. }\quad H_1:\theta\neq\theta_0
$$

UMPU level $\alpha$ test is reject $H_0$ if

$$
\frac{\sqrt{n}(\bar{x}-\theta_0)}{S}>t_{n-1,\alpha/2}\text{ or }\frac{\sqrt{n}(\bar{x}-\theta_0)}{S}<-t_{n-1,\alpha/2}
$$

$$
\iff \left|\frac{\sqrt{n}(\bar{x}-\theta_0)}{S}\right|>t_{n-1,\alpha/2}
$$

---

$$
H_0:\sigma^2\overset{=}{\le}\sigma^2_0\quad\text{ v.s. }\quad H_1:\sigma^2>\sigma^2_0
$$

UMPU level $\alpha$ test is reject $H_0$ if

$$
\underbrace{\frac{(n-1)S^2}{\sigma^2_0}}_{\overset{\sigma^2=\sigma^2_0}{\sim}\chi^2_{n-1}}>\chi^2_{n-1,\alpha}
$$

事實上，它會是 UMP, LRT。

---

$$
H_0:\sigma^2\ge\sigma^2_0\quad\text{ v.s. }\quad H_1:\sigma^2<\sigma^2_0
$$

UMPU level $\alpha$ test is reject $H_0$ if

$$
\frac{(n-1)S^2}{\sigma^2_0}<\chi^2_{n-1,1-\alpha}
$$

---

$$
H_0:\sigma^2=\sigma^2_0\quad\text{ v.s. }\quad H_1:\sigma^2\neq\sigma^2_0
$$

UMPU level $\alpha$ test is 

$$
\phi(\utilde{x})=\begin{cases}
    1 & \text{if }\frac{(n-1)S^2}{\sigma^2_0}>k_1\text{ or }\frac{(n-1)S^2}{\sigma^2_0}<k_2\\
    0 & \text{otherwise}
\end{cases}
$$

with $k_1, k_2$ s.t. $E_{\sigma^2_0}\phi(\utilde{x})=\alpha$ and $E_{\sigma^2_0}[T\phi(\utilde{x})]=\alpha\cdot E_{\sigma^2_0}T=(n-1)\alpha$

$$
\text{i.e.}\qquad\int_{k_2}^{k_1}g_{n-1}(t) dt=1-\alpha \text{ and } \int_{k_2}^{k_1}tg_{n-1}(t) dt=(n-1)(1-\alpha) \text{ where }g_{n-1}\text{ is pdf of }\chi^2_{n-1}
$$

$\implies$ 只有數值解。此時我們通常會用 *equal-tailed test*，即左右拒絕區域面積都是 $\alpha/2$ （$\chi^2$ 不對稱）。

$$
\text{i.e.}\qquad\phi^*=\begin{cases}
    1 &\text{ if }\frac{(n-1)S^2}{\sigma^2_0}>\chi^2_{n-1,\alpha/2}\text{ or }\frac{(n-1)S^2}{\sigma^2_0}<\chi^2_{n-1,1-\alpha/2}\\
    0 &\text{ otherwise}
\end{cases}
$$

Fact: $\phi^*\rightarrow[n\to\infty]{D}\phi$

## 雙 normal distribution 的參數檢定

$$
\perp\Big<\begin{align*}
    X_1, \cdots, X_n\overset{\text{iid}}{\sim}N(\theta_x, \sigma^2_x)\\
    Y_1, \cdots, Y_m\overset{\text{iid}}{\sim}N(\theta_y, \sigma^2_y)
\end{align*}
$$

$$
\implies \perp\Bigg<\begin{align*}
    &\bar{X}-\bar{Y}\sim N(\theta_x-\theta_y, \frac{\sigma^2_x}{n}+\frac{\sigma^2_y}{m})\\
    &\frac{\sum^n(X_i-\bar{X})^2}{\sigma^2_x}+\frac{\sum^m(Y_i-\bar{Y})^2}{\sigma^2_y}\sim\chi^2_{n+m-2}
\end{align*}
$$

$$
\begin{align*}
    \implies& \frac{\bar{X}-\bar{Y}-(\theta_x-\theta_y)}{\sqrt{\frac{\sigma^2_x}{n}+\frac{\sigma^2_y}{m}}\sqrt{\left[\frac{\sum^n(X_i-\bar{X})^2}{\sigma^2_x}+\frac{\sum^m(Y_i-\bar{Y})^2}{\sigma^2_y}\right]/(n+m-2)}}\xlongequal{\text{d}}\perp\Big<\frac{N(0,1)}{\sqrt{\frac{\chi^2_{n+m-2}}{n+m-2}}}\sim t_{n+m-2}\\
    \xlongequal{\sigma^2_x=\sigma^2=\sigma^2_y}&\frac{\bar{X}-\bar{Y}-(\theta_x-\theta_y)}{\sqrt{(\frac{1}{n}+\frac{1}{m})S^2_p}}\sim t_{n+m-2}\quad S^2_p=\frac{(n-1)S^2_x+(m-1)S^2_y}{n+m-2}
\end{align*}
$$

---

$$
H_0:\sigma^2_x\le\tau_0\sigma^2_y\quad\text{ v.s. }\quad H_1:\sigma^2_x>\tau_0\sigma^2_y\iff H_0:\sigma^2_x/\sigma^2_y\le\tau_0\quad\text{ v.s. }\quad H_1:\sigma^2_x/\sigma^2_y>\tau_0
$$

$$
\frac{\hat{\sigma^2_x}}{\hat{\sigma^2_y}}=\frac{S^2_x/\sigma^2_x}{S^2_y/\sigma^2_y}\overset{\text{d}}{=}\frac{\chi^2_{n-1}/(n-1)}{\chi^2_{m-1}/(m-1)}\sim F_{n-1, m-1}
$$

UMPU level $\alpha$ test is reject $H_0$ if $\frac{S^2_x/\sigma^2_x}{S^2_y/\sigma^2_y}>F_{n-1,m-1,\alpha}\iff \frac{S^2_x}{S^2_y}>\tau_0\cdot F_{n-1,m-1,\alpha}$ with

$$
\beta(\frac{\sigma^2_x}{\sigma^2_y}=\tau>\tau_0)=P(\frac{S^2_x/\sigma^2_x}{S^2_y/\sigma^2_y}>\frac{\sigma^2_y}{\sigma^2_x}\tau_0\cdot F_{n-1,m-1,\alpha})=P(F_{n-1,m-1}>\frac{\tau_0}{\tau}\cdot F_{n-1,m-1,\alpha})
$$

---

$$
H_0:\sigma^2_x\ge\tau_0\sigma^2_y\quad\text{ v.s. }\quad H_1:\sigma^2_x<\tau_0\sigma^2_y
$$

UMPU level $\alpha$ test is reject $H_0$ if $\frac{S^2_x}{S^2_y}<\tau_0\cdot F_{n-1,m-1,1-\alpha}$

---

$$
H_0:\sigma^2_x=\tau_0\sigma^2_y\quad\text{ v.s. }\quad H_1:\sigma^2_x\neq\tau_0\sigma^2_y
$$

Usually, we use equal-tailed test. I.e. Reject $H_0$ if

$$
\frac{S^2_y}{S^2_x}>\frac{1}{\tau_0}\cdot F_{n-1,m-1,\alpha/2}\text{ or }\frac{S^2_y}{S^2_x}<\frac{1}{\tau_0}\cdot F_{n-1,m-1,1-\alpha/2}
$$

---

Assume $\sigma^2_x=\sigma^2_y=\sigma^2$

$$
H_0:\theta_x\le\theta_y\quad\text{ v.s. }\quad H_1:\theta_x>\theta_y
$$

UMPU level $\alpha$ test is reject $H_0$ if

$$
\underbrace{\frac{\bar{x}-\bar{y}}{\sqrt{S^2_p(\frac{1}{n}+\frac{1}{m})}}}_{\sim t_{n+m-2}}>t_{n+m-2,\alpha}
$$

---

$$
H_0:\theta_x=\theta_y\quad\text{ v.s. }\quad H_1:\theta_x\neq\theta_y
$$

UMPU level $\alpha$ test is reject $H_0$ if

$$
\left|\frac{\bar{x}-\bar{y}}{\sqrt{S^2_p(\frac{1}{n}+\frac{1}{m})}}\right|>t_{n+m-2,\alpha/2}\qquad\text{Called two-sample t-test}
$$

**Remark**:
1. 當 $\sigma^2_x\neq\sigma^2_y$ 時，目前還沒有結論。
2. $\sigma^2_x=\sigma^2=\sigma^2\implies X\sim N(\theta_x, \sigma^2)\perp Y\sim N(\theta_y, \sigma), E[S^2_x]=\sigma^2=E[S^2_y]=E[S^2_p]$, and
   $$
   \frac{(n-1)S^2_x}{\sigma^2}\sim\chi^2_{n-1}\quad\frac{(m-1)S^2_y}{\sigma^2}\sim\chi^2_{m-1}\quad\frac{(n+m-2)S^2_p}{\sigma^2}\sim\chi^2_{n+m-2}
   $$

   $\implies S^2_p$ 的方差會是三個中最小的。

