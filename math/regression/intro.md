---
sidebar_position: 0
---

# Introduction

## 回歸分析的步驟

```mermaid
flowchart LR
    a(start)-->b[Exploratory <br>data analysis] 
    b -->c[develop one or more <br>tentative reg models]
    c -->d{Is one or more of <br>the reg model <br> suitable for the data ?}
    d --Yes --> e[Identity the most<br> suitable reg model]
    d --No --> f[revise reg model <br>and/or <br>devep new one]
    f --> d
    e --> g[make infer based on<br> the final fitted model]
    g --> h(end)
```

## 統計關係

在一般的函數關係中，給定 X 的值，那麼對於的 Y 值是確定的。

$$
Y=f(X)\xlongequal{e.g.}\beta_0+\beta_1X
$$

![alt text](img/0-1.png)

但在統計關係中，X 和 Y 之前除了函數關係，還存在一定的擾動項，也就是誤差項。這個誤差項是一個隨機變量，並且我們假設它的平均是 0。

![alt text](img/0-2.png)

$$
Y=f(X)+\varepsilon\xlongequal{e.g.}\beta_0+\beta_1X+\varepsilon \qquad \text{with} \quad E(\varepsilon)=0
$$

$$
\implies E[Y] = E[f(X)] + E[\varepsilon] = f(X)
$$

$f(X)$ 是 $Y$ 的平均函數，也就是回歸函數