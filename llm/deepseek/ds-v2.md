# DeepSeek-V2: A Strong, Economical, and Efficient Mixture-of-Experts Language Model

https://arxiv.org/pdf/2405.04434

# Abstract
  
We present DeepSeek-V2, a strong Mixture-of-Experts (MoE) language model characterized by economical training and efficient inference. It comprises 236B total parameters, of which 21B are activated for each token, and supports a context length of 128K tokens. DeepSeek-V2 adopts innovative architectures including Multi-head Latent Attention (MLA) and DeepSeekMoE. MLA guarantees efficient inference through significantly compressing the Key-Value (KV) cache into a latent vector, while DeepSeekMoE enables training strong models at an economical cost through sparse computation. Compared with DeepSeek 67B, DeepSeek-V2 achieves significantly stronger performance, and meanwhile saves 42.5% of training costs, reduces the KV cache by 93.3%, and boosts the maximum generation throughput to 5.76 times. We pretrain DeepSeek-V2 on a high-quality and multi-source corpus consisting of 8.1T tokens, and further perform Supervised Fine-Tuning (SFT) and Reinforcement Learning (RL) to fully unlock its potential. Evaluation results show that, even with only 21B activated parameters, DeepSeek-V2 and its chat versions still achieve top-tier performance among open-source models. The model checkpoints are available at https://github.com/deepseek-ai/DeepSeek-V2.

我们提出了 DeepSeek-V2，一种强大的混合专家 (MoE) 语言模型，具有训练经济、推理高效的特点。它包含 236B 总参数，其中每个 token 激活 21B，支持 128K token 的上下文长度。DeepSeek-V2 采用了包括多头潜在注意力 (MLA) 和 DeepSeekMoE 在内的创新架构。MLA 通过将键值 (KV) 缓存显著压缩为潜在向量来保证高效推理，而 DeepSeekMoE 通过稀疏计算以经济的成本训练强大的模型。与 DeepSeek 67B 相比，DeepSeek-V2 实现了显著增强的性能，同时节省了 42.5% 的训练成本、减少了 93.3% 的 KV 缓存、并将最大生成吞吐量提升至 5.76 倍。我们在由 8.1T 标记组成的高质量多源语料库上对 DeepSeek-V2 进行了预训练，并进一步执行监督微调 (SFT) 和强化学习 (RL) 以充分释放其潜力。评估结果表明，即使只有 21B 激活参数，DeepSeek-V2 及其聊天版本仍然在开源模型中实现了顶级性能。模型检查点可在 https://github.com/deepseek-ai/DeepSeek-V2 上找到。

# 1 Introduction
  
In the past few years, Large Language Models (LLMs) (Anthropic, 2023; Google, 2023; OpenAI, 2022, 2023) have undergone rapid development, offering a glimpse into the dawn of Artificial General Intelligence (AGI). In general, the intelligence of an LLM tends to improve as the number of parameters increases, allowing it to exhibit emergent capabilities across various tasks (Wei et al., 2022). However, the improvement comes at the cost of larger computing resources for training and a potential decrease in inference throughput. These constraints present significant challenges that impede the widespread adoption and utilization of LLMs. In order to tackle this problem, we introduce DeepSeek-V2, a strong open-source Mixture-of-Experts (MoE) language model, characterized by economical training and efficient inference through an innovative Transformer architecture. It is equipped with a total of 236B parameters, of which 21B are activated for each token, and supports a context length of 128K tokens. 

在过去的几年中，大型语言模型 (LLM) (Anthropic，2023 年；Google，2023 年；OpenAI，2022 年，2023 年) 经历了快速发展，让我们得以一窥通用人工智能 (AGI) 的曙光。一般来说，LLM 的智能往往会随着参数数量的增加而提高，从而使其能够在各种任务中展现出新兴的能力 (Wei et al.，2022 年)。然而，这种改进是以更大的训练计算资源和推理吞吐量的潜在下降为代价的。这些限制带来了重大挑战，阻碍了 LLM 的广泛采用和利用。为了解决这个问题，我们推出了 DeepSeek-V2，这是一个强大的开源混合专家 (MoE) 语言模型，其特点是通过创新的 Transformer 架构进行经济的训练和高效的推理。它总共配备了236B个参数，其中每个token激活21B，支持128K个token的上下文长度。

We optimize the attention modules and Feed-Forward Networks (FFNs) within the Transformer framework (Vaswani et al., 2017) with our proposed Multi-head Latent Attention (MLA) and DeepSeekMoE. (1) In the context of attention mechanisms, the Key-Value (KV) cache of the Multi-Head Attention (MHA) (Vaswani et al., 2017) poses a significant obstacle to the inference efficiency of LLMs. Various approaches have been explored to address this issue, including Grouped-Query Attention (GQA) (Ainslie et al., 2023) and Multi-Query Attention (MQA) (Shazeer, 2019). However, these methods often compromise performance in their attempt to reduce the KV cache. In order to achieve the best of both worlds, we introduce MLA, an attention mechanism equipped with low-rank key-value joint compression. Empirically, MLA achieves superior performance compared with MHA, and meanwhile significantly reduces the KV cache during inference, thus boosting the inference efficiency. (2) For Feed-Forward Networks (FFNs), we follow the DeepSeekMoE architecture (Dai et al., 2024), which adopts fine-grained expert segmentation and shared expert isolation for higher potential in expert specialization. The DeepSeekMoE architecture demonstrates great advantages compared with conventional MoE architectures like GShard (Lepikhin et al., 2021), enabling us to train strong models at an economical cost. As we employ expert parallelism during training, we also devise supplementary mechanisms to control communication overheads and ensure load balance. By combining these two techniques, DeepSeek-V2 features strong performance (Figure 1(a)), economical training costs, and efficient inference throughput (Figure 1(b)), simultaneously

我们利用我们提出的多头潜在注意力 (MLA) 和 DeepSeekMoE 优化了 Transformer 框架 (Vaswani et al., 2017) 中的注意力模块和前馈网络 (FFN)。（1）在注意力机制的背景下，多头注意力 (MHA) (Vaswani et al., 2017) 的键值 (KV) 缓存对 LLM 的推理效率构成了重大障碍。已经探索了各种方法来解决这个问题，包括分组查询注意力 (GQA) (Ainslie et al., 2023) 和多查询注意力 (MQA) (Shazeer, 2019)。然而，这些方法在尝试减少 KV 缓存时往往会损害性能。为了实现两全其美，我们引入了 MLA，一种配备低秩键值联合压缩的注意力机制。经验上，MLA 相比 MHA 取得了更优异的性能，同时显著减少了推理过程中的 KV 缓存，从而提高了推理效率。（2）对于前馈网络（FFN），我们遵循 DeepSeekMoE 架构（Dai et al., 2024），该架构采用细粒度专家细分和共享专家隔离，以实现更高的专家专业化潜力。与 GShard（Lepikhin et al., 2021）等传统 MoE 架构相比，DeepSeekMoE 架构表现出巨大优势，使我们能够以经济的成本训练出强大的模型。由于我们在训练过程中采用了专家并行，我们还设计了补充机制来控制通信开销并确保负载平衡。通过结合这两种技术，DeepSeek-V2 兼具强大的性能（图 1(a)）、经济的训练成本和高效的推理吞吐量（图 1(b)），

We construct a high-quality and multi-source pre-training corpus consisting of 8.1T tokens. Compared with the corpus used in DeepSeek 67B (our previous release) (DeepSeek-AI, 2024), this corpus features an extended amount of data, especially Chinese data, and higher data quality. We first pretrain DeepSeek-V2 on the full pre-training corpus. Then, we collect 1.5M conversational sessions, which encompass various domains such as math, code, writing, reasoning, safety, and more, to perform Supervised Fine-Tuning (SFT) for DeepSeek-V2 Chat (SFT). Finally, we follow DeepSeekMath (Shao et al., 2024) to employ Group Relative Policy Optimization (GRPO) to further align the model with human preference and produce DeepSeek-V2 Chat (RL). 

我们构建了一个由 8.1T 标记组成的高质量、多源预训练语料库。与 DeepSeek 67B（我们之前的版本）中使用的语料库（DeepSeek-AI，2024）相比，该语料库的数据量更大，尤其是中文数据，数据质量更高。我们首先在完整的预训练语料库上对 DeepSeek-V2 进行预训练。然后，我们收集了 1.5M 个对话会话，涵盖数学、代码、写作、推理、安全等各个领域，以对 DeepSeek-V2 Chat（SFT）进行监督微调（SFT）。最后，我们遵循 DeepSeekMath（Shao 等人，2024），采用组相对策略优化（GRPO）进一步使模型与人类偏好保持一致，并生成 DeepSeek-V2 Chat（RL）。


We evaluate DeepSeek-V2 on a wide range of benchmarks in English and Chinese, and compare it with representative open-source models. Evaluation results show that even with only 21B activated parameters, DeepSeek-V2 still achieves top-tier performance among open-source models and becomes the strongest open-source MoE language model. Figure 1(a) highlights that, on MMLU, DeepSeek-V2 achieves top-ranking performance with only a small number of activated parameters. In addition, as shown in Figure 1(b), compared with DeepSeek 67B, DeepSeek-V2 saves 42.5% of training costs, reduces the KV cache by 93.3%, and boosts the maximum generation throughput to 5.76 times. We also evaluate DeepSeek-V2 Chat (SFT) and DeepSeek-V2 Chat (RL) on open-ended benchmarks. Notably, DeepSeek-V2 Chat (RL) achieves 38.9 length-controlled win rate on AlpacaEval 2.0 (Dubois et al., 2024), 8.97 overall score on MT-Bench (Zheng et al., 2023), and 7.91 overall score on AlignBench (Liu et al., 2023). The English open-ended conversation evaluations demonstrate that DeepSeek-V2 Chat (RL) has toptier performance among open-source chat models. In addition, the evaluation on AlignBench indicates that in Chinese, DeepSeek-V2 Chat (RL) outperforms all of open-source models, and even beats most of closed-source models.

我们对英文和中文的大量 Benchmark 测试集进行了 DeepSeek-V2 评估，并与具有代表性的开源模型进行了比较。评估结果表明，即便在仅有 21B 激活参数的情况下，DeepSeek-V2 依然取得了开源模型中的顶级性能，成为最强的开源 MoE 语言模型。图 1(a) 表明，在 MMLU 上，DeepSeek-V2 仅使用少量激活参数便取得了顶级性能。此外，如图 1(b) 所示，与 DeepSeek 的 67B 相比，DeepSeek-V2 节省了 42.5% 的训练成本，减少了 93.3% 的 KV 缓存，并将最大生成吞吐量提升至 5.76 倍。我们还在开放式 Benchmark 测试集中评估了 DeepSeek-V2 Chat (SFT) 和 DeepSeek-V2 Chat (RL)。值得注意的是，DeepSeek-V2 Chat (RL) 在 AlpacaEval 2.0（Dubois 等，2024）上实现了 38.9 的长度控制胜率，在 MT-Bench（Zheng 等，2023）上实现了 8.97 的总分，在 AlignBench（Liu 等，2023）上实现了 7.91 的总分。英文开放式对话评测表明，DeepSeek-V2 Chat (RL) 在开源聊天模型中拥有顶级表现。此外，AlignBench 的评测表明，在中文中，DeepSeek-V2 Chat (RL) 的表现优于所有开源模型，甚至超越了大多数闭源模型。

In order to facilitate further research and development on MLA and DeepSeekMoE, we also release DeepSeek-V2-Lite, a smaller model equipped with MLA and DeepSeekMoE, for the open-source community. It has a total of 15.7B parameters, where 2.4B are activated for each token. Detailed descriptions about DeepSeek-V2-Lite can be found in Appendix B.

为了促进对 MLA 和 DeepSeekMoE 的进一步研究和开发，我们还向开源社区发布了 DeepSeek-V2-Lite，这是一个配备 MLA 和 DeepSeekMoE 的较小模型。它总共有 15.7B 个参数，其中每个 token 激活 2.4B。有关 DeepSeek-V2-Lite 的详细描述可在附录 B 中找到。

In the rest of this paper, we first provide a detailed description of the model architecture of DeepSeek-V2 (Section 2). Subsequently, we introduce our pre-training endeavors, including the training data construction, hyper-parameter settings, infrastructures, long context extension, and the evaluation of model performance and efficiency (Section 3). Following this, we demonstrate our efforts in alignment, encompassing Supervised Fine-Tuning (SFT), Reinforcement Learning (RL), the evaluation results, and other discussion (Section 4). Finally, we summarize the conclusion, deliberate on the current limitations of DeepSeek-V2, and outline our future work (Section 5).

在本文的其余部分，我们首先详细描述了 DeepSeek-V2 的模型架构（第 2 节）。随后，我们介绍了我们的预训练工作，包括训练数据构建、超参数设置、基础设施、长上下文扩展以及模型性能和效率的评估（第 3 节）。随后，我们展示了我们在协调方面的努力，包括监督微调 (SFT)、强化学习 (RL)、评估结果和其他讨论（第 4 节）。最后，我们总结结论，讨论 DeepSeek-V2 当前的局限性，并概述我们未来的工作（第 5 节）。

# 2. Architecture

By and large, DeepSeek-V2 is still in the Transformer architecture (Vaswani et al., 2017), where each Transformer block consists of an attention module and a Feed-Forward Network (FFN). However, for both the attention module and the FFN, we design and employ innovative architectures. For attention, we design MLA, which utilizes low-rank key-value joint compression to eliminate the bottleneck of inference-time key-value cache, thus supporting efficient inference. For FFNs, we adopt the DeepSeekMoE architecture (Dai et al., 2024), a high-performance MoE architecture that enables training strong models at an economical cost. An illustration of the architecture of DeepSeek-V2 is presented in Figure 2, and we will introduce the details of MLA and DeepSeekMoE in this section. For other tiny details (e.g., layer normalization and the activation function in FFNs), unless specifically stated, DeepSeek-V2 follows the settings of DeepSeek 67B (DeepSeek-AI, 2024).

总体而言，DeepSeek-V2 仍采用 Transformer 架构（Vaswani et al., 2017），其中每个 Transformer 块由一个注意模块和一个前馈网络 (FFN) 组成。然而，对于注意模块和 FFN，我们都设计并采用了创新架构。对于注意，我们设计了 MLA，它利用低秩键值联合压缩来消除推理时间键值缓存的瓶颈，从而支持高效推理。对于 FFN，我们采用 DeepSeekMoE 架构（Dai et al., 2024），这是一种高性能的 MoE 架构，能够以经济的成本训练出强大的模型。图 2 展示了 DeepSeek-V2 的架构，我们将在本节中介绍 MLA 和 DeepSeekMoE 的细节。对于其他微小的细节（例如，层规范化和 FFN 中的激活函数），除非特别说明，DeepSeek-V2 遵循 DeepSeek 67B（DeepSeek-AI，2024）的设置。

## 2.1 Multi-Head Latent Attention: Boosting Inference Efficiency

Conventional Transformer models usually adopts Multi-Head Attention (MHA) (Vaswani et al., 2017), but during generation, its heavy Key-Value (KV) cache will become the bottleneck that limit the inference efficiency. In order to reduce the KV cache, Multi-Query Attention (MQA) (Shazeer, 2019) and Grouped-Query Attention (GQA) (Ainslie et al., 2023) are proposed. They require a smaller magnitude of KV cache, but their performance does not match MHA (we provide the ablation of MHA, GQA and MQA in Appendix D.1). 

传统的Transformer模型通常采用多头注意力机制（MHA）（Vaswani等，2017），但在生成过程中，其繁重的键值（KV）缓存将成为限制推理效率的瓶颈。为了减少KV缓存，提出了多查询注意力机制（MQA）（Shazeer，2019）和分组查询注意力机制（GQA）（Ainslie等，2023）。它们需要的KV缓存量级较小，但性能不如MHA（我们在附录D.1中提供了MHA，GQA和MQA的消融）。

For DeepSeek-V2, we design an innovative attention mechanism called Multi-head Latent Attention (MLA). Equipped with low-rank key-value joint compression, MLA achieves better performance than MHA, but requires a significantly smaller amount of KV cache. We introduce its architecture in the following, and also provide a comparison between MLA and MHA in Appendix D.2.

对于DeepSeek-V2，我们设计了一种创新的注意力机制，称为多头潜在注意力机制（MLA）。MLA配备了低秩键值联合压缩，性能优于MHA，但需要的KV缓存量明显较少。我们在下面介绍它的架构，并在附录 D.2 中提供 MLA 和 MHA 的比较。

  ![image](https://github.com/user-attachments/assets/34f86e60-bf0b-4ba9-b053-08565f79f86d)


### 2.1.1 Preliminaries: Standard Multi-Head Attention

![image](https://github.com/user-attachments/assets/3618e0c7-c211-4b04-af34-1d3b9780f9dd)

We first introduce the standard MHA mechanism as background. Let 𝑑 be the embedding dimension, 𝑛ℎ be the number of attention heads, 𝑑ℎ be the dimension per head, and h𝑡 ∈ R𝑑 be the attention input of the 𝑡-th token at an attention layer. Standard MHA first produces q𝑡 , k𝑡 , v𝑡 ∈ R𝑑ℎ𝑛ℎ through three matrices 𝑊𝑄 ,𝑊𝐾 ,𝑊𝑉 ∈ R𝑑ℎ𝑛ℎ×𝑑 , respectively:

我们首先介绍标准的 MHA 机制作为背景。令 𝑑 为 embedding 维度，𝑛ℎ 为注意力头的数量，𝑑ℎ 为每个注意力头的维度，h𝑡 ∈ R𝑑 为注意力层上第 𝑡 个 token 的注意力输入。标准 MHA 首先通过三个矩阵 𝑊𝑄 ,𝑊𝐾 ,𝑊𝑉 ∈ R𝑑ℎ𝑛ℎ×𝑑 分别生成 q𝑡 , k𝑡 , v𝑡 ∈ R𝑑ℎ𝑛ℎ：

![image](https://github.com/user-attachments/assets/5bef2adf-d6ad-4e68-90ce-9ba4e24ad9d8)


Then, q𝑡 , k𝑡 , v𝑡 will be sliced into 𝑛ℎ heads for the multi-head attention computation
然后，q𝑡、k𝑡、v𝑡 将被切分为 𝑛ℎ 个头，以进行多头注意力计算

 ![image](https://github.com/user-attachments/assets/cc47099e-d3ee-447f-9f55-081b57349750)

 ![image](https://github.com/user-attachments/assets/cb1a5601-4e50-4337-955c-ba378b96848b)



where q𝑡,𝑖 , k𝑡,𝑖 , v𝑡,𝑖 ∈ R𝑑ℎ denote the query, key, and value of the 𝑖-th attention head, respectively; 𝑊𝑂 ∈ R𝑑×𝑑ℎ𝑛ℎ denotes the output projection matrix. During inference, all keys and values need to be cached to accelerate inference, so MHA needs to cache 2𝑛ℎ𝑑ℎ𝑙 elements for each token. In model deployment, this heavy KV cache is a large bottleneck that limits the maximum batch size and sequence length.
其中 q𝑡,𝑖 、k𝑡,𝑖 、v𝑡,𝑖 ∈ R𝑑ℎ 分别表示第 𝑖 个注意力头的 query、key 和 value；𝑊𝑂 ∈ R𝑑×𝑑ℎ𝑛ℎ 表示输出投影矩阵。在推理过程中，需要缓存所有 key 和 value 以加速推理，因此 MHA 需要为每个 token 缓存 2𝑛ℎ𝑑ℎ𝑙 个元素。在模型部署中，这种繁重的 KV 缓存是限制最大批次大小和序列长度的一大瓶颈。

### 2.1.2 Low-Rank Key-Value Joint Compression

The core of MLA is the low-rank joint compression for keys and values to reduce KV cache:
MLA的核心是对key和value进行低秩联合压缩，以减少KV缓存：

![image](https://github.com/user-attachments/assets/cce59a0c-0528-4310-963d-c3498aa7fa43)

where c 𝐾𝑉 𝑡 ∈ R𝑑𝑐 is the compressed latent vector for keys and values; 𝑑𝑐(≪ 𝑑ℎ𝑛ℎ) denotes the KV compression dimension; 𝑊𝐷𝐾𝑉 ∈ R𝑑𝑐×𝑑 is the down-projection matrix; and 𝑊𝑈𝐾 ,𝑊𝑈𝑉 ∈ R𝑑ℎ𝑛ℎ×𝑑𝑐 are the up-projection matrices for keys and values, respectively. During inference, MLA only needs to cache c 𝐾𝑉 𝑡 , so its KV cache has only 𝑑𝑐 𝑙 elements, where 𝑙 denotes the number of layers. In addition, during inference, since 𝑊𝑈𝐾 can be absorbed into 𝑊𝑄 , and 𝑊𝑈𝑉 can be absorbed into 𝑊𝑂 , we even do not need to compute keys and values out for attention. Figure 3 intuitively illustrates how the KV joint compression in MLA reduces the KV cache.
其中 c 𝐾𝑉 𝑡 ∈ R𝑑𝑐 是键和值的压缩潜在向量；𝑑𝑐(≪ 𝑑ℎ𝑛ℎ) 表示 KV 压缩维度；𝑊𝐷𝐾𝑉 ∈ R𝑑𝑐×𝑑 是下投影矩阵；𝑊𝑈𝐾 ,𝑊𝑈𝑉 ∈ R𝑑ℎ𝑛ℎ×𝑑𝑐 分别是键和值的上投影矩阵。在推理过程中，MLA 只需要缓存 c 个 𝐾𝑉 𝑡 ，因此其 KV 缓存只有 𝑑𝑐 𝑙 个元素，其中 𝑙 表示层数。此外，在推理过程中，由于 𝑊𝑈𝐾 可以被吸收到 𝑊𝑄 中， 𝑊𝑈𝑉 可以被吸收到 𝑊𝑂 中，我们甚至不需要计算出用于注意的键和值。图 3 直观地说明了 MLA 中的 KV 联合压缩如何减少 KV 缓存。
Moreover, in order to reduce the activation memory during training, we also perform low-rank compression for the queries, even if it cannot reduce the KV cache:

此外，为了减少训练期间的激活内存，我们还对查询执行低秩压缩，即使它不能减少 KV 缓存：

![image](https://github.com/user-attachments/assets/4a922368-0518-4ac4-85b9-8c8fcd5cb4bc)

![image](https://github.com/user-attachments/assets/b1336455-d1dd-4263-886f-cb59204b04b0)


where c 𝑄 𝑡 ∈ R𝑑 ′ 𝑐 is the compressed latent vector for queries; 𝑑 ′ 𝑐 (≪ 𝑑ℎ𝑛ℎ) denotes the query compression dimension; and 𝑊𝐷𝑄 ∈ R𝑑 ′ 𝑐×𝑑 ,𝑊𝑈𝑄 ∈ R𝑑ℎ𝑛ℎ×𝑑 ′ 𝑐 are the down-projection and upprojection matrices for queries, respectively.

其中 c 𝑄 𝑡 ∈ R𝑑 ′ 𝑐 是查询的压缩潜在向量；𝑑 ′ 𝑐 (≪ 𝑑ℎ𝑛ℎ) 表示查询压缩维度；𝑊𝐷𝑄 ∈ R𝑑 ′ 𝑐×𝑑 ,𝑊𝑈𝑄 ∈ R𝑑ℎ𝑛ℎ×𝑑 ′ 𝑐 分别是查询的下投影和上投影矩阵。

### 2.1.3 Decoupled Rotary Position Embedding

![image](https://github.com/user-attachments/assets/23b6ca96-a20e-479a-9c77-6f8bdf7463be)


Following DeepSeek 67B (DeepSeek-AI, 2024), we intend to use the Rotary Position Embedding (RoPE) (Su et al., 2024) for DeepSeek-V2. However, RoPE is incompatible with low-rank KV compression. To be specific, RoPE is position-sensitive for both keys and queries. If we apply RoPE for the keys k 𝐶 𝑡 , 𝑊𝑈𝐾 in Equation 10 will be coupled with a position-sensitive RoPE matrix. In this way, 𝑊𝑈𝐾 cannot be absorbed into 𝑊𝑄 any more during inference, since a RoPE matrix related to the currently generating token will lie between 𝑊𝑄 and 𝑊𝑈𝐾 and matrix multiplication does not obey a commutative law. As a result, we must recompute the keys for all the prefix tokens during inference, which will significantly hinder the inference efficiency.

继 DeepSeek 67B（DeepSeek-AI，2024 年）之后，我们打算在 DeepSeek-V2 中使用旋转位置嵌入 (RoPE)（Su 等人，2024 年）。但是，RoPE 与低秩 KV 压缩不兼容。具体来说，RoPE 对键和查询都是位置敏感的。如果我们将 RoPE 用于键 k 𝐶 𝑡 ，则等式 10 中的 𝑊𝑈𝐾 将与位置敏感的 RoPE 矩阵耦合。这样，𝑊𝑈𝐾 在推理过程中就无法再被吸收到 𝑊𝑄 中，因为与当前生成的 token 相关的 RoPE 矩阵将位于 𝑊𝑄 和 𝑊𝑈𝐾 之间，并且矩阵乘法不遵循交换律。因此，我们必须在推理过程中重新计算所有前缀标记的键，这将严重阻碍推理效率。

![image](https://github.com/user-attachments/assets/80eb2e7b-b161-4976-aea5-1f4dcaa0458b)


As a solution, we propose the decoupled RoPE strategy that uses additional multi-head queries q 𝑅 𝑡,𝑖 ∈ R𝑑 𝑅 ℎ and a shared key k 𝑅 𝑡 ∈ R𝑑 𝑅 ℎ to carry RoPE, where 𝑑 𝑅 ℎ denotes the per-head dimension of the decoupled queries and key. Equipped with the decoupled RoPE strategy, MLA performs the following computation:

作为解决方案，我们提出了解耦 RoPE 策略，该策略使用额外的多头查询 q 𝑅 𝑡,𝑖 ∈ R𝑑 𝑅 ℎ 和共享密钥 k 𝑅 𝑡 ∈ R𝑑 𝑅 ℎ 来承载 RoPE，其中 𝑑 𝑅 ℎ 表示解耦查询和密钥的每个头维度。配备解耦 RoPE 策略后，MLA 可执行以下计算：

![image](https://github.com/user-attachments/assets/4d62333c-00b9-4871-b0c2-b47025da5e23)

![image](https://github.com/user-attachments/assets/5757286b-bc83-4040-893b-73236694bc3e)

where 𝑊𝑄𝑅 ∈ R𝑑 𝑅 ℎ 𝑛ℎ×𝑑 ′ 𝑐 and 𝑊𝐾𝑅 ∈ R𝑑 𝑅 ℎ ×𝑑 are matrices to produce the decouples queries and key, respectively; RoPE(·) denotes the operation that applies RoPE matrices; and [·; ·] denotes the concatenation operation. During inference, the decoupled key should also be cached. Therefore, DeepSeek-V2 requires a total KV cache containing (𝑑𝑐 + 𝑑 𝑅 ℎ )𝑙 elements. 

其中 𝑊𝑄𝑅 ∈ R𝑑 𝑅 ℎ 𝑛ℎ×𝑑 ′ 𝑐 和 𝑊𝐾𝑅 ∈ R𝑑 𝑅 ℎ ×𝑑 分别是生成解耦查询和键的矩阵；RoPE(·) 表示应用 RoPE 矩阵的操作；[·; ·] 表示连接操作。在推理过程中，解耦的键也应该被缓存。因此，DeepSeek-V2 需要一个包含 (𝑑𝑐 + 𝑑 𝑅 ℎ )𝑙 元素的总 KV 缓存。

In order to demonstrate the complete computation process of MLA, we also organize and provide its full formulas in Appendix C.

为了展示 MLA 的完整计算过程，我们还在附录 C 中整理并提供了其完整公式。

###2.1.4. Comparison of Key-Value Cache
  
We demonstrate a comparison of the KV cache per token among different attention mechanisms in Table 1. MLA requires only a small amount of KV cache, equal to GQA with only 2.25 groups, but can achieve stronger performance than MHA.

我们在表 1 中展示了不同注意力机制中每个 token 的 KV 缓存的比较。MLA 只需要少量的 KV 缓存，相当于只有 2.25 个组的 GQA，但可以获得比 MHA 更强的性能。

![image](https://github.com/user-attachments/assets/8f0f1b30-8ca2-48ab-821b-02fb406b2484)


> Table1 不同注意力机制每个 token 的 KV 缓存对比。𝑛ℎ 表示注意力头数量，𝑑ℎ 表示每个注意力头的维度，𝑙 表示层数，𝑛𝑔 表示 GQA 中的组数，𝑑𝑐 和 𝑑 𝑅 ℎ 分别表示 MLA 中解耦查询和键的 KV 压缩维度和每个头的维度。KV 缓存量以元素数量衡量，与存储精度无关。对于 DeepSeek-V2，𝑑𝑐 设置为 4𝑑ℎ，𝑑 𝑅 ℎ 设置为 𝑑ℎ 2 。因此，它的 KV 缓存与只有 2.25 个组的 GQA 相同，但其性能强于 MHA
