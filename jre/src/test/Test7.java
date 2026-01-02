package test;

public class Test7 {
    int a = 1;

    public void add(){
       //a = 1;
    }
    public int test(){
        add();
        if( a== 1){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }

    public static void main(String[] args) {
        Test7 test7 = new Test7();
        test7.test();
    }
}
