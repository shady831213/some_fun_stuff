![](https://www.ycombinator.com/packs/static/ycdc/ycombinator-logo-ee6c80faf1d1ce2491d8.png)

之前一直没搞明白 [Y combinator](https://en.wikipedia.org/wiki/Fixed-point_combinator#Y_combinator)这个东西咋来的，网上也基本上查不到其推导过程，只是证明其确实可以求函数的不动点。。。
最近主要基于[《递归函数论》](https://books.google.com.hk/books/about/%E9%80%92%E5%BD%92%E5%87%BD%E6%95%B0%E8%AE%BA.html?id=QaqjOAAACAAJ&redir_esc=y)学习递归论有了一些理解，记录一下。

注： 这里只考虑一元函数的情况，因为有配对函数存在的函数集，多元函数均可划归为一元函数。

# 枚举函数
半递归函数集存在自身枚举函数(《递归函数论》第6章 )，有的书里叫universal function，我理解是一个东西。大概意思为：
## 存在一二元半递归函数U(e, x)， 对所有一元半递归函数(可计算函数)f(x)，均存在自然数e, f(x) = U(e, x).

正是由于这个自身枚举函数的存在，导致了[哥德尔不完备定理](https://zh.wikipedia.org/zh-hans/%E5%93%A5%E5%BE%B7%E5%B0%94%E4%B8%8D%E5%AE%8C%E5%A4%87%E5%AE%9A%E7%90%86)。。。

g(x) = U(x, x)被大量应用与可计算理论中很多定理的证明中， 这个[对角线法](https://zh.wikipedia.org/wiki/%E5%B0%8D%E8%A7%92%E8%AB%96%E8%AD%89%E6%B3%95)本质上是一致的。

比如证明U(e, x)不可能是一般递归函数：
```
证明： 假设U(e,x)是一般递归函数，由于对所有f(x)，均有f(x) = U(e,x)，

则所有f(x)都是一般递归函数，既处处有定义，

则一元函数g(x) = U(x, x) + 1 也应处处有定义且存在e0使 g(x) = U(e0, x)

则g(e0) = U(e0, e0) + 1 = U(e0, e0), 矛盾，所以U(e,x)不可能是一般递归函数。
```

我的理解是U相当于function call或者eval，而e相当于f的指针。

设一函数p(f) = e将函数映射为index（一般这个工作由编译器或解释器完成），
则可做U1(f,x）= U(p(f), x) = f(x)。


# 不动点
## 对于所有一元半递归函数(可计算函数)f(x), 均存在不动点：
```
证明：设g(x) = U1(x, x) = x(x)

由于U(e, x)枚举所有f(x)，必有e使(f.g)(x) = U1(f.g, x) = U(e, x)

f(g(x)) = (f.g)(x) = U1(f.g, x)

将f.g带入x，f(g(f.g)) = (f.g)(f.g) = U1(f.g, f.g) = g(f.g)

g(f.g)为f的不动点。
```
# Y combinator

因为Y combinator应能求f的不动点
```
Y(f) = g(f.g) = (f.g)(f.g)

= (lambda x. (f.g)(x))lambda x. (f.g)(x)    // f.g = lambda x. (f.g)(x)

= (lambda x. fxx) lambda x. fxx   // g(x) = x(x)

Y = lambda f. (lambda x. fxx) lambda x. fxx
```
这样就得到了Y combinator. Ah~Ha! 
