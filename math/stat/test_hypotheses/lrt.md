---
sidebar_position: 3
---

# Likelihood Ratio Test(LRT)

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
    X_1, \cdots, X_n\overset{\text{iid}}{\sim}\theta_1e^{-\theta_1x}, x>0\\
    Y_1, \cdots, Y_m\overset{\text{iid}}{\sim}\theta_2e^{-\theta_2y}, y>0
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
    &=\frac{1}{1+\frac{2\theta\sum^m y/2m}{2\theta\sum^n x/2n}\frac{m}{n}}\quad\text{where }\frac{2\theta\sum^m y/2m}{2\theta\sum^n x/2n}\xlongequal{\text{d}}\frac{\chi^2_{2m}/2m}{\chi^2_{2n}/2n}\sim F_{2m, 2n}
\end{align*}
$$

$$
\implies T<k_1\text{ or }T>k_2\iff \frac{\bar{Y}}{\bar{X}}<C_1 \text{ or }\frac{\bar{Y}}{\bar{X}}>C_2
$$

I.e. LRT is reject H_0 at level $\alpha$ if $\frac{\bar{Y}}{\bar{X}}<F_{2m,2n,1-\alpha/2}$ or $\frac{\bar{Y}}{\bar{X}}>F_{2m,2n,\alpha/2}$

