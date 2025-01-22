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

## 3. A program example


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
min=a[O]; max=min; i=1;
/****************** LOOP STARTS *************/
while (i <n) {
    u=a[i]; v=a[i+1];
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

## 4. The Program Dependence Graph

The program dependence graph is a convenient way to summarize both the control dependence and data dependence among the code instructions, While the concept of data dependence, that carries the basic idea of one instruction computing a data value and another instruction using this value, was employed in compilers a long time ago, the notion of control dependence was introduced quite recently [FOW87]. 

程序依赖图是总结代码指令之间的控制依赖和数据依赖的一种便捷方式。数据依赖的概念很早以前就在编译器中使用，其基本思想是一条指令计算一个数据值，而另一条指令使用该值。而控制依赖的概念则是最近才引入的[FOW87]。

In what follows we discuss the notions of control and data dependence separately.

接下来我们分别讨论控制和数据依赖的概念。

### 4.1 Control dependences

We describe the idea of control dependence using the program example of Figure 1. 

我们使用图 1 的程序示例来描述控制依赖的想法。

In Figure 3 the control flow graph of the loop of Figure 2 is described, where each node corresponds to a single basic block in the loop. 

图 3 描述了图 2 的循环的控制流图，其中每个节点对应循环中的一个基本块。

![image](https://github.com/user-attachments/assets/4bf97ba7-8cad-4467-86c8-e5ce24052760)

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

For the strongly connected regions (that represent loops in this context), the assumption of a control flow graph having a single entry corresponds to the assumption that the control flow graph is reducible.

对于强连通区域（在此上下文中表示循环），控制流图具有单个 Entry 的假设对应于控制流图可约化的假设。
> "is reducible"（是可减的）指的是控制流图具有"可约"性质。可约控制流图是指那些可以通过消除循环来简化的图，通常包含一个单一的入口点。

The meaning of an edge from a node A to a node B in a control flow graph is that the control of the program may flow from the basic block A to the basic block B. 

控制流图中从节点A到节点B的边的含义是程序的控制可以从基本块A流向基本块B。

(UsuaUy, edges are annotated with the conditions that control the flow of the program from one basic block to another.) From the graph of Figure 3 however, it is not apparent which basic block will be executed under which condition. 

（通常，边用控制程序从一个基本块到另一个基本块的流程的条件来注释。）然而，从图 3 的图中，无法清楚地看出哪个基本块将在哪种条件下执行。

The control subgraph of the PDG (CSPDG) of the loop of Figure 2 is shown in Figure 4. As in Figure 3, each node of the graph corresponds to a basic block of the program. Here, a solid edge from a node A to a node B has the following meaning:

图 4 显示了图 2 循环的 PDG（CSPDG）的控制子图。与图 3 一样，图中的每个节点对应于程序的一个基本块。这里，从节点 A 到节点 B 的实线边具有以下含义：

![image](https://github.com/user-attachments/assets/2bad3765-33a4-4086-a6e9-83faaff6e627)

1. there is a condition COND in the end of A that is evaluated to either TRUE or FALSE, and
A 末尾有一个条件 COND，其结果要么为 TRUE，要么为 FALSE，并且
2. if COND is evaluated to TRUE, B will definitely be executed, othenvise B will not be executed.
如果 COND 的结果为 TRUE，则 B 一定会被执行，否则 B 不会被执行。

The control dependence edges are annotated with the corresponding conditions as for the control flow graph. In Figure 4 solid edges designate control dependence edges, while dashed edges will be discussed below. 

控制依赖边标注了控制流图中的相应条件。图 4 中的实线边表示控制依赖边，虚线边将在下文讨论。

For example, in Figure 4 the edges emanating from BL1 indicate that BL2 and BL4 will be executed if the condition at the end of BL1 will be evaluated to TRUE, while BL6 and BL8 will be executed while the same condition is FALSE.

例如，在图 4 中，从 BL1 发出的边表示如果 BL1 末尾的条件被评估为 TRUE，则将执行 BL2 和 BL4，而当相同条件为 FALSE 时，将执行 BL6 和 BL8。

As was mentioned in the introduction, currently we schedule instructions within a single iteration of a loop. 
正如介绍中提到的，目前我们在循环的单次迭代中调度指令。

So, for the purposes of this type of instruction scheduling, we follow [CHH89] and build the forwa-d control dependence graph only, i.e. we do not compute the control dependence that result from or propagate through the back edges in the control flow graph. 

因此，出于这种类型的指令调度的目的，我们遵循 [CHH89] 并仅构建前向控制依赖图，即，我们不计算由控制流图中的 back edges 引起或传播的控制依赖性。

The CSPDG of Figure 4 is a forward control dependence graph. In the following we discuss forward control dependence graphs only. Notice that forward control dependence graphs are acyclic

图 4 中的 CSPDG 是一个前向控制依赖图。下面我们只讨论前向控制依赖图。请注意，前向控制依赖图是非循环的

The usefulness of the control subgraph of PDG stems from the fact that basic blocks that have the same set of control dependence (like BL 1 and BL1O, or BL2 and BL4, or BL6 and BL8 in Figure 4) can be executed in parallel up to the existing data dependence. 

PDG 控制子图的实用性源于这样一个事实：具有相同控制依赖关系的基本块（如图 4 中的 BL 1 和 BL1O、BL2 和 BL4 或 BL6 和 BL8）可以并行执行，直至达到现有的数据依赖性。

For our purposes, the instructions of such basic blocks can be scheduled together.

就我们的目的而言，这些基本块的指令可以一起调度。

Now let us introduce several deffitions that are required to understand our scheduling framework. 

现在让我们介绍理解我们的调度框架所需的几个定义。

Let A and B be two nodes of a control flow graph such that B is reachable from A, i.e., there is a path in the control flow graph from A to B

假设 A 和 B 是控制流图的两个节点，并且 B 可以从 A 到达，即控制流图中存在从 A 到 B 的路径

Definition 1. A dominates B if and only if A appears on every path from ENTRY to B

Definition 1. 当且仅当 A 出现在从 ENTRY 到 B 的每条路径上时，A 才支配(deminates) B

Definition 2. B postdominates A if and only if B appears on every path from A to EXIT.

Definition 2. 当且仅当 B 出现在从 A 到 EXIT 的每条路径上时，B 才后支配(postdominates) A。

Definition 3. A and B are equivalent if and only if A dominates B and B postdorninates A.

Definition 3. 当且仅当 A 支配 B 且 B 后支配 A 时，A 和 B 等价。

Definition 4. We say that moving an instruction from B to A is useful if and only if.A and B are equivalent.

Definition 4. 当且仅当 A 和 B 等价时，我们说将指令从 B 移动到 A 是有用的。

Definition 5. We say that moving an instruction from B to A is speculative if B does not postdorninate A.

Definition 5. 如果 B 不后支配 A，我们说将指令从 B 移动到 A 是推测性的。

Definition 6. We say that moving an instruction from B to A requires duplication if A does not dominate B.

Definition 6. 如果 A 不支配 B，我们说将指令从 B 移动到 A 需要重复。

Definition 7: We say that moving instructions from B to A is n-branch speculactive if there exists a path in CSPDG from A to B of length n.

Definition 7：如果在 CSPDG 中存在一条长度为 n 的从 A 到 B 的路径，则我们称将指令从 B 移动到 A 是 n 分支推测性的。

It turns out that CSPDGs are helpful while doing useful scheduling. To find equivalent nodes, we search a CSPDG for nodes that are identically
control dependent, i.e. they depend of “the same set of nodes under the same conditions. 

事实证明，CSPDGS 在进行有用的调度时很有用。为了找到等效节点，我们在 CSPDG 中搜索相同控制依赖的节点，即它们在相同条件下依赖于“同一组节点”。

For example, in Figure 4, BL 1 and BL 10 are equivalent, since they do not depend on any node. Also, BL2 and BL4 are equivalent, since both of them depend on
BL1 under the TRUE condition. 

例如，在图 4 中，BL 1 和 BL 10 是等效的，因为它们不依赖于任何节点。此外，BL2 和 BL4 是等效的，因为它们都在 TRUE 条件下依赖于 BL1。

In Figure 4 we mark the equivalent nodes with dashed edges, the direction of these edges provides the dominance relation between the nodes. 

在图 4 中，我们用虚线边标记等效节点，这些边的方向提供了节点之间的支配关系。

For example, for equivalent nodes BL1 and BL10, we conclude that BL1 dominates BL1O.

例如，对于等效节点 BL1 和 BL10，我们得出结论，BL1 支配 BL1O。

CSPDG is useful also for speculative scheduling. It provides “the degree of speculativeness” for moving instructions from one block to another. 

CSPDG 也适用于推测调度。它为将指令从一个块移动到另一个块提供了“推测程度”。

When scheduling a speculative instruction, we always “gamble” on the outcome of one or more branches;

在调度推测指令时，我们总是“赌”一个或多个分支的结果；

only when we guess the direction of these branches correctly, the moved instruction becomes profitable.

只有当我们正确猜测这些分支的方向时，移动的指令才会有利可图。

CSPDG provides for every pair of nodes the number of branches we gamble on (in case of speculative scheduling). 

CSPDG 为每对节点提供了我们赌的分支数（在推测调度的情况下）。

For example, when moving instructions from BL8 to BL1, we gamble on the outcome of a single branch, since when moving from BL8 to BL1 in Figure 4, we cross a single edge. 

例如，当将指令从 BL8 移动到 BL1 时，我们会赌单个分支的结果，因为在图 4 中从 BL8 移动到 BL1 时，我们会跨越一条边。

(This is not obvious from the control flow graph of Figure 3.) Similarly, moving from BL5 to BL 1 gambles on the outcome of two branches, since we cross two edges of Figure 4.

（从图 3 的控制流图中看不出这一点。）同样，从 BL5 移动到 BL1 会赌两个分支的结果，因为我们跨越了图 4 的两个边。


Notice that useful scheduling is O-branch speculative.
请注意，有用的调度是 O 分支推测性的。

### 4.2 Data dependences

While control dependence are computed at a basic block level, data dependencies are computed on an instruction by instruction basis. 

虽然控制依赖性是在基本块级别计算的，但数据依赖性是逐条指令计算的。

We compute both intrablock and interlock data dependencies. A data dependence may be caused by the usage of registers or by accessing memory locations.

我们计算块内和互锁数据依赖性。数据依赖性可能是由寄存器的使用或访问内存位置引起的。

Let a and b be two instructions in the code. A data dependence edge from a to b is inserted into PDG in one of the following cases:

假设 a 和 b 是代码中的两条指令。在下列情况之一中，将从 a 到 b 的数据依赖边插入到 PDG 中：

- A register defined in a is used in b (flow dependence);
- a 中定义的寄存器在 b 中使用（流依赖性）；
- A register used in a is defined in b (anti-dependence);
- a 中使用的寄存器在 b 中定义（反依赖性）；
- A register defined in a is defined in b (output dependence);
- a 中定义的寄存器在 b 中定义（输出依赖性）；
- Both a and b we instructions that touch memory (loads, stores, calls to subroutines) and it is not proven that they address different locations (memory disambiguation)
- a 和 b 都是触及内存的指令（加载、存储、调用子例程），并且无法证明它们寻址不同的位置（内存消歧义）

Only the data dependence edges leading from a definition of a register to its use carry a (potentially non-zero) delay, which is a characteristic of the underlying machine, as was mentioned in Section 2.

只有从寄存器定义到其使用的数据依赖边缘才会产生（可能非零的）延迟，这是底层机器的特性，如第 2 节所述。

The rest of the data dependence edges carry zero delays. To minimize the number of anti and output data dependence, which may unnecessarily constrain the scheduling process, the XL compiler does certain renaming of registers, which is similar to the effect of the static single assignment form [CFRWZ].

其余数据依赖边带有零延迟。为了尽量减少反数据和输出数据依赖的数量（这可能会不必要地限制调度过程），XL 编译器对寄存器进行了某些重命名，这类似于静态单赋值形式 [CFRWZ] 的效果。

To compute all the data dependence in a basic block, essentially every pair of instructions there has to be considered. 

为了计算基本块中的所有数据依赖性，基本上必须考虑其中的每一对指令。

However, to reduce the compilation time, we take advantage of the following observation. Let a, b and c be three instructions in the code. 

然而，为了减少编译时间，我们利用以下观察。让 a、b 和 c 成为代码中的三条指令。

Then, if we discover that there is a data dependence edge from a to b and from b to c, there is no need to compute the edge from a to c. 

然后，如果我们发现从 a 到 b 和从 b 到 c 存在数据依赖边，则无需计算从 a 到 c 的边。

To use this observation, the basic block instructions are traversed in an order such that when we come to determine the dependency between a and c, we have already considered the pairs (a,b) and (b,c), for every possible b in a basic block. (Actually, we compute the transitive closure for the data dependence relation in a basic block.)

为了利用这一观察结果，基本块指令按以下顺序遍历：当我们确定 a 和 c 之间的依赖关系时，我们已经考虑了基本块中每个可能的 b 的对 (a,b) 和 (b,c)。（实际上，我们计算基本块中数据依赖关系的传递闭包。）

Next for each pair A and B of basic blocks such that B is reachable from A in the control flow graph, the intrablock data dependence are computed. 

接下来，对于控制流图中可以从 A 到达的每对基本块 A 和 B，计算块内数据依赖性。

The observation in the previous paragraph helps to reduce the number of pairs of instructions that are considered during the computation of the intrablock data dependence as well.

上一段中的观察结果也有助于减少在计算块内数据依赖性时考虑的指令对数量。

Let us demonstrate the computation of data dependence for BL1; we will reference the instructions by their numbers from Figure 2. There is an anti-dependence from (I1) to (I2), since (I1) uses r31 and (I2) defines a new value for r31. 

让我们演示一下 BL1 数据依赖性的计算；我们将通过图 2 中的编号引用指令。从 (I1) 到 (I2) 存在反依赖性，因为 (I1) 使用 r31 并且 (I2) 为 r31 定义了一个新值。

There is a flow data dependence from both (I1) and (I2) to (I3), since (I3) uses r12 and rO defined in (I1) and (I2), respectively.

从 (I1) 和 (I2) 到 (I3) 都存在流数据依赖性，因为 (I3) 分别使用了 (I1) 和 (I2) 中定义的 r12 和 rO。

The edge ((I2),(I3)) carries a one cycle delay, since (I2) is a load instruction (delayed load), while ((I1),(I3)) is not computed since it is transitive. 

边 ((I2),(I3)) 带有一个周期的延迟，因为 (I2) 是加载指令（延迟加载），而 ((I1),(I3)) 则无需计算，因为它是传递的。

There is a flow data dependence edge from (I3) to (I4), since (I3) sets cr7 which is used in (I4). 

从 (I3) 到 (I4) 存在流数据依赖性边，因为 (I3) 设置了 (I4) 中使用的 cr7。

This edge has a three cycle delay, since (I3) is a compare instruction and (I4) is the corresponding branch instruction. 

该边有三个周期的延迟，因为 (I3) 是比较指令，而 (I4) 是相应的分支指令。

Finally, both of ((I1),(I4)) and ((I2),(I4)) are transitive edges.

最后，((I1),(I4)) 和 ((I2),(I4)) 都是传递边。

It is important to notice that, since both the control and data dependence we compute are acyclic, the resultant PDG is acyclic as well. This facilitates convenient scheduling of instructions which is discussed next.

需要注意的是，由于我们计算的控制和数据依赖性都是非循环的，因此得到的 PDG 也是非循环的。这有助于方便地安排接下来要讨论的指令。

## 5. The scheduling framework

The global scheduling framework consists of the top-level process, which tries to schedule instructions cycle by cycle, and of a set of heuristics which decide what instruction will be scheduled next, in case there is a choice. 

全局调度框架由顶层进程和一组启发式算法组成，顶层进程尝试逐个周期地调度指令，启发式算法决定在有选择的情况下接下来将调度什么指令。

While the top-level process is suitable for a range of machines dicussed here, it is suggested that the set of heuristics and their relative ordering should be tuned for a specfic machine at hand. We present the top-level process in Section 5.1, while the heuristics are discussed in Section 5.2.

虽然顶层进程适用于此处讨论的一系列机器，但建议对启发式算法集及其相对顺序进行调整，以适应手头的特定机器。我们在第 5.1 节中介绍顶层进程，而在第 5.2 节中讨论启发式算法。

### 5.1 The top-level process

We schedule instructions in the program on a region by region basis. 

我们按区域逐个安排程序中的指令。

In our terminology a region represents either a strongly connected component that corresponds to a loop (which has at least one back edge) or a body of a subroutine without the enclosed loops (which has no back edges at all).

在我们的术语中，区域表示与循环相对应的强连通组件（至少有一个back edge）或没有封闭循环的子程序主体（根本没有back edges）。

Since currently we do not overlap the execution of different iterations of a loop, there is no difference in the process of scheduling the body of a loop and the body of a subroutine.

由于目前我们没有重叠执行循环的不同迭代，因此在安排循环主体和子程序主体的过程中没有区别。

Innermost regions are scheduled first. There are a few principles that govern our scheduling process:

首先调度最内层的区域。我们的调度过程遵循以下几个原则：

- Instructions are never moved out or into a region.
- 指令永远不会移出或移入区域。
- All the instructions are moved in the upward duection, i.e, they are moved against the direction of the control flow edges.
- 所有指令都以向上方向移动，即，它们沿控制流边缘的方向移动。
- The original order of branches in the program is preserved.
- 程序中分支的原始顺序得以保留。

Also, there are several limitations that characterize the current status of our implementation for global scheduling. This includes:
此外，我们目前实施的全局调度还存在一些局限性。其中包括：

- NO duplication of code is allowed (see Deftition 6 in Section 4.1).
- 不允许重复代码（参见第 4.1 节中的定义 6）。
- Only 1-branch speculative instructions are supported (see Deftition 7 in Section 4.1).
- 仅支持 1 分支推测指令（参见第 4.1 节中的定义 7）。
- No new basic blocks are created in the control flow graph during the scheduling process.
- 在调度过程中，控制流图中不会创建新的基本块。

These limitations will be removed in future work.

We schedule instructions in a region by processing its basic blocks one at a time. The basic blocks are visited in the topological order, i.e., if there is a path in the control flow graph from A to B, A is processed before B.

我们通过一次处理一个基本块来调度区域中的指令。基本块按拓扑顺序访问，即，如果控制流图中存在从 A 到 B 的路径，则先处理 A，再处理 B。

Let A be the basic block to be scheduled next, and let _EQUIV(A)_ be the set of blocks that are equivalent to A and are dominated by A (see Deftition 3). 

让 A 成为下一个要调度的基本块，让 _EQUIV(A)_ 成为与 A 等同且由 A 主导的块集（参见定义 3）。

We maintain a set _C(A)_ of _candidate blocks for A_, i.e., a set of basic blocks which can contribute instructions to A. 

我们为 A_ 维护一个 _候选块集 _C(A)_，即可以为 A 贡献指令的基本块集。

Currently there are two levels of scheduling:
目前有两种调度级别：

1. Useful instructions only: _C(A)= EQUIV(A)_;
1. 仅有用的指令：_C(A)= EQUIV(A)_;
2. 1-branch speculative: _C(A)_ includes the following blocks:
2. 1-分支推测：_C(A)_ 包含以下块：
   a. the blocks of EQUIV(A);
   a. EQUIV(A) 的块；
   b. All the immediate successors of A in CSPDG;
   b. CSPDG 中 A 的所有直接后继；
   c. All the immediate successors of blocks in _EQUIV(A)_ in CSPDG.
   c. CSPDG 中 _EQUIV(A)_ 中块的所有直接后继。

Once we initialize the set of candidate blocks, we compute the set of candidate instructions for A. 

一旦我们初始化候选块集，我们就会计算 A 的候选指令集。

An instruction I is a candidate for scheduling in block A if it belongs to one of the following categories:

如果指令 I 属于以下类别之一，则它是块 A 中调度的候选指令：

- I belonged to A in the first place.
- I 首先属于 A。
- I belongs to one of the blocks in _C(A)_ and:
- I 属于 _C(A)_ 中的一个块，并且：
    - 1. I belongs to one of the blocks in _EQUIV(A)_ and it may be moved beyond its basic block boundaries. (There are instructions that are never moved beyond basic block boundaries, like calls to subroutines.)
    - 1. I 属于 _EQUIV(A)_ 中的一个块，并且它可能被移出其基本块边界。（有些指令永远不会移出基本块边界，例如对子例程的调用。）
    - 2. I does not belong to one of the blocks in _EQUIV(A)_ and it is allowed to schedule it speculatively. (There are instructions that are never scheduled speculatively, like store to memory instructions.)
    - 2. I 不属于 _EQUIV(A)_ 中的一个块，并且允许对其进行推测性调度。（有些指令永远不会被推测性调度，例如存储到内存的指令。） 

During the scheduling process we maintain a list of ready instructions, i.e., candidate instructions whose data dependence are fulfilled. 

在调度过程中，我们维护一个就绪指令列表，即满足数据依赖性的候选指令。

Every cycle we pick from the ready list as many instructions to be scheduled next as required by the machine architecture, by consulting the parametric machine
description. 

每个周期，我们都会通过查阅参数机器描述，从就绪列表中挑选出尽可能多的指令，以满足机器架构的下一步调度要求。

If there are too many ready instructions, we choose the %est” ones based on priority criteria. 

如果就绪指令过多，我们会根据优先级标准选择“最佳”指令。

Once an instruction is picked up to be scheduled, it is moved to the proper place in the code, and its data dependence to the following instructions are marked as fulfilled, potentiality enabling new instructions to become ready. 

一旦挑选出一条指令进行调度，它就会被移动到代码中的适当位置，并且其对后续指令的数据依赖性被标记为已完成，从而有可能使新指令就绪。

Once all the instructions of A are scheduled, we move to the next basic block. 

一旦 A 的所有指令都调度完毕，我们就会转到下一个基本块。

The net result is that the instructions in A are reordered and there might be instructions eexternal to A that are physically moved into A.

最终结果是 A 中的指令被重新排序，并且可能存在 A 之外的指令，这些指令会物理移动到 A 中。

It turns out that the global scheduler does not always create the best schedule for each individual basic block. 

事实证明，全局调度程序并不总是为每个单独的基本块创建最佳调度。

It is mainly due to the two following reasons:

这主要是由于以下两个原因：

- The parametric machine description of Section 2 does not cover all the secondary features of
the machine;
- 第 2 节的参数机器描述没有涵盖机器的所有次要特征；
- The global decisions are not necessarily optimal in a local context.
- 全局决策在本地环境中不一定是最佳的。

To solve this problem, the basic block scheduler is applied to every single basic block of a program after the global scheduling is completed. 

为了解决这个问题，在全局调度完成后，将基本块调度程序应用于程序的每个单个基本块。

The basic block scheduler has a more detailed model of the machine which allows more precise decisions for reordering the instructions within the basic blocks

基本块调度程序具有更详细的机器模型，可以更精确地决定对基本块内的指令进行重新排序

### 5.2 Scheduling heuristics

The heart of the scheduling scheme is a set of heuristics that provide the relative priority of an instruction to be scheduled next. 

调度方案的核心是一组启发式方法，它们提供接下来要调度的指令的相对优先级。

There are two integer-valued functions that are computed locally (within a basic block) for every instruction in the code, these functions are used to set the priority of instructions in the program.

有两个整数值函数，它们是针对代码中的每条指令在本地（基本块内）计算的，这些函数用于设置程序中指令的优先级。

Let I be an instruction in a block B. The first function D(I), called _delay heuristic_, provides a measure of how many delay slots may occur on a
path from 1 to the end of B. 

假设 I 是块 B 中的一条指令。第一个函数 D(I) 称为 _延迟启发式_ ，用于衡量从 1 到 B 末尾的路径上可能出现多少个延迟槽。

Initially, D(I) is set to O for every I in B. 

最初，对于 B 中的每个 I，D(I) 都设置为 O。

Assume that J1,J2, ... are the immediate data dependence successors of I in B, and let the delays on those edges be d(IJ1), d(I,J2), .... 

假设 J1、J2、... 是 B 中 I 的直接数据依赖后继，并让这些边上的延迟为 d(IJ1)、d(I、J2)、...。

Then, by visiting I after visiting its data dependence successors, D(I) is computed as follows:

然后，通过在访问其数据依赖后继之后访问 I，D(I) 计算如下：

D(l) = max((D(J1) + d(I,J1)),(D(J2 + d(I,J2), ... )

The second function CP(I), called _critical path heuristic_, provides a measure of how long it will take to complete the execution of instructions that
depend on I in B, including I itself, and assuming an unbounded number of computational units. 

第二个函数 CP(I) 称为 _关键路径启发式_ ，它衡量了 B 中依赖于 I 的指令（包括 I 本身）需要多长时间才能完成执行，并假设计算单元数量无限。

Let E(I) be the execution time of I.

E(I) 为 I 的执行时间。

First, CP(I) is initialized to E(I) for every I in B. 

首先，对于 B 中的每个 I，将 CP(I) 初始化为 E(I)。

Then, again by visiting I after visiting its data dependence successors, CP(I) is computed as follows:

然后，在访问其数据依赖后继之后再次访问 I，CP(I) 计算如下：

CP(I) = max((CP(J1) + d(I,J1)), (CP(J2) + d(I,J2)), ...) + E(I)

During the decision process, we schedule useful instructions before speculative ones. For the same class of instructions (useful or speculative) we pick
an instruction with has the biggest delay heuristic (D). 

在决策过程中，我们先安排有用指令，然后再安排推测指令。对于同一类指令（有用或推测），我们选择具有最大延迟启发式（D）的指令。

For the instructions of the same class and delay we pick one that has a biggest critical path heuristic (CP). 

对于相同类型和延迟的指令，我们选择具有最大关键路径启发式（CP）的指令。

Finally, we try to preserve the original order of instructions.

最后，我们尝试保留指令的原始顺序。

To make it formally, let A be a block that is currently scheduled, and let I and J be two instructions that (should be executed by a functional unit of the same type and) are ready at the same time in the scheduling process, and one of them has to be scheduled next. 

为了正式化，A 为当前调度的块，让 I 和 J 成为在调度过程中同时准备就绪的两个指令（应由相同类型的功能单元执行），其中一个必须接下来调度。

Also, let U(A) = A ∪ EQUIV(A), and let B(I) and B(J) be the basic blocks to which I and J belong. 

此外，让 U(A) = A ∪ EQUIV(A)，让 B(I) 和 B(J) 成为 I 和 J 所属的基本块。

The the decision is made in the following order:

决策按以下顺序进行：

1. If B(l) ∈ U(A) and B(J) ∉ U(A), then pick ~
2. If B(J) ∈ U(A) and B(I) ∉ U(A), then pick J
3. If D(I)> D(J), then pick I
4, If D(J)> D(I), then pick J
5. If CP(I) > CP(J), then pick I
6. If CP(J) > CP(I), then pick J
7. Pick an instruction that occurred in the code first

Notice that the current ordering of the heuristic functions is tuned towards a machine with a small number of resources. 

请注意，启发式函数的当前排序针对资源较少的机器进行了调整。

This is the reason for always preferring to schedule a useful instmction before a speculative one, even though a speculative instruction may cause longer delay. 

这就是始终优先在推测性指令之前安排有用指令的原因，即使推测性指令可能会导致更长的延迟。

In any case, experimentation and tuning are needed for better results.

无论如何，需要进行实验和调整才能获得更好的结果。

### 5.3 Speculative scheduling

In the global scheduling framework, while doing non-speculative scheduling, to preserve the correctness of the program it is sufficient to respect the data dependence as they were defined in Section 4.2. 

在全局调度框架中，在进行非推测性调度时，为了保持程序的正确性，只需遵守第 4.2 节中定义的数据依赖性即可。

It turns out that for speculative scheduling this is not true, and a new type of information has to be maintained. 

事实证明，对于推测性调度而言，情况并非如此，必须维护一种新类型的信息。

Examine the following excerpt of a C program:

检查以下 C 程序摘录：

```
if (cond) x = 5;
else x = 3;
printf("x=%d", x)
```

The control flow graph of this piece of code looks as follows:

![image](https://github.com/user-attachments/assets/20e0d37d-289e-4d95-a28e-34f4aa7d16c6)

Instruction x=5 belongs to B2, while x=3 belongs to B3. 

指令 x=5 属于 B2，而 x=3 属于 B3。

Each of them can be (speculatively) moved into B 1, but it is apparent that both of them are not allowed to move there, since a wrong value may be printed in B4. Data dependence do not prevent the movement of these instructions into B1.

它们中的每一个都可以（推测性地）移动到 B 1，但显然它们都不允许移动到那里，因为 B4 中可能会打印错误的值。数据依赖性不会阻止这些指令移动到 B1。

To solve this problem, we maintain the information about the (symbolic) registers that are _live on exit_ from a basic block. 

为了解决这个问题，我们维护有关从基本块退出时处于活动(live on exit)状态的（符号）寄存器的信息。

If an instruction that is being considered to be moved speculatively to a block B computes a new value for a register that is live on exit from B, such speculative movement is disallowed.

如果正在考虑推测性移动到块 B 的指令为从 B 退出时处于活动状态的寄存器计算新值，则不允许进行这种推测性移动。

Notice that this type of information has to be updated dynamically, i.e., after each speculative motion this information has to be updated. Thus, let us say, x=5 is first moved to B1.

请注意，这种类型的信息必须动态更新，即，每次推测性移动之后都必须更新此信息。因此，假设 x=5 首先移动到 B1。

Then, x (or actually a symbolic register that correspondsto x) becomes live on exit from B1, and the movement of x=3 to B1 will be prevented.

然后，x（或实际上与 x 对应的符号寄存器）在退出 B1 时变为活动状态，并且将阻止 x=3 移动到 B1。

More detailed description ofthe speculative scheduling and its relationship to the PDG-based global scheduling is out of the scope of this paper.

有关推测调度及其与基于 PDG 的全局调度的关系的更详细描述超出了本文的范围。
