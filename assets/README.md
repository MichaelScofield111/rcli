## textsign

文本hash

如何保证文件没有更改

1. 先对文件进行一个hash，用私钥对这个hash进行一个签名
2. 用公钥解密签名拿到hash，在和文本用同样的算法算出一个hash对比

构建CLI对输入文本哈希/签名


## HTTP Server

构建 CLI：
rcli http server -d .  => http server 就可以保存当前目录的文件
