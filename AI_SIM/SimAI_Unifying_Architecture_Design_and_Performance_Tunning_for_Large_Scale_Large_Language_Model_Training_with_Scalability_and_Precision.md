https://ennanzhai.github.io/pub/nsdi25spring-simai.pdf

# SimAI: Unifying Architecture Design and Performance Tunning for Large-Scale Large Language Model Training with Scalability and Precision
# SimAI：统一架构设计和性能调优，实现可扩展、精准的大规模语言模型训练

# Abstract

The large number of GPUs required for a single LLM training significantly hinders the validation of new designs, tunings, and optimizations, calling for the occurrence of efficient simulators. 

单个 LLM 训练所需的大量 GPU 严重阻碍了新设计、调整和优化，要求进行有效的模拟。

Existing simulators, however, only target a specific granularity of the entire training, intrinsically leading to imprecision. 

然而，现有的模拟器仅针对整个训练的特定粒度，本质上导致不精确。

This paper presents SimAI, a unified simulator aiming at precisely and efficiently simulating the LLM training procedure at scale. 

本文介绍了一种统一的模拟器 SimAI，旨在精确、高效地大规模模拟 LLM 训练过程。

Through selective and high-fidelity integration of the training frameworks, the kernel computation, and the collective communication library into the simulating procedure, SimAI achieves high precision in simulations. 

SimAI 通过将训练框架、内核计算和集体通信库有选择地高保真地集成到模拟过程中，实现了模拟的高精度。

SimAI further conducts multi-thread acceleration and implements lock-free global context-sharing to accelerate the execution speed. 

SimAI进一步进行多线程加速，并实现无锁的全局上下文共享，以加快执行速度。

The effectiveness of SimAI is validated by its performance results, which show an average of 98.1% alignment to real-world results under various test scenarios and affirm its robustness and adaptability from small-scale labs to large-scale industrial environments. 

SimAI 的有效性通过其性能结果得到验证，该结果显示，在各种测试场景下与真实结果的平均一致率为 98.1%，并证实了其从小型实验室到大规模工业环境的稳健性和适应性。

SimAI delivers meaningful guidelines for new host designs and parameter settings, directly benefiting in-production LLM training. We also share experiences and lessons learned during the evolution of SimAI. 

SimAI is open sourced at https://github.com/aliyun/SimAI.

# 1.Introduction

As the field of Artificial Intelligence (AI) rapidly advances, particularly with the rise of large language models like OpenAI’s GPT-4 [41], the need for scaling AI infrastructure has grown significantly. 

随着人工智能 (AI) 领域的快速发展，特别是随着 OpenAI 的 GPT-4 [41] 等大型语言模型的兴起，对扩展 AI 基础设施的需求显著增长。

For instance, training GPT-4 reportedly requires around 25,000 state-of-the-art GPUs [19]. 

This massive resource demand presents a significant barrier to entry for organizations looking to compete in this high-stakes domain.

这种巨大的资源需求对于希望在这个高风险领域竞争的组织来说构成了巨大的进入壁垒。

In this context, simulators become essential both before and after infrastructure investments.

在这种背景下，基础设施投资前后的模拟都变得至关重要。

In the planning phase, simulators help organizations estimate the scale and architecture required to achieve performance goals. 

在规划阶段，模拟可帮助组织估计实现性能目标所需的规模和架构。

In the operation phase, they help increase resource utilization, ensuring a return on investment. 

在运营阶段，它们有助于提高资源利用率，确保投资回报。

Thus, simulators are not just tools for improving efficiency but are strategic assets that maximize resource use and ensure that infrastructure investments deliver measurable results.

因此，模拟器不仅仅是提高效率的工具，而且是最大限度利用资源并确保基础设施投资产生可衡量成果的战略资产。

Currently, it is common practice to use separate simulators for capacity planning and performance tuning. 

目前，使用单独的模拟器进行容量规划和性能调整是一种常见的做法。

For capacity planning, simulation is typically performed at the flow or job level, ignoring packet-level behavior [32]. 

对于容量规划，模拟通常在流或作业级别执行，忽略数据包级别行为[32]。

In contrast, performance tuning relies on packet-level simulations to analyze network traffic patterns, latency, and packet loss—factors critical for optimizing communication and computation in AI model training and inference.

相比之下，性能调整依赖于数据包级模拟来分析网络流量模式、延迟和数据包丢失——这些因素对于优化 AI 模型训练和推理中的通信和计算至关重要。

However, our experience shows that using multiple simulators with different levels of granularity presents three main challenges. 

然而，我们的经验表明，使用具有不同粒度级别的多个模拟器会带来三个主要挑战。

First, this approach leads to inaccurate cost-performance analyses, making it difficult to predict loadbalancing performance and assess system failures. 

首先，这种方法导致成本性能分析不准确，从而难以预测负载平衡性能和评估系统故障。

Second, the poor performance of detailed simulators and the low accuracy of coarse-grained simulators limit the ability to optimize model training in large-scale deployments. 

其次，详细模拟器性能较差，粗粒度模拟器准确度较低，限制了在大规模部署中优化模型训练的能力。

Finally, this fragmented approach complicates development and testing, increasing the risk of discrepancies between simulated and real-world performance.

最后，这种分散的方法使开发和测试变得复杂，增加了模拟和实际性能之间出现差异的风险。

To overcome these challenges, we developed a unified simulator that handles both capacity planning and performance tuning within a single framework. 

为了克服这些挑战，我们开发了一个统一的模拟器，可以在单一框架内处理容量规划和性能调整。

However, this approach introduces several difficulties:

+ Generating workloads with high precision to reflect realistic AI training behaviors.
+ 生成高精度的工作负载以反映现实的 AI 训练行为。
+ Simulating computation accurately across various GPU architectures.
+ 跨各种 GPU 架构准确模拟计算。
+ Precisely modeling communication to account for network traffic patterns and latencies.
+ 精确建模通信以解释网络流量模式和延迟。
+ Scaling the simulator to support diverse, large-scale AI infrastructure configurations.
+ 扩展模拟器以支持多样化、大规模的 AI 基础设施配置。

In this paper, we share the development and operation experience of SimAI, a unified simulator designed for scalable, high-precision simulations of large-scale LLM training. 

在本文中，我们分享了 SimAI 的开发和运行经验，SimAI 是一个专为大规模 LLM 训练的可扩展、高精度模拟而设计的统一模拟器。

SimAI addresses these challenges by introducing highfidelity models across the entire AI training stack. 

SimAI 通过在整个 AI 训练堆栈中引入高保真模型来解决这些挑战。

To generate precise workloads for LLM training at any scale, SimAI “hijacks” mainstream training frameworks, such as Megatron [52] and DeepSpeed [46], to run on a single host and create fine-grained workloads (§3.2). 

为了为任何规模的 LLM 训练生成精确的工作负载，SimAI“劫持”了主流训练框架，例如 Megatron [52] 和 DeepSpeed [46]，在单个主机上运行并创建细粒度的工作负载（§3.2）。

Different solutions ensure accurate simulation of both computation and communication.

不同的解决方案确保计算和通信的精确模拟。

For computation, we break the workload into fine-grained kernels, measuring execution times on existing GPUs and mapping them to other GPU types (§3.3). 

对于计算，我们将工作负载分解为细粒度的内核，测量现有 GPU 上的执行时间并将其映射到其他 GPU 类型（§3.3）。

For communication, we “hijack” the NVIDIA Collective Communications Library (NCCL) [36] to accurately simulate packet-level behavior for collective communication (§3.4). 

对于通信，我们“劫持”了 NVIDIA 集体通信库 (NCCL) [36] 来准确模拟集体通信的数据包级行为 (§3.4)。

To improve the simulator’s efficiency, we implement multi-threaded acceleration and lock-free global context sharing among threads
(§3.5).

为了提高模拟器的效率，我们实现了多线程加速和线程间无锁全局上下文共享（第 3.5 节）。

The performance results of our unified simulator are a testament to its efficacy.

我们统一模拟器的性能结果证明了它的功效。

In terms of accuracy, it achieves an average deviation of only 1.9% compared to real-world results across various test scenarios.

就准确性而言，在各种测试场景中，与真实世界结果相比，平均偏差仅为1.9%。

On the scalability front, the simulator handles simulations from small-scale lab environments to large-scale industrial deployments, proving its robustness and adaptability (§4).

在可扩展性方面，模拟器处理从小型实验室环境到大规模工业部署的模拟，证明了其健壮性和适应性（§4）。

Since incorporating this unified simulator into our AI development pipeline, we have served various teams and gained valuable insights.

自从将这个统一模拟器纳入我们的AI开发流程以来，我们已为各个团队提供了服务并获得了宝贵的见解。

It has improved our infrastructure management and accelerated AI model development and deployment.

它改善了我们的基础设施管理并加快了AI模型的开发和部署。

We share the benefits and contributions of SimAI, including guidelines for new host designs and accurate assessments of scaling benefits (§5).

我们分享了SimAI的好处和贡献，包括新主机设计的指南和有关扩展收益的准确评估（§5）。

These guidelines have been adopted by engineering teams and incorporated into production deployments.

这些指南已被工程团队采纳，并纳入生产部署。

Additionally, we share lessons learned in transforming SimAI from a standalone simulator to a widely used simulation service (§6).

此外，我们分享了将SimAI从独立模拟器转变为广泛使用的模拟服务所学到的经验教训（§6）。

SimAI is a high-precision, full-stack simulator designed to benefit researchers across various domains involved in Large Language Model (LLM) training.

SimAI是一个高精度的全栈模拟器，旨在惠及参与大型语言模型（LLM）训练的各个领域的研究人员。

This versatile tool caters to multiple levels of the LLM training ecosystem.

这个多功能工具适用于LLM培训生态系统的多个层面。

At the framework level, SimAI enables the exploration of optimal parallel strategies and communication-computation overlap techniques, facilitating parameter tuning to reduce end-to-end training time.

在框架层面上，SimAI使得能够探索最佳的并行策略和通信计算重叠技术，从而促进参数调整以减少端到端的训练时间。

For collective communication research, it offers a platform to validate and quantify novel algorithms’ performance gains.

对于集体通信研究，它提供了一个平台来验证和量化新算法的性能增益。

SimAI’s system architecture design allows for experimentation with diverse intra-host and inter-host configurations, helping identify the most cost-effective solutions.

SimAI的系统架构设计允许对多样的主机内和主机间配置进行实验，有助于确定最具成本效益的解决方案。

By providing the flexibility to customize and fine-tune different components, SimAI empowers users to conduct multifaceted research accelerating LLM training processes, and is an invaluable tool for scholars and practitioners throughout the LLM development pipeline.

通过提供定制和微调不同组件的灵活性，SimAI能够赋予用户进行多方面研究以加速LLM训练过程的能力，并成为LLM开发流程中学者和从业者的宝贵工具。

# 2.Background and Motivation

## 2.1 AI Training Infrastructure

Large language models (LLMs) require specialized infrastructure, often involving dozens to thousands of GPUs working together to handle pretraining or fine-tuning tasks.

大型语言模型（LLMs）需要专门的基础设施，通常涉及数十到数千个GPU共同合作来处理预训练或微调任务。

For instance, training a GPT-3 model with 175 billion parameters demands 1,024 high-end GPUs running continuously for 34 days [35].

例如，对具有1750亿参数的GPT-3模型进行训练需要连续运行1024台高端GPU达34天[35]。

To optimize GPU usage, mainstream training frameworks like Megatron [52] and DeepSpeed [46] offer parallelization techniques such as Data Parallelism (DP), Pipeline Parallelism (PP), and Tensor Parallelism (TP).

为了优化GPU的使用情况，像Megatron [52]和DeepSpeed [46]这样的主流训练框架提供了数据并行（DP）、管道并行（PP）和张量并行（TP）等并行化技术。

These methods enable the efficient distribution of training tasks across multiple GPUs.

这些方法可以有效地将训练任务分配到多个GPU上。

The process relies on collective communication libraries (CCLs) to manage data exchanges, such as using AllReduce for gradient synchronization or AllGather for parameter sharing.

这一过程依赖于集体通信库（CCLs）来管理数据交换，比如使用AllReduce进行梯度同步或使用AllGather进行参数共享。

The CCL breaks each collective communication task into a series of peer-to-peer Send and Receive operations to carry out the necessary data transfers between GPUs.

CCL将每个集体通信任务分解成一系列点对点的发送和接收操作，以进行GPU之间所需的数据传输。

In Alibaba’s clusters, each server contains multiple GPUs.

在阿里巴巴的集群中，每台服务器都包含多个GPU。

GPUs within the same server are connected through a high-bandwidth intra-host network, such as NVLink or NVLink Switch [40], and each GPU connects to the inter-host RDMA network via network interface cards (NICs).

同一服务器内的GPU通过高带宽的主机内部网络互连，如NVLink或NVLink Switch [40]，每个GPU通过网络接口卡（NIC）连接到主机间的RDMA网络上。

For example, in line with [23], our A100 servers are equipped with eight NVIDIA A100 GPUs [1] and four NVIDIA CX6Dx NICs, each providing 2×100Gbps [6] bandwidth.

例如，参照 [23] 的说法，我们的A100服务器配备了8个NVIDIA A100 GPU [1] 和四个NVIDIA CX6Dx NIC，每个提供2×100Gbps [6] 带宽。

Each GPU is linked to other GPUs in the same server via a 600GB/s NVLink and to GPUs in other servers via a 100Gbps RDMA network.

每个GPU通过600GB/s的NVLink与同一服务器中的其他GPU连接，通过100Gbps的RDMA网络与其他服务器中的GPU连接。

In production, Megatron and DeepSpeed are the two dominant frameworks, NCCL [36] is the dominant CCL.

在生产中，Megatron和DeepSpeed是两个主导框架，NCCL [36] 是主要的CCL。

## 2.2 Demands for a Unified Simulator

The rapid evolution of LLMs necessitates advancements in AI training infrastructure and optimization methods.

LLMs的快速演进需要推进人工智能训练基础设施和优化方法的进步。

To address these challenges, simulations are crucial for three primary goals:

为了应对这些挑战，模拟对于三个主要目标至关重要：

**Comprehensive evaluation of AI infrastructure**. To ensure the effective deployment of new hardware and configurations, AI infrastructure must be evaluated from multiple perspectives:

全面评估人工智能基础设施。为了确保新硬件和配置的有效部署，人工智能基础设施必须从多个角度进行评估：

- GPU Selection: Before adopting new GPU models, cloud service providers (CSPs) need to evaluate their performance on AI workloads at scale.
- GPU选择：在采用新的GPU型号之前，云服务提供商（CSPs）需要评估它们在大规模AI工作负载下的性能。
- Network Architecture Design: Once specific GPUs and intra-host interconnects are chosen, the next challenge is optimizing network architecture for scalability.
- 网络架构设计：一旦选择了特定的GPU和主机内互连，下一个挑战就是优化网络架构以实现可扩展性。
- Host Architecture Design: Evaluating different host configurations for each type of GPU is essential to determine the optimal number of GPUs per host and the best intra-host interconnect.
- 主机架构设计：评估每种GPU类型的不同主机配置对于确定每台主机的最佳GPU数量和最佳的主机内互连至关重要。

**Cost-effective validation of optimizations.** In addition to hardware evaluations, simulations are indispensable for validating new optimization techniques during model development and system upgrades. This involves:


**优化的成本效益验证。**除了硬件评估外，模拟在模型开发和系统升级期间验证新的优化技术是不可或缺的。其中包括：

Parameter Tuning: Testing a variety of model parameters and training framework settings is critical to achieving optimal performance.

参数调整：测试各种模型参数和训练框架设置对于实现最佳性能至关重要。

Evaluating New Mechanisms: As innovative enhancements—such as new training frameworks, collective communication methods, and network congestion control algorithms—are introduced, simulations provide a low-cost method for evaluating their effectiveness in realistic settings.

评估新机制：随着创新增强——例如新的训练框架、集体通信方法和网络拥塞控制算法的引入，模拟提供了一个在真实环境中评估它们有效性的低成本方法。

**Development of a unified simulation framework.** Given the diverse needs for simulations across different components and layers of the AI infrastructure, a unified simulation framework is essential.


**统一模拟框架的开发。** 考虑到跨越人工智能基础设施不同组件和层面的多样化的模拟需求，统一的模拟框架是至关重要的。

Our goal is to develop a unified simulator that addresses all these requirements in a single platform.

我们的目标是开发一个统一的模拟器，在一个平台上满足所有这些需求。

This unified approach will enable consistent, high-precision simulations across different layers of the AI infrastructure, ensuring that teams can validate new designs and optimizations accurately and efficiently.

这个统一的方法将能够在人工智能基础设施的不同层面实现一致的、高精度的模拟，确保团队能够准确高效地验证新的设计和优化。

## 2.3 Our Goals

**Generating workloads that reflect real-world training.** To achieve accurate simulation results, realistic input sources—capturing the detailed behaviors of training frameworks—are essential. Simply estimating workload based on the required floating-point operations is too coarse-grained. Some approaches, like Chakra [53], improve this by using trace-driven methods to extract function-level data from PyTorch Execution Trace. However, this only works for LLMs with the same parameters and scale, limiting the ability to simulate new models or configurations.

**为了获得准确的模拟结果，需要生成反映真实训练情况的工作负载。** 捕捉训练框架的详细行为的真实输入源至关重要。简单地根据所需的浮点运算量来估算工作负载过于粗糙。一些方法，如Chakra [53]，通过使用基于跟踪的方法从PyTorch执行跟踪中提取函数级别的数据来改进这一点。但是，这仅适用于具有相同参数和规模的LLM，限制了对新模型或配置的模拟能力。

Goal 1: We need a flexible and precise workload generator that can handle various models, parameters, and scales.
目标1：我们需要一个灵活且精确的工作负载生成器，可以处理各种模型、参数和规模。

**High-fidelity communication simulation.** Classical network simulators, such as NS-3 [47] and OMNET++ [55], offer packet-level network behavior simulations but don’t address the collective communication used in distributed LLM training. To maximize performance, collective communication libraries (e.g., NCCL) apply various optimizations that affect traffic patterns. Simulating these from scratch can lead to low fidelity.

*8高保真度的通信模拟。**经典的网络模拟器，如NS-3 [47]和OMNET++ [55]，提供了基于数据包的网络行为模拟，但并未涉及分布式LLM训练中使用的集体通信。为了最大化性能，集体通信库（例如NCCL）应用各种影响流量模式的优化。从零开始模拟这些可能导致保真度较低。

Goal 2: We need a high-precision collective communication simulator that incorporates key optimizations and enhancements.
目标2：我们需要一个高精度的集体通信模拟器，融入关键优化和增强。

**High-fidelity computation simulation.** Current solutions like GPGPU-Sim [2] simulate GPU kernel computations at a detailed level but are too time-consuming for large-scale LLM simulations. Other approaches, such as ASTRA-sim [45], fail to support different GPUs or lack the necessary precision.

**高保真度的计算模拟。**当前的解决方案，如GPGPU-Sim [2]，在细节级别模拟GPU内核计算，但对于大规模LLM模拟来说耗时过长。其他方法，如ASTRA-sim [45]，不支持不同的GPU或缺乏必要的精度。

Goal 3: We need an efficient computation simulator that delivers both precision and scalability for large-scale simulations.
目标3：我们需要一个既精确又可扩展的高效计算模拟器，用于大规模模拟。

**Fast simulation speed.** Using a combination of current methods (i.e., PyTorch trace generator with ASTRA-sim), simulating a single iteration of GPT-3 training with 128 GPUs can take an entire day, while the same task on real hardware takes just two seconds. Efficiency is critical to scale simulations for practical use.

快速模拟速度。使用目前的方法的组合（例如，PyTorch跟踪生成器与ASTRA-sim），使用128个GPU模拟GPT-3训练的单次迭代可能需要整整一天，而在真实硬件上完成同样的任务只需两秒钟。效率对于扩展模拟以实际应用至关重要。

Goal 4: The simulator must not only meet Goals 1-3 but also be scalable and capable of running large-scale LLM simulations efficiently.
目标4：模拟器不仅要满足目标1-3，还必须具有可扩展性，并且能够高效地运行大规模LLM模拟。

# 3 The SimAI Simulator

## 3.1 SimAI Overview

Figure 1 illustrates the key components of SimAI. 
图1说明了SimAI的关键组件。

Each simulation request includes detailed information about the training process, such as the model itself and parameters, training framework configurations, CCL parameters, and the intra/inter-host network topology.
每个模拟请求包括关于训练过程的详细信息，如模型本身和参数、训练框架配置、CCL参数以及主机内/主机间网络拓扑结构。

Workload Generator (SimAI-WG) generates realistic workloads for each simulation request (§3.2). 
SimAI的工作负载生成器（SimAI-WG）为每个模拟请求生成真实的工作负载（§3.2）。

The output, called a workload description file, outlines algorithm modules, collective communication operations, and their dependencies. 
输出的工作负载描述文件概述了算法模块、集体通信操作及其依赖关系。

The workload file is then processed by the Execution Engine, which simulates the execution of both computation and communication operations as discrete events. 
然后工作负载文件由执行引擎处理，该引擎将计算和通信操作的执行模拟为离散事件。

We utilize the Computation Simulator (SimAI-CP) and the Communication Simulator (SimAI-CM) to simulate computation and communication tasks, respectively. 
我们利用计算模拟器（SimAI-CP）和通信模拟器（SimAI-CM）来分别模拟计算和通信任务。

SimAI-CP transforms submodules into detailed kernels, providing precise computation simulations using a self-built, fine-grained operation library (§3.3). 
SimAI-CP将子模块转换为详细的内核，使用自建的细粒度操作库提供精确的计算模拟（§3.3）。

SimAI-CM integrates parts of NCCL, breaking down each collective communication into peer-to-peer operations to deliver accurate communication simulation results (§3.4).
SimAI-CM集成了NCCL的部分内容，将每个集体通信分解为点对点操作，以提供准确的通信模拟结果（§3.4）。

Additionally, we implement multi-threaded acceleration and lock-free global context sharing to boost simulation speed further (§3.5).
此外，我们实施多线程加速和无锁全局上下文共享以进一步提高模拟速度（§3.5）。

![image](https://github.com/user-attachments/assets/215a66ba-ca90-4e7f-9aa9-75a112fd0540)


