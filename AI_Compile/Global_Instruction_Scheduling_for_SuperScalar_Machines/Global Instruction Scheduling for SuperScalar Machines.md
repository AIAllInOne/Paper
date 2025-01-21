[https://pages.cs.wisc.edu/~fischer/cs701.f06/berstein_rodeh.pdf](https://pages.cs.wisc.edu/~fischer/cs701.f06/berstein_rodeh.pdf)

<h2 id="hS6U8">Abstraction</h2>
To improve the utilization of machine resources in <font style="color:#DF2A3F;">superscalar</font> processors, the instructions have to be carefully scheduled by the compiler. 

为了提升超标量处理器的机器资源利用率，编译器必须要仔细的调度指令集。



As internal parallelism and pipelining increases,<font style="color:#DF2A3F;"> it becomes evident that scheduling should be done beyond the basic block level. </font>

随着内部并行性和流水线数量的提升，很显然我们需要更加细粒度的调度，而不仅仅是在基本块的规模上进行调度。



A scheme for global (intra-loop) scheduling is proposed, which uses the control and data dependence information summarized in a Program Dependence Graph, to <font style="color:#DF2A3F;">move instructions well beyond basic block boundaries</font>. 

本文提出了一种全局(intra-loop)调度方案，该方案使用程序依赖图中提供的控制和数据依赖信息，<font style="color:#DF2A3F;">将指令移动到基本块边界之外 ？</font>



This novel scheduling framework is based on the parametric description of the machine architecture, which spans a range of <font style="color:#DF2A3F;">superscakis and VLIW machines</font>, and <font style="color:#DF2A3F;">exploits speculative execution</font> of instructions to further enhance the performance of the general code. 

这种新颖的调度框架基于机器架构的参数描述，涵盖了一系列<font style="color:#DF2A3F;">超级CPU (superscakis) 和VLIW机器 ?</font>，并利用指令的<font style="color:#DF2A3F;">推测执行</font>来进一步提高通用代码的性能。



We have implemented our algorithms in the IBM XL family of compilers and have evaluated them on the IBM RISC System/6000 machines.

我们已经在 IBM XL 系列编译器中实现了我们的算法，并在 IBM RISC System/6000 机器上对其进行了评估。

<h2 id="CyQP8">Introduction</h2>
Starting in the late seventies, a new approach for building high speed processors emerged which emphasizes streamlining of program instructions; subsequently this direction in computer architecture was called RISC [P85] 

从 70 年代末开始，出现了一种构建高速处理器的新方法，强调简化程序指令；随后计算机架构的这个方向被称为 RISC [P85]



It turned out that in order to take advantage of pipelining so as to improve performance, instructions have to be rearranged, usually at the intermediate language or assembly code level. 

事实证明，为了利用流水线来提高性能，必须重新排列指令，通常是在中间语言或汇编代码级别。



The burden of such transformations, called <font style="color:#DF2A3F;">instruction scheduling</font>, has been placed on optimizing compilers.

这种转换的负担被称为指令调度，已被放在优化编译器上。



Previously, scheduling algorithms at the instruction level were suggested for processors with several functional units [BJR89], pipelined machines [BG89, BRG89, HG83, GM86, W90] and Very Large Instruction Word (VLIW) machines [E85] While for machines with n functional units the idea is to be able to execute as many as n instructions each cycle, for pipelined machines the goal is to issue a new instruction every cycle, effectively eliminating the so-called NOPS (No Operations).

此前，指令级的调度算法被建议用于具有多个功能单元的处理器 [BJR89]、流水线机器 [BG89、BRG89、HG83、GM86、W90] 和超大指令字 (VLIW) 机器 [E85]。<font style="color:#DF2A3F;">对于具有 n 个功能单元的机器</font>，其想法是能够每个周期执行多达 n 条指令，而对于<font style="color:#DF2A3F;">流水线机器</font>，目标是每个周期发出一条新指令，从而有效地消除所谓的 NOPS（无操作）。



However, for both types of machines, the common feature required from the compiler is to discover in the code instructions that are data independent, allowing the generation of code that better utilizes the machine resources.

然而，对于这两种类型的机器，编译器所需的共同特性是在代码中发现与数据无关的指令，从而生成更好地利用机器资源的代码。



It was a common view that such data independent instructions can be found within basic blocks, and <font style="color:#DF2A3F;">there is no need to move instructions beyond basic block boundaries.</font> 

人们普遍认为，这种与数据无关的指令可以在基本块内找到，没有必要将指令移到基本块边界之外。



Virtually, all of the previous work on the implementation of instruction scheduling for pipelined machines concentrated on scheduling within basic blocks [HG83, GM86, W90]. 

实际上，之前关于流水线机器指令调度实现的所有工作都集中在<font style="color:#DF2A3F;">基本块内的调度上[HG83、GM86、W90]</font>。



Even for basic RISC architectures such restricted type of scheduling may result in code with many NOPS for certain Unixl -type programs that include many small basic blocks terminated by unpredictable branches. 

即使对于基本的 RISC 架构，这种受限类型的调度也可能会导致某些 Unixl 类型程序的代码具有许多 NOPS，其中包含许多由不可预测的分支终止的小基本块。



On the other hand, for scientific programs the problem is not so severe, since there, basic blocks tend to be larger.

另一方面，对于科学程序来说，问题并不那么严重，因为基本块往往更大。



Recently, a new type of architecture is evolving that extends RISC by the ability to issue more than one instruction per cycle [G089]. 

最近，一种新型架构正在发展，它扩展了 RISC，使其能够每周期发出多条指令 [G089]。



This type of high speed processors, called <font style="color:#DF2A3F;">superscalar or superpipelined architecture</font>, poses more serious challenges to compilers, since instruction scheduling at the basic block level is in many cases not sufficient to allow generation of code that utilizes machine resources to a desired extent [JW89].

这种称为超标量或超流水线架构的高速处理器对编译器提出了更严峻的挑战，因为在许多情况下，基本块级别的指令调度不足以生成利用机器资源到所需程度的代码[JW89]。



One recent effort to pursue instruction scheduling for superscalar machines was reported in [GR90], where code ,replication techniques for scheduling beyond the scope of basic blocks were investigated, resulting in fair improvements of running time of the compiled code, Also, one can view a superscalar processor as a VLIW machine with a small number of resources. 

[GR90] 报道了最近对超标量机器指令调度的一项努力，其中研究了超出基本块范围的调度的代码复制技术，从而公平地提高了编译代码的运行时间，此外，人们可以将超标量处理器视为具有少量资源的 VLIW 机器。



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



For speculative instructions, previously-it was suggested that they have to be supported by the machine architecture [ESS, SLH90]. 

对于推测指令，以前有人建议它们必须得到机器架构的支持[ESS，SLH90]。



Since architectural support for <font style="color:#DF2A3F;">speculative execution</font> carries a significant run-time overhead, we are evaluating techniques for replacing such support with compile-time analysis of the code, still retaining most of the performance effect promised by speculative execution. 

<font style="color:#DF2A3F;">由于对推测执行的架构支持会带来很大的运行时开销，我们正在评估用代码的编译时分析来替代这种支持的技术，同时仍然保留推测执行所承诺的大部分性能效果</font>。



We have implemented our scheme in the context of the IBM XL family of compilers for the IBM RISC System/6000 (RS/6K for short) computers. 

我们已经在 IBM RISC System/6000（简称 RS/6K）计算机的 IBM XL 系列编译器环境中实现了我们的方案。



The preliminary performance results for our scheduling prototype were based on a set of SPEC benchmarks [ss9]. 

我们的调度原型的初步性能结果基于一组 SPEC 基准 [ss9]。



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



<h2 id="gVwhj">Parametric machine description</h2>
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

此外，指令的执行还受到流水线约束，这些约束由分配给计算图的数据依赖边的整数延迟建模。

