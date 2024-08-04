---
sidebar_position: 1
---

# 隨機變量的變換（Transformations of Random Variables）

## 單變量變換

**EX** $X\sim B(n,p)$, let $Y=n-X=h(X)$

$$
\implies f_Y(y)=P(Y=y)=P(n-X=y)=P(X=n-y)=\binom{n}{n-y}p^{n-y}(1-p)^y=\binom{n}{y}p^y(1-p)^{n-y}
$$

i.e. $Y\sim B(n,p)$

---

**EX** $X\sim N(\mu,\sigma^2)$

$$
Y=\begin{cases}
    1 & \text{if } X\ge \mu\\
    -1 & \text{if } X<\mu
\end{cases}
$$

$$
\implies f_Y(y)=P(Y=y)=
\begin{cases}
    P(X\ge \mu) & \text{if } y=1\\
    P(X<\mu) & \text{if } y=-1
\end{cases}=\frac{1}{2}
$$

---

對於連續的隨機變量變換, $X\sim f(x), Y=h(x)$

$\implies$ cdf of $Y$ is $FY(y)=P(Y\le y)$ and pdf of $Y$ is $f_Y(y)=\frac{d}{dy}F_Y(y)$


如果 $h(x)$ 是單調遞增，則 $h^{-1}(y)$ 存在，並且微分為正

$$
F_y(y)\triangleq P(Y\le y)=P(h(X)\le y)=P(X\le h^{-1}(y))=F_X(h^{-1}(y))
$$

$$
\implies f_Y(y)=\frac{d}{dy}F_Y(y)=\frac{d}{dy}F_X(h^{-1}(y))=f_X(h^{-1}(y))\underbrace{\frac{d}{dy}h^{-1}(y)}_{>0}
$$

如果 $h(x)$ 是單調遞減，則 $h^{-1}(y)$ 存在，並且微分為負

$$
F_y(y)\triangleq P(Y\le y)=P(h(X)\le y)=P(X\ge h^{-1}(y))=1-F_X(h^{-1}(y))
$$

$$
\implies f_Y(y)=\frac{d}{dy}F_Y(y)=\frac{d}{dy}(1-F_X(h^{-1}(y)))=-f_X(h^{-1}(y))\underbrace{\frac{d}{dy}h^{-1}(y)}_{<0}
$$

:::tip[Theorem]
如果 $X$ 有連續的 pdf $f_X(x)$， 並且 $h$ 是 1-1 ，且 $h'(x)$ 存在連續導數，則 $Y=h(X)$ 有 pdf

$$
f_Y(y)=f_X(h^{-1}(y))\left|\frac{d}{dy}h^{-1}(y)\right|, y\in \mathscr{Y}
$$

其中 $\mathscr{Y}=h(\mathscr{X}), \mathscr{X}=\set{x;f_X(x)>0}$
:::

**EX**: $X\sim N(\mu,\sigma^2)$, let $Y=h(X)=\frac{X-\mu}{\sigma}\iff X=\sigma Y+\mu=h^{-1}(y)\implies \frac{d}{dy}h^{-1}(y)=\sigma$

$$
\begin{align*}
    f_Y(y)&=f_X(\sigma y+\mu)\left|\frac{d}{dy}(\sigma y+\mu)\right|\\
    &=\frac{1}{\sqrt{2\pi}}e^{-\frac{y^2}{2}}\quad \forall y\in\R
\end{align*}
$$

$\implies Y\sim N(0,1)$

---

**EX**: Let $X\sim \text{Gamma}(\alpha,\beta)$ with $\alpha>0,\beta>0$ given, and $Y=\frac{2X}{\beta}=h(X)$

$\implies \mathscr{Y}=h(\mathscr{X})=(0,\infty)$ and $X=\frac{\beta Y}{2}h^{-1}(Y)$

$$
\begin{align*}
    f_Y(y)&=f_X(h^{-1}(y))\left|\frac{d}{dy}h^{-1}(y)\right|, y\in\mathscr{Y}\\
    &=f_X\left(\frac{\beta y}{2}\right)\left|\frac{\beta}{2}\right|, y>0\\
    &=\frac{1}{\Gamma(\alpha)\beta^\alpha}\left(\frac{\beta y}{2}\right)^{\alpha-1}\exp\left(-\frac{\beta y}{\beta}\cdot\frac{\beta}{2} \right)\\
    &=\frac{1}{\Gamma(\alpha)2^\alpha}y^{\alpha-1}e^{-y/2}, y>0\\
    &=\text{pdf of Gamma}(\alpha,2)
\end{align*}
$$

i.e. $X\sim \text{Gamma}(\alpha,\beta)\implies \frac{\beta^*X}{\beta}\sim \text{Gamma}(\alpha,\beta^*)$

---

**Remark**:
1. $X\sim\text{Gamma}(\frac{k}{2},2)=\chi^2_k$
2. $X\sim\text{Gamma}(\alpha,\beta), Y=\frac{1}{X}$
   
   $$
   \begin{align*}
       f_Y(y)=f_X(\frac{1}{y})\left|-\frac{1}{y^2}\right|=\frac{1}{\Gamma(\alpha)\beta^\alpha}(\frac{1}{y})^{\alpha+1}e^{-1/\beta y}, y>0
   \end{align*}
   $$

   $\implies y$ has a inverse Gamma dist

---

**EX**: $X\sim U(a,b)$, and $Y=\frac{X-a}{b-a}=h(X)$

$\implies \mathscr{Y}=(0,1), X=(b-a)Y+a=h^{-1}(y)$

$$
\begin{align*}
    \implies f_Y(y)&=f_X(h^{-1}(y))\left|\frac{d}{dy}h^{-1}(y)\right|, y\in\mathscr{Y}\\
    &=\frac{1}{b-a}\cdot (b-1)=1
\end{align*}
$$

i.e. $Y\sim U(0,1)$

---

**EX**: $U\sim U(0,1), X=-2\ln U=h(U)\in(0,\infty)\implies U=e^{-X/2}=h^{-1}(X)$

$$
\implies f_X(x)=f_U(h^{-1}(x))\left|\frac{d}{dx}h^{-1}(x)\right|=1\cdot\left|\frac{-1}{2}e^{-x/2}\right|=\frac{1}{2}e^{-x/2}, x>0
$$

i.e. $X\sim\chi^2_2\implies -2\ln U\sim \chi^2_2$

---

**EX**: $Z\sin N(0,1), Y=Z^2=h(Z)$，變換並不是 1-1

$$
h=\begin{cases}
    z^2 & z\ge 0\\
    z^2 & z<0
\end{cases}\implies h^{-1}=\begin{cases}
    \sqrt{z} & z\ge 0\\
    -\sqrt{z} & z<0
\end{cases}
$$

$$
\implies F_Y(y)=P(Y\le y)=P(Z^2\le y)=P(-\sqrt{y}\le Z\le \sqrt{y})=\Phi(\sqrt{y})-\Phi(-\sqrt{y})
$$

$$
\begin{align*}
    f_Y(y)&=\frac{d}{dy}F_Y(y)\\
    &=\frac{d}{dy}[\Phi(\sqrt{y})-\Phi(-\sqrt{y})]\\
    &=\phi(\sqrt{y})\frac{d}{dy}\sqrt{y}-\phi(-\sqrt{y})\frac{d}{dy}(-\sqrt{y})\\
    &=2\phi(\sqrt{y})\frac{d}{dy}\sqrt{y}=2\cdot\frac{1}{\sqrt{2\pi}}e^{-y/2}\cdot\frac{1}{2}y^{-1/2}\\
    &=\frac{1}{\sqrt{\pi}2^{1/2}}y^{\frac{1}{2}-1}e^{-y/2}, y>0\\
    &\propto \text{pdf of Gamma}(\frac{1}{2},2)
\end{align*}
$$

i.e. $Z^2\sim\text{Gamma}(\frac{1}{2},2)=\chi^2_1$

---

**Remark**: 如果發現兩個 pdf 函數成比例（變數相關部分相同 $\forall x$），則兩個 pdf 相同。

因為 pdf 積分為 1，如果變數部分相同，則常數部分一定相同。

:::tip[Theorem]
Let $X$ have a conti. pdf $f_X(x)$ and $\mathscr{X}=\set{x:f(x)>0}$ be the support of $X$, $Y=h(X)$.

Suppose $\exist\set{A_0,A_1,,\cdot,A_k}$ :partition of $\mathscr{X}$ s.t. $P(X\in A_0)=0$ and suppose $\exist h_i$ on $A_i,i=1,2,\cdots,k$ with

1. $h(x)=h_i(x),\forall x\in A_i$
2. $h_i$ is monotone on $A_i$
3. $h_i(A_i)=\set{y:y=h_i(x),x\in A_i},\forall i$
   
   $\mathscr{Y}=h(\mathscr{X})=\set{y:y=h(x),x\in\mathscr{X}}$
4. $h_i^{-1}(y)$ has a conti derivative

then pdf of $Y$ is

$$
f_Y(y)=\sum_{i=1}^k f_X(h_i^{-1}(y))\left|\frac{d}{dy}h_i^{-1}(y)\right|, y\in\mathscr{Y}
$$
:::

將 $X$ 的 pdf 分段，使得每段有其對應的 1-1 變換函數 $h_i$。將每段的 pdf 分別變換，然後合併。

**Idea**: $Y=F_X(X)\in[0,1]$

$$
F_Y(y)=P(Y\le y)=P(F_X(X)\le y)=P(X\le F_X^{-1}(y))=F_X(F_X^{-1}(y))=y
$$

$$
\implies f_Y(y)=\frac{d}{dy}F_Y(y)=1, y\in[0,1]
$$

是 $U(0,1)$ 的 pdf

:::tip[Theorem]
$U\sim U(0,1)$ 並且 $F$ 是 cdf （不要求連續，但單調遞增），定義 $F^{-1}(y)=\inf\set{x;F(x)\ge y}$

$\implies X\sim F$
:::

:::tip[Lemma]
上面定義的 $F^{-1}$ 會有以下性質：
1. $F^{-1}$ 是單調遞增的（不一定嚴格遞增）
2. $F(F^{-1}(Y))\ge y,\forall y\in(0,1)$
3. $F^{-1}(F(t))\le t,\forall t$
4. $F^{-1}(y)\le t\iff y\le F(t),\forall y\in(0,1),\forall t\in\R$

如果進一步， $F$ 是連續的，則 $F(F^{-1}(t))=t,\forall t$
:::

**Proof**:

$$
P(X\le X)=P(F^{-1}(u)\le x)=P(u\le F(x))=F(x)
$$

i.e. X 的 cdf 是 $F$

:::tip[Theorem]
**The prob integral transformation**

$X$ 有連續的 cdf $F_X$，令 $Y=F_X(X)\implies Y\sim U(0,1)$
:::

$$
\begin{align*}
    F_Y(y)&\triangleq P(Y\le y)\\
    &=P(F_X(X)\le y)\\
    &=P(F_X^{-1}(F_X(X))\le F_X^{-1}(y))\\
    &=P(\underbrace{F_X^{-1}(F_X(X))}_{\le X}\le F_X^{-1}(y), X\le F_X^{-1}(y))+P(\underbrace{F_X^{-1}(F_X(X))}_{\le X}\le F_X^{-1}(y), X> F_X^{-1}(y))\\
    &=P(X\le F_X^{-1}(y))+0\\
    &=F_X(F_X^{-1}(y))=y \quad\because F \text{ is conti.}
\end{align*}
$$

**Remark**: $U\sim U(0,1)\implies 1-U\sim U(0,1)$

$$
-2\ln U\sim \chi^2_2\implies -2\ln(1-U)\sim \chi^2_2
$$

i.e. $\forall F$ 是連續的 cdf 

$$
-2\ln F(X)\sim \chi^2_2\qquad -2\ln(1-F(X))\sim \chi^2_2
$$

## 多變量變換

:::tip[Theorem]
Let $X$ be a k-dim r.v. with conti. pdf $f_X(x)$ and $Y=h(X)=(Y_1,\cdots,Y_k)^t=\left(h_1(X),\cdots,h_k(X)\right)^t$

Suppose $h:\mathscr{X}\xrightarrow{1-1}\mathscr{Y}$ with $\mathscr{X}=\set{x:f_X(x)>0}$ and $\mathscr{Y}=h(\mathscr{X})$ s.t. $X=h^{-1}(y)=\left(g_1(y),\cdots,g_k(y)\right)^t$ exists and $\frac{\partial}{\partial y_i}g_j(y)$ exist and conti. $\forall i,j$

Then the pdf of $Y$ is

$$
f_Y(y)=f_Y(y1,\cdots,y_k)=f_X(h^{-1}(y))|J| \quad\text{where } J=\det\begin{bmatrix}
    \frac{\partial g_1}{\partial y_1} & \cdots & \frac{\partial g_1}{\partial y_k}\\
    \vdots & \ddots & \vdots\\
    \frac{\partial g_k}{\partial y_1} & \cdots & \frac{\partial g_k}{\partial y_k}
\end{bmatrix}_{k\times k}
$$
:::

**EX**: $X=(X_1,X_2,X_3,X_4)^t\sim f_X(x_1,x_2,x_3,x_4)=24e^{-x_1-x_2-x_3-x_4}$ with $0<x_1<x_2<x_3<x_4<\infty$, let

$$
\begin{align*}
    Y_1&=X_1\\
    Y_2&=X_2-X_1\\
    Y_3&=X_3-X_2\\
    Y_4&=X_4-X_3
\end{align*}\implies
\begin{align*}
    X_1&=Y_1\\
    X_2&=Y_1+Y_2\\
    X_3&=Y_1+Y_2+Y_3\\
    X_4&=Y_1+Y_2+Y_3+Y_4
\end{align*}
$$

$$
J=\det\begin{bmatrix}
    1 & 0 & 0 & 0\\
    1 & 1 & 0 & 0\\
    1 & 1 & 1 & 0\\
    1 & 1 & 1 & 1
\end{bmatrix}=1\neq 0
$$

$$
\begin{align*}
   f_Y(y_1,y_2,y_3,y_4)&=f_X(y_1,y_1+y_2,y_1+y_2+y_3,y_1+y_2+y_3+y_4)|J|\\
   &=4e^{-4y_1}3e^{-3y_2}2e^{-2y_3}e^{-y_4}
\end{align*}
$$

$$
\begin{align*}
    f_{Y_1}&=4e^{-4y_1}, &y_1>0\quad &Y_1\sim\text{Gamma}(1,\frac{1}{4})\\
    f_{Y_2}&=3e^{-3y_2}, &y_2>0\quad &Y_2\sim\text{Gamma}(1,\frac{1}{3})\\
    f_{Y_3}&=2e^{-2y_3}, &y_3>0\quad &Y_3\sim\text{Gamma}(1,\frac{1}{2})\\
    f_{Y_4}&=e^{-y_4}, &y_4>0\quad &Y_4\sim\text{Gamma}(1,1)
\end{align*}
$$

and $Y_1,Y_2,Y_3,Y_4$ are independent

**EX**: Given $X\sim\text{Gamma}(\alpha_1,\beta), Y\sim\text{Gamma}(\alpha_2,\beta)$ and $X\perp Y\iff f_{X,Y}(x,y)=f_X(x)f_Y(y)$

Let $T=X+Y, W=\frac{X}{X+Y}\implies X=TW, Y=T-TW$

$$
J=\det\begin{bmatrix}
    W & T\\
    1-W & -T
\end{bmatrix}=-TW-(1-W)T=-T
$$

$$
\begin{align*}
    f_{T,W}(t,w)&=f_{X,Y}(tw,t-tw)|J|\\
    &=f_X(tw)f_Y(t-tw)t\\\
    &=\frac{1}{\Gamma(\alpha_1)\beta^{\alpha_1}}(tw)^{\alpha_1-1}e^{-tw/\beta}\cdot\frac{1}{\Gamma(\alpha_2)\beta^{\alpha_2}}(t-tw)^{\alpha_2-1}e^{-(t-tw)/\beta}\cdot t\\
    &=\frac{1}{\Gamma(\alpha_1+\alpha_2)\beta^{\alpha_1+\alpha_2}}t^{\alpha_1+\alpha_2-1}e^{-t/\beta}\cdot\frac{\Gamma(\alpha_1+\alpha_2)}{\Gamma(\alpha_1)\Gamma(\alpha_2)}w^{\alpha_1-1}(1-w)^{\alpha_2-1}
\end{align*}
$$

$\implies T\perp W$ and $T\sim\text{Gamma}(\alpha_1+\alpha_2,\beta), W\sim\text{Beta}(\alpha_1,\alpha_2)$

i.e.

$$
\perp\bigg<\begin{align*}
    X&\sim\text{Gamma}(\alpha_1,\beta)\\
    Y&\sim\text{Gamma}(\alpha_2,\beta)\\
\end{align*}\implies\perp\bigg<
\begin{align*}
    X+Y&\sim\text{Gamma}(\alpha_1+\alpha_2,\beta)\\
    \frac{X}{X+Y}&\sim\text{Beta}(\alpha_1,\alpha_2)
\end{align*}
$$

$X_i\sim \text{Gamma}(\alpha_i,\beta),u=1,\cdots,k$ indep. 

$$
\implies\sum_{i=1}^k X_i\sim\text{Gamma}(\sum_{i=1}^k\alpha_i,\beta),\qquad l<k \quad\frac{\sum_{i=1}^l X_i}{\sum_{i=1}^k X_i}\sim\text{Beta}(\sum_{i=1}^l\alpha_i,\sum_{i=1}^k\alpha_i)
$$

---

**Remark** $X_1,\cdots,X_n\overset{\text{iid}}{\sim}$ conti cdf $F\implies F(X_i)\overset{\text{iid}}{\sim} U(0,1)$

$$
\implies -2\ln\prod_{i=1}^n F(X_i)=-2\sum_{i=1}^n\ln F(X_i)\sim\chi^2_{2n}
$$

---

**Remark**:

1. $Z_1,\cdots,Z_n\overset{\text{iid}}{\sim}N(0,1)\implies\sum_{i=1}^nZ^2_i\sim\chi^2_n$
   
   $\implies X_i\sim N(\mu_i,\sigma^2_i)$ indep. $\implies\sum_{i=1}^n\left(\frac{X_i-\mu_i}{\sigma_i}\right)^2\sim\chi^2_n$

2. $Z\sim N(0,1)\perp X\sim\chi^2_m$
   
   Let $T=\frac{Z}{\sqrt{X/m}},W=\sqrt{\frac{X}{m}}\implies \int f_{T,W}(t,w)dw=T$ 's pdf

   i.e $N(0,1)/\sqrt{\chi^2_m/m}\sim t_m\implies t_m^2=\frac{(N(0,1)^2)}{\chi^2_m/m}=\frac{\chi^2_1/1}{\chi^2_m/m}=F_{1,m}$

---

**EX**: $X,Y\overset{\text{iid}}{\sim} U(\alpha,\beta)$，求 $X+Y$ 的分佈

$\implies T=X+Y$, let $W=X$

$$
\begin{align*}
    X&=W\\
    Y&=T-W
\end{align*}\implies J=\det\begin{bmatrix}
    0 & 1\\
    1 & -1
\end{bmatrix}=-1
$$

$$
\implies f_{T,W}(t,w)=f_X(w)f_Y(t-w)=\frac{1}{(\beta-\alpha)^2}
$$

$$
\begin{align*}
    \alpha<w<\beta\\
    \alpha<t-w<\beta
\end{align*}\iff\begin{align*}
    \alpha<w<\beta\\
    t-\beta<w<t-\alpha
\end{align*}\iff\begin{align*}
    \max\set{\alpha,t-\beta}<w<\min\set{\beta,t-\alpha}
\end{align*}
$$

$$
\begin{align*}
    \implies f_T(t)&\triangleq\int_{-\infty}^\infty f_{T,W}(t,w)dw\\
    &=\int_{\max\set{\alpha,t-\beta}}^{\min\set{\beta,t-\alpha}}\frac{1}{(\beta-\alpha)^2}dw\\
    &=\frac{\left(\min\set{\beta,t-\alpha}-\max\set{\alpha,t-\beta}\right)}{(\beta-\alpha)^2}\\
    &=\begin{cases}
        \frac{1}{(\beta-\alpha)^2}(t-2\alpha) & 2\alpha\le t< \alpha+\beta\\
        \frac{1}{(\beta-\alpha)^2}(2\beta-t) & \alpha+\beta\le t< 2\beta
    \end{cases}
\end{align*}
$$

---

**EX**: $X\sim P(\lambda)\perp Y\sim P(\theta)$，求 $X+Y$ 的分佈

Let $T=X+Y$

$$
\begin{align*}
    f_T(t)&=P(X+Y=t)\\
    &=\sum_x P(X=x,Y=t-x)\\
    &=\sum_{x=0}^t \frac{e^{-\theta}\theta^{t-x}}{(t-x)!}\cdot\frac{e^{-\lambda}\lambda^x}{x!}\\
    &=e^{-(\lambda+\theta)}\cdot\frac{1}{t!}\underbrace{\sum_{x=0}^t\frac{t!}{(t-x)!x!}\lambda^x\theta^{t-x}}_{\text{Binomial theorem}}\\
    &=\frac{e^{-(\lambda+\theta)}(\lambda+\theta)^t}{t!}
\end{align*}
$$

i.e. $X_i\sim P(\lambda_i)$ indep. $\implies \sum X_i\sim P(\sum\lambda_i)$