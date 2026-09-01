# PUSH Command
<br/>
We use it to push a constant value to the top of the operation stack .<BR>
For example :

```ir/bytecode
PUSH INT 5
```
It will push a integer 5 to the stack.
Thus, now the stack is \[5] now.<br/>

Usually, we use it to set some temp constant value / literal value.

If you want to use variables, please use `LOAD` Command.