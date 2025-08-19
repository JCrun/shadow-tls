# 改进的 ShadowTLS v3 实现

此分支包含了对 ShadowTLS v3 协议的改进实现，主要目的是增强其抗检测能力。

## 主要改进

1. **HMAC 嵌入实现**: 修改了原有在 TLS 握手消息之后添加 HMAC 的方式，改为将 HMAC
   嵌入到现有消息中，以避免改变消息长度，提高抗检测能力。

2. **添加伪造 NewSessionTicket**: 在握手完成后添加了伪造的 NewSessionTicket
   消息，更好地模拟真实 TLS 流量特征。

## 技术细节

原始的 ShadowTLS v3 实现通过在每个 TLS 握手消息之后添加
HMAC，导致消息长度增加，可能被检测。改进版本：

1. 对于服务器端:
   - 修改了 `copy_by_frame_with_modification` 函数，将 HMAC
     嵌入到消息中而不是附加到消息之后
   - 实现伪造 NewSessionTicket 消息的发送

2. 对于客户端:
   - 修改了 HMAC 验证逻辑，从消息中提取嵌入的 HMAC
   - 处理伪造的 NewSessionTicket 消息

## 使用方法

与原始 ShadowTLS
的使用方法相同，无需修改配置文件或命令行参数。改进对用户完全透明。

## 测试状态

目前，此改进版本在 Linux 环境中通过了 V2 和 V3 协议测试，但在 Windows
平台上可能存在兼容性问题。建议在 Linux 环境中使用和测试。

## 性能影响

此改进不会对性能产生明显影响，但可能会略微增加握手过程的计算量。对于实际使用场景，这些影响是微不足道的。

## 贡献与反馈

如果您在使用过程中发现任何问题或有任何建议，欢迎提交 Issue 或 Pull Request。
