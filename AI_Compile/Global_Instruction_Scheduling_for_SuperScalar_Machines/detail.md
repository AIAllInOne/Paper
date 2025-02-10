# 计算过程
```
CL.0:
(I1) L r12=a(r31,4) 1oad u
(I2) LU rO, r31=a(r31,8) load v and increment index
(I3) C cr7=r12, r0 U>v
(I4) BF CL.4, cr7,0x2/gt
--------------------------------------- END BL1
(I5) C cr6=r12, r30 u > max
(I6) BF CL.6,cr6,0x2/gt
--------------------------------------- END BL2
(I7) LR r30=r12 max = u
--------------------------------------- END BL3
CL.6:
(I8) C cr7=r0,r28 v < min
(I9) BF CL.9,cr7,0xl/lt
--------------------------------------- END BL4
(I10) LR r28=r0 min = v
(I11) B CL.9
--------------------------------------- END BL5
CL.4:
(I12) C cr6=r0,r30 v > max
(I13) BF CL.11,cr6,0x2/gt
--------------------------------------- END BL6
(I14) LR r30=r0 max = v
--------------------------------------- END BL7
CL. 11:
(I15) C cr7=r12,r28 u < min
(I16) BF CL.9,cr7,0xl/lt
--------------------------------------- END BL8
(I17) LR r28=r12 min = u
--------------------------------------- END BL9
CL.9:
(I18) AI r29=r29,2 i =i+2
(I19) C cr4=r29,r27 i<n
(I20) BT CL.0,cr4,0xl/lt
```

##  we estimate that the code executes in 20, 21 or 22 cycles,

I1, I2, [], I3, [], [], [], [I4, I12], [], [], [], [I13, I15], [], [], [], [I16, I18], I19, [], [], [], I20 > 21




## 4.2数据依赖

```
(I1) L r12=a(r31,4) 1oad u
(I2) LU rO, r31=a(r31,8) load v and increment index
(I3) C cr7=r12, r0 U>v
(I4) BF CL.4, cr7,0x2/gt
```
(1,2)  反依赖

(1, 2) (1, 3)  流数据依赖  (I3) 分别使用了 (I1) 和 (I2) 中定义的 r12 和 rO。

(2, 3) 带有一个周期的延迟，因为 (I2) 是加载指令（延迟加载）但(1,3)省略，因为已经计算了(1,2)和(2,3)

(3, 4) 流数据依赖 因为 (I3) 设置了 (I4) 中使用的 cr7。

所以最后计算 (1,2)(2,3) (3,4) 


## 5.2delay heuristic

1. D(i) = 0,  D1 = 0, D2 = 0, D3 = 0 D4 = 0

2. D(I) = max((D(J1) + d(I,J1)),(D(J2 + d(I,J2), ... )))

D4 = 0

D3 = Max(D(4) + d(3,4)) = 3

D2 = Max(D3 + d(2,3)) = Max(3+1) = 4

D1 = Max(D2 + d(1,2)) = Max(4+0) = 4

## 5.2 critical path heuristic

1. E(i) = 0
E(1) = 1, E(2) = 1, E(3) = 1， E(4) = 1

2. CP(i) = E(i)
CP1 = 1, CP(2) = 1, CP(3) = 1， CP(4) = 1

3. CP(I) = max((CP(J1) + d(I,J1)), (CP(J2) + d(I,J2)), ...) + E(I)


CP4 = max() + E4 = 1

CP3 = max(CP4 + d(3,4)) + E3 = (1 + 3) + 1= 5

CP2 = max(CP3 + d(2,3)) = (5 + 1) + 1 = 7

CP1 = max(CP2 + d(1,2)) = (7 + 0) + 1 = 8


## 5.1 The top-level process

拓扑序访问 block
_EQUIV(A)_   A 等同且由 A 主导的块集（参见定义 3

EQUIV(1) = 10

EQUIV(2) = 4

EQUIV(6) = 8



C(A)：
 Useful instructions only: _C(A)= EQUIV(A)_;
1-分支推测：_C(A)_ 包含以下块：
   a. the blocks of EQUIV(A);
   a. EQUIV(A) 的块；
   b. All the immediate successors of A in CSPDG;
   b. CSPDG 中 A 的所有直接后继；
   c. All the immediate successors of blocks in _EQUIV(A)_ in CSPDG.
   c. CSPDG 中 _EQUIV(A)_ 中块的所有直接后继。

useful

C(1) = 10 C(2) = 4 C(6) = 8

1-branch

C(1) = 10,2,4,6,8

C(2) = 4, 3

C(4) = 5

## 5.4 example

### useful

A1

EQUIV(1) = B10

C(1) = B10

候选指令集

1,2,3,4,18,19,20

调度

1,2,18,3,19,4

A1 完成

同理

18,19  B10->B1

8 B4->B2

15 B8->B6

最后
1,2,18,3,19,4,5,8,6,7,9,10,11,12,15,13,14,16,17,20

#### Theresultantprogram in Figure 5 takes 12- 13 cycles per iteration

I1, I2, I18, I3, I19, I5, I12, [I4, I15], [](I13 依赖的cr6 定义在 I5， 3个 delay), [](12-13 delay), I13, I16, I20  -> 12 cycles（不算20）



### useful and speculative

EQUIV(1) = B10

C(1) = B10, B2, B4


候选指令集

1,2,3,4,18,19,20,5,6,8,9,12,13,15,16

A1调度

1,2,18,3,19,5,12,4

A1完成

EQUIV(2) = 4
C(4) = B4, B3

候选指令集
5,6,7,8,9

A2 调度

5 已经完成 6 未就绪 9未就绪， 判断7,8 优先级

U(2) = B2 U EQUAIV(2) = B2 U B4 = 5,6,8,9

根据规则1, If B(I) 属于 U(A) and B(J)不属于 U(A), then pick I

选8，

所以 B2调度 8,6

A2完成

A3

7

最终

1,2,18,3,19,5,12,4,8,6,7,9,10,11,15,13,14,16,17,20

> I1, I2, I18, I3, I19, I5, I12, [I4, I8], I6, I9, I20  -> 11 cycles
