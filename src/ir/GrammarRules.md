IR/Bytecode Grammar Rules:
===
Here is the IR/Bytecode, which is based on a main Stack.[^note]
- use `<LABEL>:` to mark a part of program. (until next \<label\> start)
- use `#` to mark contents behind this character as a comment.
- use `PUSH <TYPE> <VALUE>` to push a value to the stack. Type is required. [^types]
- use `ADD` / `SUB` / `MUL` / `DIV`  to execuate basic calcuations, which consume two items from the top of the stack and push an result item into the stack.
- use `GT` / `GE` / `LT` / `LE` to do the compare.[^comparation]
- use `POP` to remove the item from the top of the stack.
- use `CALL <INNER-FUNCTION>` to call a internal `FUNCTION` ,which is built-in. [^functions]
- use `JUMP <LABEL>` to execuate a label directly. If you don't use `JUMP` to come back, it will not come back automatically.
- use `JUMPIF <LABEL>` to go to a label when the value of the item located in the top of the stack is true.
- use `INC` to increase one the item in the top of the stack. (+1)
- use `DEC` to  decrease one the item in the top of the stack.
- use `CLEAR` to remove all items from the stack.
- use `LOAD <SLOT>` to load a variable to the top of the stack.
- use `STORE <SLOT>` to save the top of the stack into the slot.

### For example
```ir/bytecode
MAIN:
    PUSH INT 5  #[5]
    PUSH INT 7  #[5,7]
    LT          #[1]
    JUMPIF true_label
    JUMP finally
finally:
    PUSH CHAR 65        # In ascii code, integer 65 can stand for letter 'A'
    CALL ECHO           # built-in function ECHO consume none.
    # function ECHO only echo the top of the stack, and not echo '\n'
true_label:
    CALL ECHO
    JUMP finally
```

It can be translated into C like this:
```c
#include <stdio.h>
#include <stdbool.h>
int a = 5;
int b = 7;
bool result = a < b;
if (result) {
    // true_label
    printf("%d", result);
}
// finally label
char c = 'A';
printf("%c", c);

```
[If you want to know more about the keywords, please go here](GrammarRules/)


[^note]: In fact, the stack is global. The operations in anthor label may impact the current.
[^types]: Only INT/CHAR/FLOAT are supported.
[^comparation]: They will consume two items and push the result (true: 1, false: 0) to the top of the stack.
[^functions]: Internal Functions, for example, `ECHO`, `READ` and so on...