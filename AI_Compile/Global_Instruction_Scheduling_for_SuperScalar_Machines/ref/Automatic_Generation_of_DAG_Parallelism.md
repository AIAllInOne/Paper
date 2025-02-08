https://dl.acm.org/doi/pdf/10.1145/74818.74823

During the execution of a sequential program, the results of some operations, such as branches, determine whether other statements will subsequently be executed.

在顺序程序的执行过程中，某些操作（例如分支）的结果决定是否随后执行其他语句。

The control dependence graph [FOW87] summarizes those conditions that may affect a statement’s execution. 

控制依赖图 [FOW87] 总结了可能影响语句执行的那些条件。

To the precision of flow (non-semantic) analysis, control dependences represent control flow relationships that must be respected by any execution of the program, whether parallel or sequential.

为了精确地进行流（非语义）分析，控制依赖表示程序的任何执行（无论是并行执行还是顺序执行）都必须遵守的控制流关系。

By examining the control dependence graph, we can eliminate unnecessary sequencing and expose potential parallelism. 

通过检查控制依赖图，我们可以消除不必要的排序并揭示潜在的并行性。


![image](https://github.com/user-attachments/assets/5d86b48e-71ec-4cd8-8de5-8a72b662d78f)


Consider the example shown in Figure 2. 

All of the labeled statements except S1 and S8 are control dependent on the branch taken by statement S1, even
though other branches intervene between statements S7 and S9. 

除 S1 和 S8 之外的所有带标签的语句都依赖于语句 S1 所采取的分支，即使其他分支介于语句 S7 和 S9 之间。

If the flow of control passes from S1 to S2, the execution of statements S3, S4, S5, S6, S7, and S9 is guaranteed.

如果控制流从 S1 传递到 S2，则保证执行语句 S3、S4、S5、S6、S7 和 S9。

Control dependence can be computed from the control flow graph of a program. 

可以根据程序的控制流图计算控制依赖性。

Nodes of the control flow graph could be any form of basic block: statements, maximal basic blocks, or any straight-line sequence. 

控制流图的节点可以是任何形式的基本块：语句、最大基本块或任何直线序列。

We make the following assumptions: 

+ a basic block corresponds to a statement of the program.
+ 一个基本块对应于程序的一个语句。
+ the control flow graph contains a unique entry node Entry and a unique exit node Exit.
+ 控制流图包含一个唯一的入口节点 Entry 和一个唯一的出口节点 Exit。
+ the control flow graph is augmented with an edge from the Entry to the Exit node to obtain a singly rooted control dependence graph. 
+ 控制流图通过从 Entry 到 Exit 节点的边进行扩充，以获得单根控制依赖图。
+ if a node has multiple successors, then the edges from that node have distinct labels (such as "T" and "F").
+ 如果一个节点有多个后继节点，那么来自该节点的边具有不同的标签（例如“T”和“F”）。

![image](https://github.com/user-attachments/assets/0c70e31b-7d32-43fc-844f-7ad6f7f0281b)

Definition 1 Let Gcf = (N, Ecf) be a control flow graph. We say that node Z is post-dominated by node Y in Gcf if every directed path from Z to Exit contains Y, Z ≠ Y, and Y ≠ Exit. 

定义 1 令 Gcf = (N, Ecf) 为控制流图。如果从 Z 到 Exit 的每条有向路径都包含 Y、Z ≠ Y 且 Y ≠ Exit，则称节点 Z 在 Gcf 中被节点 Y 后支配。

The post-dominator graph is a tree, and we denote the post-dominator tree for the graph Gcf by PDcf.

后支配图是一棵树，我们用 PDcf 表示图 Gcf 的后支配树。

Post-dominators can be computed by solving the dominators problem [LT79] over the reverse control flow graph. 

后支配可以通过求解反向控制流图上的支配问题 [LT79] 来计算。

![image](https://github.com/user-attachments/assets/fa5f3972-1d46-42a5-a1b1-f3aa65c476a0)


Definition 2 The control dependence graph Gcd = (N, Ecd) has the same nodes as Gcf, with edges determined by the control dependence relation:
X cd Y if
1. there exists a non-empty, directed path P in Gcf from X to Y with all nodes in P (except X and Y) post-dominated by Y and
2. X is not post-dominated by Y.

定义 2 控制依赖图 Gcd = (N, Ecd) 具有与 Gcf 相同的节点，其边由控制依赖关系确定：
X cd Y，如果
1. Gcf 中存在从 X 到 Y 的非空有向路径 P，并且 P 中的所有节点（X 和 Y 除外）均由 Y 后支配，并且
2. X 不由 Y 后支配。

In other words there is some edge from X that definitely causes Y to execute, and there is also some path from X that avoids executing Y. 

换句话说，X 中存在一些边，它肯定会导致 Y 执行，而 X 中也存在一些路径，可以避免执行 Y。

We associate with this control dependence from X to Y the label on the control flow edge from X that causes Y to execute. 

我们将从 X 到 Y 的控制依赖与从 X 到 Y 的控制流边上的标签相关联，该标签会导致 Y 执行。

Where necessary, we denote the edges of a control dependence graph as a triple: (X, Y, l), where l is the associated label. 

在必要时，我们将控制依赖图的边表示为三元组：(X, Y, l)，其中 l 是相关标签。

Two nodes X and Y are identically control dependent on a node P if both are control dependent on P and the control dependence from P to X has the same label as the control dependence from P to Y. 

如果两个节点 X 和 Y 都控制依赖于 P，并且从 P 到 X 的控制依赖与从 P 到 Y 的控制依赖具有相同的标签，则这两个节点 X 和 Y 完全控制依赖于节点 P。

In other words, (P, X, l) and (P, Y, l) are both edges in the control dependence graph.

换句话说，(P, X, l) 和 (P, Y, l) 都是控制依赖图中的边。

The control dependence relation can be computed as the solution to a flow problem over the control flow graph, as suggested by the above definition. 

控制依赖关系可以计算为控制流图上的流问题的解，如上述定义所示。

In this paper, we rely on a more efficient algorithm that requires a single pass over the post-dominator tree of the control flow graph [CFR*89] [CF87a.]. More formally, 

在本文中，我们依靠一种更高效的算法，该算法需要对控制流图的后支配树进行一次遍历 [CFR*89] [CF87a.]。更正式地说，

![image](https://github.com/user-attachments/assets/a52fc744-da32-4ed1-8a99-6d0f7b2e3473)

Gcd = CD_alg(Ecf , PDcf)

对于排序和私有化，我们使用控制依赖图的子图，该子图仅反映与语句并行化相关的控制依赖关系。

![image](https://github.com/user-attachments/assets/29be2c66-9ed3-44ec-9421-13da8d4df667)

![image](https://github.com/user-attachments/assets/5164d3b9-8431-4813-84eb-ed5dd8ec75fe)


Definition 3 The forward con,trol depen,dence subgraph is Gfcd = (N, Efed) where Efcd ⊆ Ecd is computed as
follows:
1. Compute PDcf : the post-dominators of GcJ.
2. Construct Gfcf = (N, Efcf), where Efcf = Ecf - {(X,Y) | Y dominates X in Gcf}
Thus, Efcf contains all edges of Ecf except the back edges.
3. Compute Gfcd = CD_alg(Efcf , PDcf). The edges Efcd are thus determined by the control dependence algorithm [CF87a], using the control flow
edges Efcf and the post-dominator relation PDcf. 


定义 3 前向控制依赖子图为 Gfcd = (N, Efed)，其中 Efcd ⊆ Ecd 计算如下：

1. 计算 PDcf：Gcf 的后支配者。
2. 构造 Gfcf = (N, Efcf)，其中 Efcf = Ecf - {(X,Y) | Y 在 Gcf 中支配 X}
因此，Efcf 包含 Ecf 的所有边，后边除外。
3. 计算 Gfcd = CD_alg(Efcf , PDcf)。因此，边 Efcd 由控制依赖算法 [CF87a] 确定，使用控制流边 Efcf 和后支配者关系 PDcf。


Where unambiguous, we drop the subscripts of Gfcd in favor of G = (N, E) for the forward control dependence graph. 

在明确的情况下，我们删除 Gfcd 的下标，转而使用 G = (N, E) 作为前向控制依赖图。

Theorem 1 Gfcd is a tree if Gcf is a structured control flow graph. 

定理 1 如果 Gcf 是结构化控制流图，则 Gfcd 是一棵树。

Theorem 2 Gfcd is acyclic if Gcf is reducible

定理 2 如果 Gcf 可约，则 Gfcd 是非循环的。
