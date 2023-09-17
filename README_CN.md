[English](README.md) | 简体中文

#### 一、rust中运行ai

这是一个在rust中部署AI模型的研究。主要实现如下目的

1、无需python环境  
2、静态二进制文本部署  
3、支持多线程的API   
4、后台任务等   

此外还研究了其他事项  
1、docker部署交叉编译问题  
2、git的hooks的CI流程

```shell
mkdir hooks
git config 'core.hooksPath' hooks
```

还有如下问题需要解决  
1、Rust 编译静态单体执行文件    
2、clap 解析命令行参数  
3、api文档问题  
4、实用模型部署问题  
5、tokenizer使用  
6、图像读取问题

更多进阶问题：model推理的三大工具  
1、candle的使用  
2、burn的使用  
3、tract_onnx的使用


torch.fx量化 、pytorch转ONNX技巧



