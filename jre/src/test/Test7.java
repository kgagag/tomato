package test;

public class Test7 {
    int a = 0;

    public void add(){
       a = 1;
    }
    public int test(){
        add();
        if( a== 1){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }
}
