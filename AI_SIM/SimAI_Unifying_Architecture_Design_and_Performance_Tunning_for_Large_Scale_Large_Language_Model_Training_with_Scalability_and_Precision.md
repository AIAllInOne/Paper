https://ennanzhai.github.io/pub/nsdi25spring-simai.pdf

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
