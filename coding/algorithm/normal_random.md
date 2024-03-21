# Normal 分佈的隨機生成器

假設我們現在有操作系統提供的 $U(0,1)$ 随机数生成器。

## Box-Muller 轉換

### 基本原理

$U_1, U_2 \stackrel{\text{iid}}{\sim} U(0,1)$ 且獨立，让

$$
\begin{align*}
X &= \sqrt{-2 \ln U_1} \cos(2\pi U_2) \\
Y &= \sqrt{-2 \ln U_1} \sin(2\pi U_2)
\end{align*}
$$

則 $X, Y \stackrel{\text{iid}}{\sim} N(0,1)$ 且獨立。这可以通过 Jacobian 计算得到。

$$
\begin{align*}
    
&\begin{align*}
\frac{\partial (X,Y)}{\partial (U_1, U_2)} &= 
\begin{vmatrix}

\frac{-2}{2U_1\sqrt{-2\ln U_1}} \cos(2\pi U_2) & \sqrt{-2\ln U_1} (-2\pi \sin(2\pi U_2)) \\
\frac{-2}{2U_1\sqrt{-2\ln U_1}} \sin(2\pi U_2) & \sqrt{-2\ln U_1} (2\pi \cos(2\pi U_2)) \\

\end{vmatrix}
\\
&= -\frac{1}{U_1} 2\pi  \cos^2(2\pi U_2) - \frac{1}{U_1} 2\pi  \sin^2(2\pi U_2) \\
&=-2\pi \frac{1}{U_1}

\end{align*}\\
&\implies J =  \frac{\partial (U_1, U_2)}{\partial (X,Y)}  =-\frac{U_1}{2\pi}

\end{align*}
$$

均勻分佈的機率在有效區間內是常數，所以我們不需要關心 $U_1,U_2$ 的反函數，但 $J$ 還是有包含 $U_1$，因此我們還是需要計算得到 $U_1=\exp{\frac{X^2+Y^2}{-2}}$，

$$
f_{(X,Y)}(x,y)=f_{(U_1,U_2)}(g_1(x,y), g_2(x,y))|J| = \frac{1}{2\pi} \exp{\frac{X^2+Y^2}{-2}} = \frac{1}{\sqrt{2\pi}} \exp{\frac{X^2}{-2}} \frac{1}{\sqrt{2\pi}} \exp{\frac{Y^2}{-2}}
$$

得到了兩個獨立的 normal 分佈的聯合概率密度函數，因此 $X, Y\stackrel{\text{iid}}{\sim} N(0,1)$ 獨立。

![](https://upload.wikimedia.org/wikipedia/commons/thumb/1/1f/Box-Muller_transform_visualisation.svg/300px-Box-Muller_transform_visualisation.svg.png)

以 $U_1$ 为例，可以把 $U_1$ 看做是 $U_1=R\cos(\theta)$。其中 $\theta\stackrel{\text{iid}}{\sim} U(0,2\pi)$ ，相当于是均匀的选取一个角度。而 $R^2=-2\ln U_1$ 服从卡方分布 $\chi^2_2$。

### 代碼實現

```rust
use rand::distributions::Standard;
use rand::prelude::*;
use std::f64::consts::PI;

fn box_muller() -> f64 {
    let mut u1: f64 = StdRng::from_entropy().sample(Standard);
    let mut u2: f64 = StdRng::from_entropy().sample(Standard);
    while u1 == 0.0 {
        u1 = StdRng::from_entropy().sample(Standard);
    }

    while u2 == 0.0 {
        u2 = StdRng::from_entropy().sample(Standard);
    }

    let z1 = (-2.0_f64 * u1.ln()).sqrt() * (2.0 * PI * u2).cos();
    return z1;
}
```