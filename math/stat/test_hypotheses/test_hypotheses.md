---
sidebar_position: 3
---

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
