package test;

public class Test4 {
    public int add(){
        int a = 0;
        for(int i =0; i < 100;i++){
            a += i;
        }
        return a;
    }
    public int test(){
        if(add() == 4950){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }
}
