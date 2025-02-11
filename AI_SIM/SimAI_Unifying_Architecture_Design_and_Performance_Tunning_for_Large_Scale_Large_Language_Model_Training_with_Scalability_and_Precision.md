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

**高保真度的通信模拟。** 经典的网络模拟器，如NS-3 [47]和OMNET++ [55]，提供了基于数据包的网络行为模拟，但并未涉及分布式LLM训练中使用的集体通信。为了最大化性能，集体通信库（例如NCCL）应用各种影响流量模式的优化。从零开始模拟这些可能导致保真度较低。

Goal 2: We need a high-precision collective communication simulator that incorporates key optimizations and enhancements.
目标2：我们需要一个高精度的集体通信模拟器，融入关键优化和增强。

**High-fidelity computation simulation.** Current solutions like GPGPU-Sim [2] simulate GPU kernel computations at a detailed level but are too time-consuming for large-scale LLM simulations. Other approaches, such as ASTRA-sim [45], fail to support different GPUs or lack the necessary precision.

**高保真度的计算模拟。** 当前的解决方案，如GPGPU-Sim [2]，在细节级别模拟GPU内核计算，但对于大规模LLM模拟来说耗时过长。其他方法，如ASTRA-sim [45]，不支持不同的GPU或缺乏必要的精度。

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


## 3.2 Workload Generator

§3.2.1 outlines the process for generating precise workload files for a given model. 

To further maximize the benefits for real-world training scenarios, selecting a representative benchmark suite is crucial, as discussed in §3.2.2.

### 3.2.1 Generating workload files

Generating precise workload files is essential, as even small discrepancies between the workload file and actual training execution can lead to low simulation fidelity.

生成精确的工作负载文件至关重要，即使工作负载文件与实际训练执行之间存在微小差异都可能导致模拟的精度下降。

While trace-based methods are highly accurate, they require physical clusters to run.

虽然基于跟踪的方法非常准确，但它们需要在物理集群中运行。

SimAI-WG takes a different approach by “hijacking” existing training frameworks to produce workloads identical to real tasks.

SimAI-WG采用了一种不同的方法，通过“劫持”现有的训练框架来生成与真实任务完全相同的工作负载。

We have implemented this for Megatron and DeepSpeed, two of the most popular frameworks in our production environment.

我们已在Megatron和DeepSpeed两个在我们生产环境中最流行的框架上实现了这一点。

**Generating workloads using a single host.** 
使用单台主机生成工作负载。

Without access to a large-scale GPU cluster, the key challenge for this "hijacking" approach is running it on a single host while considering the interactions of multiple peer hosts.

在无法访问大规模GPU集群的情况下，这种"劫持"方法的关键挑战在于：在单台主机上运行时需要同时考虑多个对等主机的交互。

To address this, we made two modifications to the framework and NCCL:

为此，我们对框架和NCCL进行了两处修改：

+ The framework is tricked into believing it runs in a cluster with the target number of GPUs. The inter-host topology is also set to simulate a universal cluster configuration.

+ 欺骗框架使其认为运行在具有目标GPU数量的集群中，同时设置主机间拓扑结构以模拟通用集群配置。

+ All real communication in NCCL is skipped. For workloads involving pipeline parallelism, SimAI-WG must be configured with the appropriate rank number.

+ 跳过NCCL中所有实际通信。对于涉及流水线并行的工作负载，必须为SimAI-WG配置适当的秩编号。

This allows the training framework to generate sequences of computation and communication operations, including algorithm submodules and collective or peer-to-peer communications.

这使得训练框架能够生成计算和通信操作的序列，包括算法子模块以及集体或点对点通信。

![image](https://github.com/user-attachments/assets/6df2d94f-23d2-43c4-9a60-683a15094e42)


Table 1 provides examples of the typical operations used in LLMs.

表1展示了大型语言模型中使用的典型操作示例。

However, these operations lack dependency specifications, which need further specifications.

然而这些操作缺乏依赖关系规范，需要进一步明确具体要求。

![image](https://github.com/user-attachments/assets/c826ad8e-1da0-4513-8b1d-319ff58b36e6)


**Defining operation dependencies.**

**定义操作依赖关系。**


Since computation and communication operations overlap during execution, we embed dependency information in the workload file to reflect this.

由于计算和通信操作在运行时存在重叠，我们将依赖关系信息嵌入工作负载文件以反映这一特性。

Dependencies are determined based on the parallelization framework used and are validated using Nvidia Nsight Systems on a 1,024-GPU cluster.

依赖关系基于所使用的并行化框架确定，并通过在1,024-GPU集群上使用英伟达Nsight Systems工具进行验证。

Figure 2 illustrates an example of dependencies when using TP, DP, and PP parallelisms.

图2展示了使用张量并行（TP）、数据并行（DP）和流水线并行（PP）时的依赖关系示例。

Solid arrows indicate a strict "happened-before" relationship, while dotted arrows represent overlapping operations where communication begins after the computation has started.

实线箭头表示严格的"happened-before"时序关系，虚线箭头则表示计算启动后通信才开始的重叠操作。

The figure demonstrates how Megatron’s attention and MLP backward phases benefit from overlapping optimizations.

该图展示了Megatron模型中注意力机制和MLP反向传播阶段如何通过重叠优化获得性能提升。

For example, the MLPbackward submodule starts only after the Attentionbackward submodule and the AllReduce operation are complete.

例如，MLP反向传播子模块仅在注意力反向传播子模块和AllReduce操作完成后才会启动。

We decoupled the workload from the training framework for independent simulations.

我们将工作负载与训练框架分离，进行独立的模拟。

Instead of embedding simulation code directly into frameworks like Owl [9], we used SimAI-WG to extract model workloads and simulate them within the SimAI execution engine.

我们没有直接将模拟代码嵌入像Owl [9]这样的框架，而是使用SimAI-WG来提取模型工作负载，并在SimAI执行引擎内对其进行模拟。

This approach avoids the substantial overhead of modifying training frameworks, which requires extensive changes to monitoring tools, data pipelines, and other critical components.

这种方法避免了修改训练框架所带来的重大开销，这需要对监控工具、数据管道和其他关键组件进行广泛的更改。

By implementing core algorithms in SimAI-WG, we gain flexibility while understanding how different frameworks generate communication and computation operators.

通过在SimAI-WG中实现核心算法，我们在了解不同框架如何生成通信和计算操作符的同时获得了灵活性。

Communication in large language models. In LLM training, metadata exchanges and barrier operation (under 1 KB) are negligible and not a focus of our simulations.

大语言模型的通信。 在LLM训练中，元数据交换和栅栏操作（小于1 KB）是可以忽略的，也不是我们模拟的重点。

Communication time is primarily introduced by various parallelism techniques, such as TP, PP, EP, and DP.

通信时间主要由各种并行技术引入，比如TP，PP，EP和DP。

TP, PP, and EP typically have fixed communication patterns and volumes, unaffected by cluster size, involving medium-sized messages ranging from tens to hundreds of megabytes.

TP，PP和EP通常具有固定的通信模式和量，不受集群大小的影响，涉及从几十兆字节到几百兆字节的中等大小消息。

In contrast, the communication scope of DP expands with the increased cluster size, often involving gigabyte-scale collective communications (such as AllGather / ReduceScatter) across hundreds to thousands of nodes.

相比之下，DP的通信范围随着集群规模的增加而扩大，通常涉及跨数百到数千个节点的以吉字节为单位的收集通信（例如AllGather / ReduceScatter）。

### 3.2.2 Determining the model-level benchmark suite

As a unified simulator designed to provide comprehensive evaluations of end-to-end performance in real-world LLM training scenarios, it’s essential to establish a series of model-level benchmarks, referred to as the benchmark suite.

作为一个统一的模拟器，旨在为现实世界的LLM训练场景提供全面的端到端性能评估，建立一系列模型级别的基准测试非常重要，称之为基准测试套件。

![image](https://github.com/user-attachments/assets/3613bdee-c647-4811-b600-afe6b6e50ee2)

Model and framework selections.

模型和框架选择。

Table 2 shows the distribution of model parameters and task scales over the past six months.

表2显示了过去六个月模型参数和任务规模的分布情况。

The models fall into three categories—small, medium, and large—each making up over 20% of the total.

这些模型分为三类——小型、中型和大型，每类模型占总量的20%以上。

More than 94% of LLMs are variants of GPT-3 or LLaMA, with Megatron and DeepSpeed being the dominant frameworks.

超过94%的LLM模型是GPT-3或LLaMA的变体，其中Megatron和DeepSpeed是主导框架。

Although Megatron usage is on the rise, covering all models is impractical. Instead, we include a selection of opensource LLMs of varying sizes.

尽管Megatron的使用正在增加，但覆盖所有模型是不切实际的。因此，我们包括了一系列不同大小的开源LLM模型。

**Parameter selections.** We focus on the parameters that impact workload patterns:

参数选定。我们关注影响工作负载模式的参数：

+ Model Parameters (e.g., hidden_size, num_layers, seq_len, etc.)
+ 模型参数（例如hidden_size、num_layers、seq_len等）
+ Framework Parameters (e.g., world size, zero level, reduce_bucket_size/allgather_bucket_size, parallelization strategies like TP, PP, DP, and SP)
+ 框架参数（例如world size、zero level、reduce_bucket_size/allgather_bucket_size、像TP、PP、DP和SP这样的并行化策略）。

![image](https://github.com/user-attachments/assets/5caa85d7-a689-45c2-9728-84b162afbc57)

Based on the statistics in Table 2, we chose a minimal set of benchmarks that cover typical settings without exploring all possible combinations, as detailed in Table 3.

根据表2中的统计数据，我们选择了一组涵盖典型设置的基准测试，而未探索所有可能的组合，详细内容如表3所示。

Internal reports indicate that model architects frequently adjust framework parameters to optimize performance or precision. Users can also generate custom benchmarks with SimAI-WG as explained in §3.2.1.

内部报告表明，模型架构师经常调整框架参数以优化性能或精度。用户也可以根据第3.2.1节中SimAI-WG的说明生成自定义基准测试。

We believe this benchmark suite accurately represents the real LLM training workloads commonly used by our customers.

我们相信这个基准测试套件准确地代表了我们客户常用的真实LLM训练工作负载。

Unless otherwise specified, the benchmarks in this suite will be used for evaluation and discussion throughout the rest of the paper.

除非另有说明，否则本套件中的基准将用于本文其余部分的评估和讨论。

## 3.3 Precise Computation Simulation

**Precisely simulating existing GPUs.** As explained in § 3.2, by running the mocked framework on a single host, we obtain the detailed submodule workflow.

精确模拟现有的GPU。如 § 3.2中所解释的，在单台主机上运行模拟框架，我们可以获得详细的子模块工作流程。

For simulations targeting existing GPUs, SimAI-WG outputs the execution times for all submodules on a host with the corresponding GPU.

针对现有GPU的模拟，SimAI-WG会输出在具有相应GPU的主机上所有子模块的执行时间。

Since each GPU in practical LLM training is dedicated to a single task, we can accurately simulate the entire computation procedure by following the workload file and filling in the execution times for each submodule.

由于实际的LLM训练中每个GPU都专用于单个任务，我们可以通过跟踪工作负载文件并填入每个子模块的执行时间来准确模拟整个计算过程。

As shown in §4.3, the simulation precision ranges from 96.9% to 99.5%.

如 §4.3所示，模拟精度在96.9%到99.5%之间。

We maintain an operation database in SimAI-CP that records the execution times for all submodules in the benchmark suite.

我们在SimAI-CP中维护一个操作数据库，记录了基准测试套件中所有子模块的执行时间。

![image](https://github.com/user-attachments/assets/55585525-6eb4-484a-98c7-50ccb7f81e4d)

Table 4 lists common computation operators and their execution times on various GPUs and configurations.

表4列出了各种GPU和配置上常见计算操作符及其执行时间。

For workloads outside the benchmark suite, specific GPU tests are required to gather additional data.

对于基准测试套件之外的工作负载，需要进行特定的GPU测试以收集额外的数据。

**Fine-grained kernel simulation.** While submodule-level simulation works in many cases, it may not be suitable for all scenarios, especially when new parallelization strategies or optimizations reorganize or refine kernels for better performance.

精细的内核模拟。 尽管在许多情况下，子模块级的模拟效果很好，但并非适用于所有情景，特别是当新的并行化策略或优化重组或优化内核以获得更好的性能时。

In such cases, fine-grained simulation is necessary We have designed a module-kernel converter to break down each submodule into smaller kernels, which are then tested on different GPUs.

在这种情况下，需要进行精细的模拟。我们设计了一个模块-内核转换器，将每个子模块分解为更小的内核，然后在不同的GPU上进行测试。

The third column in Table 1 shows an example of kernels used in different submodules. This further enriches the operation database, enabling precise simulation of advanced optimizations and new features.

表1的第三列显示了在不同子模块中使用的内核的示例。这进一步丰富了操作数据库，使其能够精确模拟高级优化和新特性。

**Supporting unreleased GPUs.**As a simulation service for CSPs, there is strong demand for simulating GPUs that have not yet been released. Decision-makers need to evaluate whether purchasing new GPUs is worth the investment, considering factors like budget, host architecture, and network architecture.

支持未发布的GPU。 作为云服务提供商的模拟服务，有强烈的需求来模拟尚未发布的GPU。决策者需要评估购买新GPU是否值得投资，考虑因素包括预算、主机架构和网络架构。

Although physical access to unreleased GPUs is not possible, we may have access to core specifications or a specification sheet. An initial approach might be to estimate computation times by scaling known values from existing GPUs, but this often results in significant inaccuracies—up to 25.1% deviation.

尽管无法获取尚未发布的GPU的实际访问权限，但我们可能可以获取核心规格或规格表。一个初始方法可能是通过对现有GPU的已知值进行缩放，估算计算时间，但这往往会导致显着的不准确性，最高可达25.1%的偏差。

Our analysis shows that different kernels have different performance bottlenecks, typically falling into two categories: computation-intensive or memory-bandwidth-intensive.

我们的分析显示，不同的内核有不同的性能瓶颈，通常分为两类：计算密集型或内存带宽密集型。

For example, the Gemm kernel, used for updating the KV cache, is memory-bandwidth-intensive, while flash attention is computation-intensive. Detailed classifications are listed in Table 1.

例如，用于更新KV缓存的Gemm kernel是内存带宽密集型，而flash attention是计算密集型。详细的分类列在表1中。

To improve accuracy, we propose using two equations tailored to these kernel types, based on data from our operation database and the Roofline performance model [58].

为了提高准确性，我们提出使用两个针对这些内核类型量身定制的方程，基于我们的操作数据库和Roofline性能模型[58]的数据。

These equations, based on measured execution times of compute-bound and memory-bound kernels (subscript Comp_Known and Mem_Known, respectively) in the existing environment, allow us to calculate execution times for these kernels on new GPUs or configurations (subscript Comp_New and Mem_New, respectively):
这些方程基于现有环境中计算密集型和内存密集型内核（分别用Comp_Known和Mem_Known表示）的实测执行时间，使我们能够计算出这些内核在新GPU或配置上的执行时间（分别用Comp_New和Mem_New表示）:

![image](https://github.com/user-attachments/assets/99eb0387-4f92-4db7-905c-4b89900b31f5)

![image](https://github.com/user-attachments/assets/99d7c86c-91db-4cd0-8b3e-a95463e888b3)

To ensure accurate results, we recommend using GPUs with similar architectures as baselines, rather than generalizing across different vendors. 

为了确保结果准确，我们建议使用具有类似架构的 GPU 作为基线，而不是跨不同供应商进行推广。

In our experiments, we use the Nvidia A100 as the baseline for calculating kernel execution times on other Nvidia GPUs. We refer to this method as SimAI-CP-Model.

在我们的实验中，我们使用 Nvidia A100 作为计算其他 Nvidia GPU 上的内核执行时间的基准。我们将此方法称为 SimAI-CP 模型。

## 3.4 Precise Simulation of Communication

Training frameworks rely on collective communication libraries, with NCCL [36] being the most widely used in production. 

训练框架依赖于集体通信库，其中 NCCL[36] 在生产中使用最为广泛。

NCCL translates collective communication operations (e.g., AllReduce, AllGather, ReduceScatter) into network-level operations (e.g., Send, Receive). 

NCCL 将集体通信操作（例如，AllReduce、AllGather、ReduceScatter）转换为网络级操作（例如，发送、接收）。

This complex process involves selecting optimal algorithms based on factors like the number of nodes, message size, and configured parameters. 

这个复杂的过程涉及根据节点数量、消息大小和配置参数等因素选择最佳算法。

Even small mismatches between practical execution and simulation can cause significant deviations in results. 

实际执行和模拟之间即使有微小的不匹配也会导致结果的显著偏差。

To accurately reproduce these algorithm selections, SimAI integrates key procedures from NCCL directly

为了准确重现这些算法选择，SimAI 直接集成了 NCCL 的关键程序

Reproducing NCCL’s key procedures. SimAI-CM uses a modified version of NCCL, called SimCCL, to intercept key operations. 

重现 NCCL 的关键程序。 SimAI-CM 使用 NCCL 的修改版本（称为 SimCCL）来拦截关键操作。

SimCCL captures the initialization and core interfaces of collective communication to generate peer-topeer communication lists.

SimCCL 捕获集体通信的初始化和核心接口以生成点对点通信列表。

Since simulations typically run on a single host, SimCCL employs a "hijacking" technique to simulate the detailed peerto-peer operations, similar to the approach discussed in §3.2.1, but at the collective communication level. 

由于模拟通常在单个主机上运行，​​SimCCL 采用“劫持”技术来模拟详细的点对点操作，类似于§3.2.1 中讨论的方法，但在集体通信层面。

Here’s how SimCCL modifies NCCL’s behavior:

1. NCCL Initialization: SimCCL intercepts the NcclCommInitAll function using the libhacknccl.so in Figure 3, creating virtual communicators for each GPU.

NCCL 初始化：SimCCL 使用图 3 中的 libhacknccl.so 拦截 NcclCommInitAll 函数，为每个 GPU 创建虚拟通信器。

This makes the system behave as though it’s running in a real multi-GPU cluster, allowing for socket connections and data exchanges during the initialization phase. 

这使得系统的行为就像在真实的多 GPU 集群中运行一样，允许在初始化阶段进行套接字连接和数据交换。

In the bootstrap and network initialization stages, multiple virtual communicators are created in the same communication group, while only one communicator is actually created.

在引导和网络初始化阶段，在同一个通信组中会创建多个虚拟通信器，但实际只创建一个通信器。

2. Topology Discovery: Instead of searching actual PCIe devices, SimCCL reads a user-specified topology file that defines GPU, NIC, and PCIe switch configurations.

2. 拓扑发现：SimCCL 不是搜索实际的 PCIe 设备，而是读取用户指定的拓扑文件，该文件定义 GPU、NIC 和 PCIe 交换机配置。

Each virtual communicator processes the topology independently and no synchronization is required.

每个虚拟通信器独立处理拓扑，不需要同步。

3. Intra-Host Communication Channel Creation: SimCCL sets up channels between virtual communicators within the host and stores the details in an isolated information table.

3. 主机内通信通道创建：SimCCL 在主机内的虚拟通信器之间建立通道，并将详细信息存储在独立的信息表中。

4. Inter-Host Communication Channel Creation: SimCCL bypasses gathering information from other GPUs using _AllGather_ operations, as it already has information on all hosts. It creates inter-host channels directly.

4. 主机间通信通道创建：SimCCL 绕过使用“AllGather”操作从其他 GPU 收集信息，因为它已经拥有所有主机的信息。它直接创建主机间通道。

5. Collective Communication Transformation: SimCCL intercepts collective communication calls, reconstructing the operations to trace lower-level communications. It skips actual data transfers and captures inter-GPU communication events, including data size, sender and receiver ranks, and routes, to simulate RDMA-layer behavior.

5. 集体通信转换：SimCCL 拦截集体通信调用，重建操作以追踪低级通信。它跳过实际的数据传输并捕获 GPU 间通信事件，包括数据大小、发送方和接收方等级以及路由，以模拟 RDMA 层行为。

**Supporting all NCCL parameters.** SimCCL reflects the communication behavior for the vast majority of NCCL parameters [37]. 

**支持所有 NCCL 参数。** SimCCL 反映了绝大多数 NCCL 参数的通信行为 [37]。

SimAI-CM has been enhanced to support specific features like PCI × NVLink (PXN). 

SimAI-CM 已得到增强，可支持 PCI × NVLink (PXN) 等特定功能。

PXN allows a GPU to use a non-local NIC (in the same node) through NVLINK connections for data transfers. 

PXN 允许 GPU 通过 NVLINK 连接使用非本地 NIC（在同一节点中）进行数据传输。

This design is typically employed in rail-optimized network topologies, enabling cross-node network traffic to remain on the same rail (single-hop switch) to achieve message aggregation and network traffic optimization.

这种设计通常用于轨道优化的网络拓扑中，使得跨节点的网络流量能够保持在同一轨道（单跳交换机）上，以实现消息聚合和网络流量优化。

By setting an appropriate NCCL_P2P_PXN_LEVEL, SimCCL can recognize these PXN traffic patterns and reflect them in the output FlowModel. 

通过设置适当的 NCCL_P2P_PXN_LEVEL，SimCCL 可以识别这些 PXN 流量模式并将其反映在输出 FlowModel 中。

For instance, if rank 1 sends data to rank 8 via rank 0 as an intermediary, the FlowModel represents this as two separate flows: 1->0 and 0->8. 

例如，如果级别 1 通过级别 0 作为中介将数据发送到级别 8，则 FlowModel 将其表示为两个单独的流：1->0 和 0->8。

Subsequently, SimAI-CM extracts these patterns and simulates them accordingly.

随后，SimAI-CM 提取这些模式并进行相应的模拟。

The SimCCL module is designed to emulate the communication operation processing workflow in NCCL or other CCL. 

SimCCL模块旨在模拟NCCL或其他CCL中的通信操作处理工作流程。

It focuses on transforming collective communication operations into a set of easily interpretable point-to-point communications, without incorporating actual data verification or integrity checks. 

它专注于将集体通信操作转换为一组易于解释的点对点通信，而不包含实际的数据验证或完整性检查。

This approach generally does not affect the end-to-end training process, as it doesn’t lead to exceptional workflows due to the absence of real data. 

这种方法通常不会影响端到端的训练过程，因为它不会因缺乏真实数据而导致异常的工作流程。

In expert parallelism (EP), the gating module’s token distribution is influenced by data values. 

在专家并行（EP）中，门控模块的令牌分布受数据值的影响。

In the simulation, we assume a balanced distribution, which has minimal impact on the simulation results.

在模拟中，我们假设一个平衡分布，这对模拟结果的影响最小。

**Porting efforts.** It took us over 10,000 lines of coding efforts for building the blocks shown in Figure 1. 

**移植工作。**我们花费了超过 10,000 行编码工作来构建图 1 所示的块。

The majority of the efforts are intended for repeated use except for the SimCCL module. 

除 SimCCL 模块外，大部分努力都是为了重复使用。

For example, to support a different version of NCCL or a new CCL, we need to re-adapt the SimCCL module. 

例如，为了支持不同版本的NCCL或新的CCL，我们需要重新调整SimCCL模块。

However, our design does not incur intrusive modifications on the original CCL codebase. 

但是，我们的设计不会对原始 CCL 代码库造成侵入性修改。

As shown in Table 5, only 572 lines of codes (LOC) are essential based on the original NCCL codebase

如表 5 所示，根据原始 NCCL 代码库，仅 572 行代码 (LOC) 是必需的

## 3.5 Large Scale Simulation Speedup

At the beginning of SimAI’s development, simulating a single iteration of GPT-3 training with 128 GPUs took over 24 hours, compared to just two seconds on a physical GPU cluster. 

在 SimAI 开发之初，使用 128 个 GPU 模拟一次 GPT-3 训练迭代需要超过 24 小时，而在物理 GPU 集群上只需两秒钟。

This challenge is similar to AstraSim, which also uses NS-3 for network simulation. 

此次挑战与AstraSim类似，也使用NS-3进行网络模拟。

To enable simulations of AI infrastructure with more than 1,000 GPUs, we implemented multi-threaded acceleration for SimAI-CM.

为了实现具有超过 1,000 个 GPU 的 AI 基础设施模拟，我们为 SimAI-CM 实现了多线程加速。

Several approaches have been proposed to speed up network simulations. 

已经提出了几种方法来加速网络模拟。

For example, parallel discrete-event simulation (PDES) [10, 18] and UNISON [3] distribute network topologies across multiple logical processes, each running on separate CPU cores.

例如，并行离散事件模拟 (PDES) [10, 18] 和 UNISON [3] 将网络拓扑分布在多个逻辑进程之间，每个逻辑进程都在单独的 CPU 核心上运行。

We chose UNISON [3] for three key reasons: 

我们选择 UNISON [3] 主要有三个原因：

(1) It is open-source and builds on NS-3. 

(1) 它是开源的并且基于 NS-3 构建。

(2) It automates the partitioning of network topologies and schedules each task to the appropriate thread. 

(2) 自动划分网络拓扑，并将各个任务调度到适当的线程。

(3) It has superior scalability, as demonstrated in their evaluations.

(3) 评估结果显示，它具有卓越的可扩展性。

**Lock-free sharing of global variables.** However, integrating UNISON into SimAI presented a significant challenge. 

**无锁共享全局变量。** 然而，将 UNISON 集成到 SimAI 中面临着巨大的挑战。

As the simulation scale increased, a large number of global configurations and contexts were shared across threads. 

随着模拟规模的增加，大量的全局配置和上下文在线程之间共享。

Updating shared data structures with global locks, even atomic ones, caused performance bottlenecks. 

使用全局锁（即使是原子锁）更新共享数据结构也会导致性能瓶颈。

As shown in Figure 4(a), these global variables track metadata on inter-host communications, such as data volumes between nodes and queue lengths at switches. 

如图 4（a）所示，这些全局变量跟踪主机间通信的元数据，例如节点之间的数据量和交换机的队列长度。

Atomic operations on these variables slowed down concurrent thread execution.

这些变量上的原子操作减慢了并发线程的执行速度。

To solve this, we restructured SimAI to run without global locks. 

为了解决这个问题，我们重组了 SimAI，使其无需全局锁即可运行。

We observed that most global variables were accessed by specific threads. 

我们观察到大多数全局变量都被特定线程访问。

By managing these variables in a threadindependent way, we eliminated the need for global locks. 

通过以线程独立的方式管理这些变量，我们消除了对全局锁的需要。

Figure 4(b) illustrates how SimAI separates these variables, enabling lock-free operation. 

图 4(b) 说明了 SimAI 如何分离这些变量，实现无锁操作。

Since each node’s simulation runs on a single thread, we isolate global variables in a node ID-indexed table.

由于每个节点的模拟在单个线程上运行，我们在节点 ID 索引表中隔离全局变量。

This allows threads to access the relevant data without locking, reducing the performance issues caused by concurrency. 

这使得线程无需锁定即可访问相关数据，从而减少并发引起的性能问题。

Our lock-free optimization resulted in a 23x speedup compared to the single-thread version and a 15% improvement over the multi-threaded version.

我们的无锁优化使速度比单线程版本提高了 23 倍，比多线程版本提高了 15%。
