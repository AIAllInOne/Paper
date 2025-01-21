[https://pages.cs.wisc.edu/~fischer/cs701.f06/berstein_rodeh.pdf](https://pages.cs.wisc.edu/~fischer/cs701.f06/berstein_rodeh.pdf)

# Global Instruction Scheduling for SuperScalar Machines


<h2 id="hS6U8">Abstraction</h2>
To improve the utilization of machine resources in <font style="color:#DF2A3F;">superscalar</font> processors, the instructions have to be carefully scheduled by the compiler. 

为了提升超标量处理器的机器资源利用率，编译器必须要仔细的调度指令集。



As internal parallelism and pipelining increases,<font style="color:#DF2A3F;"> it becomes evident that scheduling should be done beyond the basic block level. </font>

随着内部并行性和流水线数量的提升，很显然我们需要更加细粒度的调度，而不仅仅是在基本块的规模上进行调度。



A scheme for global (intra-loop) scheduling is proposed, which uses the control and data dependence information summarized in a Program Dependence Graph, to <font style="color:#DF2A3F;">move instructions well beyond basic block boundaries</font>. 

本文提出了一种全局(intra-loop)调度方案，该方案使用程序依赖图中提供的控制和数据依赖信息，<font style="color:#DF2A3F;">将指令移动到基本块边界之外 ？</font>



This novel scheduling framework is based on the parametric description of the machine architecture, which spans a range of <font style="color:#DF2A3F;">superscalar and VLIW machines</font>, and <font style="color:#DF2A3F;">exploits speculative execution</font> of instructions to further enhance the performance of the general code. 

这种新颖的调度框架基于机器架构的参数描述，涵盖了一系列<font style="color:#DF2A3F;">超标量 (superscalar) 和VLIW 架构的机器 ?</font>，并利用指令的<font style="color:#DF2A3F;">推测执行</font>来进一步提高通用代码的性能。

> 超长指令字（英语：Very long instruction word，缩写：VLIW）指的是一种被设计为可以利用指令级并行（ILP）优势的CPU体系结构。一个按照顺序执行指令的非超标量处理器不能充分的利用处理器的资源，有可能导致低性能。



We have implemented our algorithms in the IBM XL family of compilers and have evaluated them on the IBM RISC System/6000 machines.

我们已经在 IBM XL 系列编译器中实现了我们的算法，并在 IBM RISC System/6000 机器上对其进行了评估。

<h2 id="CyQP8">1. Introduction</h2>
Starting in the late seventies, a new approach for building high speed processors emerged which emphasizes streamlining of program instructions; subsequently this direction in computer architecture was called RISC [P85] 

从 70 年代末开始，出现了一种构建高速处理器的新方法，强调简化程序指令；随后计算机架构的这个方向被称为 RISC [P85]



It turned out that in order to take advantage of pipelining so as to improve performance, instructions have to be rearranged, usually at the intermediate language or assembly code level. 

事实证明，为了利用流水线来提高性能，必须重新排列指令，通常是在中间语言或汇编代码级别。



The burden of such transformations, called <font style="color:#DF2A3F;">instruction scheduling</font>, has been placed on optimizing compilers.

这种转换的负担被称为指令调度，已被放在优化编译器上。



Previously, scheduling algorithms at the instruction level were suggested for processors with several functional units [BJR89], pipelined machines [BG89, BRG89, HG83, GM86, W90] and Very Large Instruction Word (VLIW) machines [E85] While for machines with n functional units the idea is to be able to execute as many as n instructions each cycle, for pipelined machines the goal is to issue a new instruction every cycle, effectively eliminating the so-called NOPS (No Operations).

此前，指令级的调度算法被建议用于具有多个功能单元的处理器 [BJR89]、流水线机器 [BG89、BRG89、HG83、GM86、W90] 和超长指令字 (VLIW) 机器 [E85]。<font style="color:#DF2A3F;">对于具有 n 个功能单元的机器</font>，其想法是能够每个周期执行多达 n 条指令，而对于<font style="color:#DF2A3F;">流水线机器</font>，目标是每个周期发出一条新指令，从而有效地消除所谓的 NOPS（无操作）。



However, for both types of machines, the common feature required from the compiler is to discover in the code instructions that are data independent, allowing the generation of code that better utilizes the machine resources.

然而，对于这两种类型的机器，编译器所需的共同特性是在代码中发现与数据无关的指令，从而生成更好地利用机器资源的代码。



It was a common view that such data independent instructions can be found within basic blocks, and <font style="color:#DF2A3F;">there is no need to move instructions beyond basic block boundaries.</font> 

人们普遍认为，这种与数据无关的指令可以在基本块内找到，没有必要将指令移到基本块边界之外。



Virtually, all of the previous work on the implementation of instruction scheduling for pipelined machines concentrated on scheduling within basic blocks [HG83, GM86, W90]. 

实际上，之前关于流水线机器指令调度实现的所有工作都集中在<font style="color:#DF2A3F;">基本块内的调度上[HG83、GM86、W90]</font>。



Even for basic RISC architectures such restricted type of scheduling may result in code with many NOPS for certain Unix-type programs that include many small basic blocks terminated by unpredictable branches. 

即使对于基本的 RISC 架构，这种受限类型的调度也可能会导致某些 Unix 类型程序的代码具有许多 NOPS，其中包含许多由不可预测的分支终止的小基本块。

> Unix 是 AT&T 贝尔实验室的商标



On the other hand, for scientific programs the problem is not so severe, since there, basic blocks tend to be larger.

另一方面，对于科学程序来说，问题并不那么严重，因为基本块往往更大。



Recently, a new type of architecture is evolving that extends RISC by the ability to issue more than one instruction per cycle [G089]. 

最近，一种新型架构正在发展，它扩展了 RISC，使其能够每周期发出多条指令 [G089]。



This type of high speed processors, called <font style="color:#DF2A3F;">superscalar or superpipelined architecture</font>, poses more serious challenges to compilers, since instruction scheduling at the basic block level is in many cases not sufficient to allow generation of code that utilizes machine resources to a desired extent [JW89].

这种称为超标量或超流水线架构的高速处理器对编译器提出了更严峻的挑战，因为在许多情况下，基本块级别的指令调度不足以生成利用机器资源到所需程度的代码[JW89]。



One recent effort to pursue instruction scheduling for superscalar machines was reported in [GR90], where `code replication techniques` for scheduling beyond the scope of basic blocks were investigated, resulting in fair improvements of running time of the compiled code, Also, one can view a superscalar processor as a VLIW machine with a small number of resources. 

[GR90] 报道了最近对超标量机器指令调度的一项工作，其中研究了 `code replication techniques` 的调度技术超出基本块范围，从而公平地提高了编译代码的运行时间，此外，人们可以将超标量处理器视为具有少量资源的 VLIW 机器。



There are two main approaches for compiling code for the VLIW machines that were reported in the literature: the trace scheduling [F81, E851] and the enhanced percolation scheduling [EN89].

文献中报道了两种编译 VLIW 机器代码的方法：跟踪调度 [F81、E851] 和增强渗透调度 [EN89]。



In this paper, we present a technique for global instruction scheduling which <font style="color:#DF2A3F;">permits the movement of instructions well beyond basic blocks boundaries within the scope of the enclosed loop</font>. 

在本文中，我们提出了一种全局指令调度技术，该技术允许指令在封闭循环范围内移动到基本块边界之外。



The method employs a novel data structure, called the Program Dependence Graph (PDG), that was recently proposed by Ferrante et. al [FOW87] to be used in compilers to <font style="color:#DF2A3F;">expose parallelism for the purposes of vectorization and generation of code for multiprocessors</font>.

该方法采用了一种称为程序依赖图 (PDG) 的新型数据结构，该结构由 Ferrante 等人 [FOW87] 最近提出，用于编译器中以公开并行性，从而实现<font style="color:#DF2A3F;">多处理器的矢量化和代码生成</font>。



We suggest combining the PDG with the parametric description of a family of superscalar machines, thereby providing a powerful framework for global instruction scheduling by optimizing compilers.

我们建议将PDG与超标量机器系列的参数描述相结合，从而通过优化编译器为全局指令调度提供强大的框架。



While trace scheduling assumes the existence of a main trace in the program (which is likely in scientific computations, but may not be true in symbolic or Unix-type programs), global scheduling (as well as enhanced percolation scheduling) does not depend on such assumption. 

虽然<font style="color:#DF2A3F;">跟踪调度假设程序中存在主跟踪（这在科学计算中是可能的，但在符号或 Unix 类型程序中可能并非如此）</font>，但全局调度（以及增强渗透调度）并不依赖于这样的假设。



However, global scheduling is capable of taking advantage of the branch probabilities, whenever available (e.g. computed by proffig). As for the enhanced percolation scheduling, our opinion is that it is more targeted towards a machine with a large number of computational units, like VLIW machines.

但是，全局调度能够利用分支概率（只要可用）（例如，由 profig 计算）。至于增强渗透调度，我们认为它更适合具有大量计算单元的机器，例如 VLIW 机器。



Using the information available in a PDG, we distinguish between useful and speculative execution of instructions. 

利用 PDG 中可用的信息，<font style="color:#DF2A3F;">我们可以区分有用指令执行和推测指令执行</font>。



Also, we identify the cases where instructions have to be duplicated in order to be scheduled. 

此外，我们还确定了必须重复指令才能进行调度的情况。



Since we are currently interested in machines with a small number of functional units (like the RISC System/6000 machines), we established a conservative approach to instruction scheduling. 

由于我们目前对具有少量功能单元的机器（如 RISC System/6000 机器）感兴趣，因此我们建立了一种保守的指令调度方法。



First we try to exploit the machine resources with <font style="color:#DF2A3F;">useful instructions</font>, next we consider <font style="color:#DF2A3F;">speculative instructions</font>, whose effect on performance depends on the probability of branches to be taken, and scheduling with duplication, which might increase the code size incurring additional costs in terms of instruction cache misses. 

首先，我们尝试使用有用的指令来开发机器资源，接下来我们考虑推测指令，其对性能的影响取决于采取分支的概率，以及重复调度，这可能会增加代码大小，并在指令缓存未命中方面产生额外的成本。



Also, we do not overlap the execution of instructions that belong to different iterations of the loop. This more aggressive type of instruction scheduling, which is often called sofware pipelining [J-X8], is left for future work.

此外，我们不会重叠执行属于循环不同迭代的指令。这种更激进的指令调度类型通常称为软件流水线 [J-X8]，留待将来研究。



For speculative instructions, previously-it was suggested that they have to be supported by the machine architecture [E88, SLH90]. 

对于推测指令，以前有人建议它们必须得到机器架构的支持[ESS，SLH90]。



Since architectural support for <font style="color:#DF2A3F;">speculative execution</font> carries a significant run-time overhead, we are evaluating techniques for replacing such support with compile-time analysis of the code, still retaining most of the performance effect promised by speculative execution. 

<font style="color:#DF2A3F;">由于对推测执行的架构支持会带来很大的运行时开销，我们正在评估用代码的编译时分析来替代这种支持的技术，同时仍然保留推测执行所承诺的大部分性能效果</font>。



We have implemented our scheme in the context of the IBM XL family of compilers for the IBM RISC System/6000 (RS/6K for short) computers. 

我们已经在 IBM RISC System/6000（简称 RS/6K）计算机的 IBM XL 系列编译器环境中实现了我们的方案。



The preliminary performance results for our scheduling prototype were based on a set of SPEC benchmarks [ss9]. 

我们的调度原型的初步性能结果基于一组 SPEC 基准 [s89]。



The rest of the paper is organized as follows. 

In Section 2 we describe our <font style="color:#DF2A3F;">generic machine model</font> and show how it is applicable to the RS/6K machines. 

本文的其余部分组织如下。

在第 2 部分中，我们描述了通用机器模型，并展示了它如何适用于 RS/6K 机器。



Then, in Section 3 we bring a small program that will serve as a running example. 

然后，在第 3 部分中，我们将提供一个小程序作为运行示例。



In Section 4 we discuss the usefulness of the PDG, while in Section 5 several levels of scheduling, including speculative execution, are presented. 

在第 4 节中，我们讨论了 PDG 的实用性，而在第 5 节中，我们介绍了包括推测执行在内的几种级别的调度。



Finally, in Section 6 we bring some performance results and conclude in Section 7.

最后，我们在第 6 节中得出一些性能结果，并在第 7 节中得出结论。



<h2 id="gVwhj">2. Parametric machine description</h2>


Our model of a superscalar machine is based on the description of a typical RISC processor whose only instructions that reference memory are load and store instructions, while all the computations are done in registers. 

我们的超标量机器模型基于<font style="color:#DF2A3F;">典型 RISC 处理器的描述</font>，其<font style="color:#DF2A3F;">引用内存的唯一指令是加载和存储指令</font>，而所有计算都在寄存器中完成。



We view a superscalar machine as a collection of functional units of m types, where the machine has n1, n2, ....nm units of each type. Each instruction in the code can be potentially executed by any of the units of a specitied type.

我们将超标量机器视为 m 种类型的功能单元的集合，其中机器每种类型有 n1、n2、...nm 个单元。代码中的每条指令都可能由指定类型的任何单元执行。



For the instruction scheduling purposes, we assume that there is an unbounded number of symbolic registers in the machine. 

为了指令调度的目的，我们假设机器中有无限数量的符号寄存器。



Subsequently, during the register allocation phase of the compiler, the symbolic registers are mapped onto the real machine registers, using one of the standard (coloring) algorithms. 

随后，在编译器的寄存器分配阶段，使用标准（着色）算法之一将符号寄存器映射到真实机器寄存器上。



Throughout this paper we will not deal with register allocation at all. 

在本文中我们根本不会处理寄存器分配。



For the discussion on the relationships between instruction scheduling and register allocation see [BEH89].

有关指令调度和寄存器分配之间的关系的讨论，参见[BEH89]。



A program instruction requires an integral number of machine cycles to be executed by one of the functional units of its type. 

一条程序指令需要由该类型的某个功能单元执行整数个机器周期。

Also, there are pipelined constraints imposed on the execution of instructions which are modelled by the integer delays assigned to the data dependence edges of the computational graph.

此外，指令的执行还受到流水线约束，这些约束由 分配给计算图的数据依赖边的整数延迟 建模。

Let L1 and L2 be two instructions such that the edge (L1,L2)is a data dependence edge. 

令 L1 和 L2 为两个指令，边 (L1,L2) 为数据依赖边。

Let t(t >= 1) be the execution time of L1 and d (d >= 0) be the delay assigned to (L1,L2). 

令 t(t >= 1) 为 L1 的执行时间，d (d >= 0) 为分配给 (L1,L2) 的延迟。

For performance purposes, if L1 is scheduled to start at time k, then L2 should be scheduled to start no earlier than k + t + d. 

出于性能目的，如果 L1 计划在时间 k 启动，则 L2 应计划不早于 k + t + d 启动。

Notice, however, that if L2 is scheduled (by the compiler) to start earlier than mentioned above, this would not affect the correctness of the program, since we assume that the machine implements hardware interlocks to guarantee the delays at run time. 

但是，请注意，如果 L2（由编译器）被安排得比上面提到的更早启动，这不会影响程序的正确性，因为我们假设机器实现硬件联锁以保证运行时的延迟。

More information about the notion of delays due to pipelined constraints can be found in [BG89, BRG89].

有关由于流水线约束导致的延迟概念的更多信息，可以在[BG89，BRG89]中找到。

<h3 id="gVwhj">2.1 The RS/6K model</h3>


Here we show how our generic model of a superscalar machine is cotilgured to fit the RS/6K machine. The RS/6K processor is modelled as follows:

这里我们展示了如何将我们的超标量机器通用模型与 RS/6K 机器进行匹配。RS/6K 处理器的模型如下：

- m = 3, there are three types of functional units: fixed point, floating point and branch types.
- m = 3，功能单元有三种类型：定点型、浮点型和分支型。

- n1= 1, n2= 1, n3= 1, there is a single fixed point unit, a single floating point unit and a single branch unit.
- n1= 1，n2= 1，n3= 1，有一个定点单元、一个浮点单元和一个分支单元。

- Most of the instructions are executed in one cycle, however, there are also multi-cycle instructions, like multiplication, division, etc
- 大多数指令在一个周期内执行，但也存在多周期指令，例如乘法、除法等

- There are four main types of delays:
- 延迟主要有四种类型：
  - a delay of one cycle between a load instruction and the instruction that uses its result register (delayed load);
  - 加载指令和使用其结果寄存器的指令之间有一个周期的延迟（deploed load）；

  - a delay of three cycles between a fixed point compare instruction and the branch instruction that uses the result of that compare;
  - 定点比较指令与使用该比较结果的分支指令之间有三个周期的延迟；
  > More precisely, usually the three cycle delay between a fixed point compare and the respective branch instruction is
encountered only when the branch is taken. However, here for simplicity we assume that such delay exists whether
the branch is taken or not.
  > 更准确地说，通常只有在执行分支时才会遇到定点比较和相应分支指令之间的三个周期延迟。然而，为了简单起见，我们假设无论是否执行分支，这种延迟都存在。

  - a delay of one cycle between a floating point instruction and the instruction that uses its result;
  - 浮点指令与使用其结果的指令之间有一个周期的延迟；

  - a delay of five cycles between a floating point compare instruction and the branch instruction that uses the result of that compare.
  - 浮点比较指令与使用该比较结果的分支指令之间存在 5 个周期的延迟。

There are a few additional delays in the machine whose effect is secondary.

机器中还有一些额外的延迟，但其影响是次要的。

In this paper we concentrate on fixed point computations only. Therefore, only the first and the second types of the above mentioned delays will be considered.

在本文中我们只关注定点计算。因此，我们只考虑上述第一种和第二种类型的延迟。

## A program example


Next, we present a small program (written in C) that computes the minimum and the maximum of an array. 

接下来，我们将提供一个小程序（用 C 编写），计算数组的最小值和最大值。

This program is shown in Figure 1 and will serve us as a running example.

该程序如图 1 所示，将作为一个运行示例。

Figure 1. A program computing the minimum and the maximum of an array
```Figure 1. A program computing the minimum and the maximum of an array
/* find the largest and the smal lest number
in a given array */
minmax(a,n) {
int i,u,v,min,max,n,a[SIZE];
min=a[O]; max=min; i=l;
/****************** LOOP STARTS *************/
while (i <n) {
    u=a[i]; v=a[i+l];
    if (u>v) {
        if (u>max) max=u;
        if (v<min) min=v;
    }
    else {
        if (v>max) max=v;
        if (u<min) min=u;
    }
    i= i+2;
} 
/************** Loop ENDS ***************/
printf(’’min=%d max=%d\n’’,min,max);
}
```

In this program, concentrating on the loop which is marked in Figure 1, we notice that two elements of the array a are fetched every iteration of the loop. 

在这个程序中，集中精力于图 1 中标记的循环，我们注意到每次循环迭代都会获取数组 a 的两个元素。


Next, these elements of a are compared one to another (if(u > v)) , and subsequently they are compared to the max and mi n variables, updating the maximum and the minimum, if needed. The RS/6K pseudo-code for the loop, that corresponds to the real code created by the IBM XL-C compiler3 , is presented in Figure 2

接下来，将 a 的这些元素相互比较（如果（u > v）），然后将它们与 max 和 min 变量进行比较，并根据需要更新最大值和最小值。图 2 显示了该循环的 RS/6K 伪代码，该代码对应于 IBM XL-C 编译器创建的实际代码

Figure 2. The RS/6K pseudo-code forthe program of Figure 1

```
max is kept in r30
min is kept in r28
i is kept in r29
n is kept in r27
address of a[i] is kept in r31
. . . more instructions here . . .
*************** LOOP STARTS *******************
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
--------------------------------------- END BL1O
*************** LOOP ENDS **********************
. . . more instructions here . . .
```

For convenience, we number the instructions in the code of Figure 2 (I1-I20) and annotate them with the corresponding statements of the program of Figure 1. 

为了方便起见，我们对图 2 的代码中的指令进行编号（I1-I20），并用图 1 的程序的相应语句进行注释。

Also, we mark the ten basic blocks (BL1-BL10) of which the code of Figure 2 comprises for the purposes of future discussion.

另外，为了便于将来讨论，我们标记了图 2 的代码所包含的十个基本块（BL1-BL10）。

For simplicity of notation, the registers mentioned in the code are real. 

为了表示简单，代码中提到的寄存器都是真实存在的/实数？

However, as was mentioned in Section 2, we prefer to invoke the global scheduling algorithm before the register allocation is done (at this stage there is an unbounded number of registers in the code), even though conceptually there is no problem to activate the instruction scheduling after the register allocation is completed.

然而，正如第 2 节所提到的，我们倾向于在寄存器分配完成之前调用全局调度算法（在此阶段代码中的寄存器数量是无限的），即使从概念上讲在寄存器分配完成后激活指令调度没有问题。

Every instruction in the code of Figure 2, except for branches, requires one cycle in the fixed point unit, while the branches take one cycle in the branch unit. 

图 2 代码中的每一条指令（分支除外）都需要在固定点单元中占用一个周期，而分支在分支单元中则占用一个周期。

There is a one cycle delay between instruction I2 and I3, due to the delayed load feature of the RS/6K. 

由于 RS/6K 的延迟加载特性，指令 I2 和 I3 之间存在一个周期的延迟。

Notice the special form of a load with update instruction in I2: in addition to assigning to r0 the value of the memory locational address (r31) + 8, it also increments r31 by 8 (post-increment). 

注意 I2 中带更新指令的加载的特殊形式：除了将内存位置地址 (r31) + 8 的值分配给 r0 之外，它还将 r31 增加 8（后增）。

Also, there is a three cycle delay between each compare instruction and the corresponding branch instruction. 

此外，每个比较指令和相应的分支指令之间有三个周期的延迟。

Taking into consideration that the fixed point unit and the branch unit run in parallel, we estimate that the code executes in 20, 21 or 22 cycles, depending on if 0, 1 or 2 updates of max and mi n variables (LR instructions) are done, respectively.

考虑到定点单元和分支单元并行运行，我们估计代码执行需要 20、21 或 22 个周期，具体取决于是否分别完成 max 和 min 变量（LR 指令）的 0、1 或 2 次更新。

## The Program Dependence Graph

The program dependence graph is a convenient way to summarize both the control dependence and data dependence among the code instructions, While the concept of data dependence, that carries the basic idea of one instruction computing a data value and another instruction using this value, was employed in compilers a long time ago, the notion of control dependence was introduced quite recently [FOW87]. 

程序依赖图是总结代码指令之间的控制依赖和数据依赖的一种便捷方式。数据依赖的概念很早以前就在编译器中使用，其基本思想是一条指令计算一个数据值，而另一条指令使用该值。而控制依赖的概念则是最近才引入的[FOW87]。

In what follows we discuss the notions of control and data dependence separately.

接下来我们分别讨论控制和数据依赖的概念。

Control dependences
We describe the idea of control dependence using the program example of Figure 1. 

我们使用图 1 的程序示例来描述控制依赖的想法。

In Figure 3 the control flow graph of the loop of Figure 2 is described, where each node corresponds to a single basic block in the loop. 

图 3 描述了图 2 的循环的控制流图，其中每个节点对应循环中的一个基本块。

The numbers inside the circles denote the indices of the ten basic blocks BL1-BL1O. \

圆圈内的数字表示十个基本块BL1-BL1O的索引。

We augment the graph of Figure 3 with unique ENTRY and EXIT nodes for convenience. 

为了方便起见，我们在图 3 的图形中添加了独特的 ENTRY 和 EXIT 节点。

Throughout this discussion we assume a single entry node in the control flow graph, i.e., there is a single node (in our case BL1) which is connected to ENTRY. 

在整个讨论中，我们假设控制流图中有一个入口节点，即有一个连接到 ENTRY 的节点（在我们的例子中是 BL1）。

However several exit nodes that have the edges leading to EXIT may exist. 

然而，可能存在多个具有通向 EXIT 的边的出口节点。

In our case BL10 is a (single) exit node. 

在我们的例子中，BL10 是一个（单个）出口节点。

For the strongIy connected regions (that represent loops in this context), the assumption of a control flow graph having a single entry corresponds to the assumption that the control flow graph is reducible.

对于强连通区域（在此上下文中表示循环），控制流图具有单个条目的假设对应于控制流图可约化的假设。



