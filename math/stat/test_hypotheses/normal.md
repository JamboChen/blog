# 常態分佈相關檢定

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
\frac{S^2_x/\sigma^2_x}{S^2_y/\sigma^2_y}\overset{\text{d}}{=}\frac{\chi^2_{n-1}/(n-1)}{\chi^2_{m-1}/(m-1)}\sim F_{n-1, m-1}
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

---

當 X 和 Y 的資料量都為 n 時

$$
H_0:\theta_x=\theta_y\quad\text{ v.s. }\quad H_1:\theta_x\neq\theta_y
$$

UMPU level $\alpha$ test is reject $H_0$ if

$$
\left|\frac{\bar{x}-\bar{y}}{S_p\sqrt{2/n}}\right|>t_{2n-2,\alpha/2}
$$

### Paired Data

有時資料成對出現的，比如同一個人在不同時間的數據。此時，每對之間的數據可能會是相關的，而不同對之間的數據仍然會是獨立的。

$$
\begin{pmatrix}
    X_1\\ Y_1
\end{pmatrix}, \begin{pmatrix}
    X_2\\ Y_2
\end{pmatrix}, \cdots, \begin{pmatrix}
    X_n\\ Y_n
\end{pmatrix}
\overset{\text{iid}}{\sim} N(\theta_x, \theta_y, \sigma^2_x, \sigma^2_y, \rho)
$$

$$
H_0:\theta_x=\theta_y\quad\text{ v.s. }\quad H_1:\theta_x\ne\theta_y
$$

$$
\implies D_i=Y_i-X_i, i=1,2,\cdots,n\overset{\text{id}}{\sim} N(\theta_x-\theta_y, \sigma^2)
$$

這樣我們就把問題轉換成了單樣本的問題。其中 $\sigma^2=\sigma^2(\sigma^2_x, \sigma^2_y, \rho)$ 的函數，但因為 $\sigma^2_x, \sigma^2_y, \rho$ 都是未知的，所以 $\sigma^2$ 也是未知的。

$$
\implies H_0:\mu=0\quad\text{ v.s. }\quad H_1:\mu\ne 0
$$

UMPU level $\alpha$ test reject $H_0$ if

$$
\left|\frac{\sqrt{n}(\bar{Y}-\bar{X})}{S_p}\right|>t_{n-1,1-\frac{\alpha}{2}}\quad\text{ where }S_p^2=\frac{\sum^n_1(D_i-\bar{D})}{n-1}
$$

稱為 *paired t-test*。
