## 我想要写一个 C Compiler.
现已实现了简单的词法分析器（然而，我并不想实现所有的C 标准的东西。因为懒或者麻烦）<br/>
（我称这个compiler为cnone就是因为这个原因的）
我把注释处理放到了预处理（尽管我还没有开始写）

### 如何运行
1. 安装[Rust](https://rust-lang.org/)
2. 然后在目录执行
   ```shell
    cargo run--release
   ```
   在 . 目录下，有 .test.c .test2.c 文件，其包含了一定量的C代码，其可以通过这个词法分析。