# 假設檢定（Testing Hypotheses）

在得到數據 $\utilde{X}=(X_1, \cdots, X_n)\sim f(\utilde{x};\theta)$ with $\theta\in\Omega\subseteq\R^n$ 時，假設檢定問題旨在決定是否要拒絕 $H_0$ 虛無假設（null hypothese）。

## 假設

:::tip[Definition]
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
- $H-1$ 是*對立假設（attenative hypothese）*，是我們想要説明的假設。

**E.G.**:
- $H_0:$ 無罪 v.s. $H_1:$ 有罪
- $H_0:$ 及格 v.s. $H_1:$ 不及格
- $H_0:$ 不及格 v.s. $H_1:$ 及格

## 檢定函數（Test Function）

當我們拿到一組數據 $\utilde{X}=\utilde{x}$ ，希我們會需要在這組數據的基礎上利用檢定函數 $\phi$ 來決定是否拒絕 $H_0$。

:::tip[Definition]
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

:::tip[Definition]
$$
\text{Power Function: } \beta_\phi(\theta)=E_\theta\phi(\utilde{X})\quad \theta\in\omega_1
$$
:::

我們設定一個底線 $\alpha$。我們希望在保證 $\forall \theta\in\omega_0, E_\theta\phi(\utilde{X})\le \alpha$ 的前提下，最大化 $E_\theta\phi(\utilde{X}), \forall \theta\in\omega_1$。

i.e. $\max E_\theta\phi(\utilde{X}), \theta\in\omega_1$ s.t. $\sup_{\theta\in\omega_0}E_\theta\phi(\utilde{X})= \alpha$

:::tip[Definition]
$\sup_{\theta\in\omega_0}E_\theta\phi(\utilde{X})=\alpha$ 被稱為檢定函數 $phi(\utilde{X})$ 的顯著水平（Significance Level）。而 $\phi(\utilde{X})$ 被稱為 level $\alpha$ 的檢定。
:::

令 $\theta_0$ 使得 $E_{\theta_0}\phi(\utilde{X})=\sup_{\theta\in\omega_0}E_\theta\phi(\utilde{X})=\alpha$，$\theta_0$ 通常會在 $\omega_0$ 的邊界。為了使 $\theta_0\in\omega_0$，我們需要讓 $\omega_0$ 是閉區間。因此檢定假設的等號會放在 $H_0$ 上。

## UMP 檢定

:::tip[Definition]
如果 $\sup_{\theta\in\omega_0}E_\theta \phi^*(\utilde{X})=\alpha$ 是 level $\alpha$ 的檢定，並且對於所有的 level $\alpha$ 的檢定 $\phi(\utilde{X})$ 有

$$
E_\theta\phi^*(\utilde{X})\ge E_\theta\phi(\utilde{X})\quad \forall \underbrace{\theta\in\omega_1}_{\text{uniformly}}
$$

則 $\phi^*(\utilde{X})$ 被稱爲 level $\alpha$ 的 *最強檢定（Uniformly Most Powerful Test, UMP Test）*。

如果 $H_1$ 是 simple，則 UMP 被稱為 *MP* 檢定。
:::

## Neyman-Pearson Lemma

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