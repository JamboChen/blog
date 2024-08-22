---
sidebar_position: 2
---

# 次序統計量（Order Statistics）

$X_1,\cdots,X_n\overset{\text{iid}}{\sim}$ pdf $f(x)$

$\implies$ order stat $X_{(1)}\le X_{(2)}\le\cdots\le X_{(n)}$ not indep

Goal: find pdf of $X_{(k)}$

$$
f_{X_{(n)}}(x)=n\cdot f(x)\cdot F(x)^{n-1}
$$

$$
f_{X_{(1)}}(x)=n\cdot f(x)\cdot (1-F(x))^{n-1}
$$

$$
f_{X_{(k)}}(x)=\frac{n!}{(k-1)!(n-k)!}f(x)\cdot F(x)^{k-1}\cdot(1-F(x))^{n-k}
$$

以上公示可以解釋為四個部分：
- $\frac{n!}{(k-1)!(n-k)!}$: 將 $n$ 個 $X$ 分成三組，數量分別為 $k-1,1,n-k$ 的組合數。
- $F(x)^{k-1}$: 前 $k-1$ 個 $X$ 都小於等於 $x$ 的機率。
- $(1-F(x))^{n-k}$: 後 $n-k$ 個 $X$ 都大於 $x$ 的機率。
- $f(x)$: 第 $k$ 個 $X$ 剛好等於 $x$ 的機率。

joint pdf of $X_{(i)},X_{(j)},i<j$ is

$$
f_{X_{(i)},X_{(j)}}(s,t)=\frac{n!}{(i-1)!(j-i-1)!(n-j)!}f(s)f(t)F(s)^{i-1}[F(t)-F(s)]^{j-i-1}[1-F(t)]^{n-j}
$$

joint pdf of $X_{(1)},X_{(2)},\cdots,X_{(n)}$

$$
f_{X_{(1)},X_{(2)},\cdots,X_{(n)}(t_1,t_2,\cdots,t_n)}=n!f(t_1)f(t_2)\cdots f(t_n)=n!\prod_{i=1}^n f(t_i)\quad\text{where }t_1<t_2<\cdots<t_n
$$

---

**EX**: $X_1,\cdots,X_n\overset{\text{iid}}{\sim}U(0,1)$

$$
\begin{align*}
    f_{X_{(i)}}(x)&=\frac{n!}{(i-1)!(n-i)!}F(x)^{i-1}[1-F(x)]^{n-i}f(x)\\
    &=\frac{n!}{(i-1)!(n-i)!}x^{i-1}(1-x)^{n-i}\quad\text{where }0<x<1
\end{align*}
$$

i.e. $X_{i}\sim \text{Beta}(i,n-i+1)$

$Y_1,\cdots,Y_n\overset{\text{iid}}{\sim} N(\mu,\sigma^2)$

$$
\implies\Phi(\frac{Y_i-\mu}{\sigma})\sim U(0,1)\implies \Phi(\frac{Y_{(i)}-\mu}{\sigma})\sim \text{Beta}(i,n-i+1)
$$

---

**EX**: $X_1,\cdots,X_n\overset{\text{iid}}{\sim} \text{Exp}(\lambda)=\text{Gamma}(1,\frac{1}{\lambda})$

1. pdf of $X_{(1)}$
   $$
   \begin{align*}
       f_{X_{(1)}}(x)&=n(1-F(x))^{n-1}f(x)\\
       &=n(1-e^{-\lambda x})^{n-1}\lambda e^{-\lambda x}\\
       &=n\lambda e^{-n\lambda x}
   \end{align*}
   $$

   i.e. $X_{(1)}\sim \text{Exp}(n\lambda)$

2. jpdf of $X_1,\cdots,X_n$
   
   $$
   f_{X_{(1)},X_{(2)},\cdots,X_{(n)}}(t_1,t_2,\cdots,t_n)=n!\prod_{i=1}^n\lambda e^{-\lambda t_i}=n!\lambda^n e^{-\lambda\sum_{i=1}^n t_i}
   $$

---

$X_1,\cdots,X_n\overset{\text{iid}}{\sim} \exp(\lambda)$ 是事件發生的時間。將事件發生時間排序後，兩個事件相隔的時間就是等待時間。

$$
\begin{align*}
    Y_1&=X_{(1)}\\
    Y_2&=X_{(2)}-X_{(1)}\\
    \vdots\\
    Y_n&=X_{(n)}-X_{(n-1)}
\end{align*}\implies\begin{align*}
    X_{(1)}&=Y_1\\
    X_{(2)}&=Y_1+Y_2\\
    \vdots\\
    X_{(n)}&=Y_1+Y_2+\cdots+Y_n
\end{align*}\implies J=\det\begin{pmatrix}
    1&0&0&\cdots&0\\
    1&1&0&\cdots&0\\
    1&1&1&\cdots&0\\
    \vdots&\vdots&\vdots&\ddots&\vdots\\
    1&1&1&\cdots&1
\end{pmatrix}=1
$$

$$
\begin{align*}
    \implies f_{Y_1,\cdots,Y_n}(y_1,\cdots,y_n)&=f_{X_{(1)},\cdots,X_{(n)}}(y_1,y_1+y_2,\cdots,y_1+y_2+\cdots+y_n)|J|\\
    &=n!\lambda^n \exp(y_1+(y_1+y_2)+\cdots+(y_1+y_2+\cdots+y_n))\\
    &=(n\lambda)e^{-n\lambda y_1}\cdot (n-1)e^{-(n-1)\lambda y_2}\cdots \lambda e^{-\lambda y_n}\\
    &=\exp(n\lambda)\cdot\exp((n-1)\lambda)\cdots\exp(\lambda)
\end{align*}
$$

i.e. $Y_1,\cdot Y_n$ are independent $\text{Exp}(\lambda)$