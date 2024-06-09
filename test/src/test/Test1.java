package test;

public class Test1 {
    public int add(int a , int b){
        return a + b;
    }

    public int test(){
        int a = 1000;
        int b = 666;
        if(add(a, b) == 1666){
            return Result.SUCCESS;
        }
        return Result.FAILED;
        // return add(a, b);
    }
}
