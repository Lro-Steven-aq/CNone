## 我想要写一个 Compiler.
现已实现了简单的词法分析器（我 \*\*的 C 语言真标准多且灵活，算了，还不如自己设计一个语言。）<br/>
（我称这个compiler为cnone就是因为这个原因的）
我把注释处理放到了预处理（尽管我还没有开始写）

### 如何运行
1. 安装[Rust](https://rust-lang.org/)
2. 然后在目录执行
   ```shell
    cargo run --release
   ```