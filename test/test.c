/**
 * 现在是一个非法的程序段，但是它依旧可以通过词法分析。
 * 词法分析阶段不会报错，但是应该在语法、语义分析时报错。
 */
#define a 9
void fff();
int main(int, char**) {
        int *kode = 9;/*  cc=[]  */
        float num = 0.9**kode; //num=8.1
        return 0; // this means no errors.
}