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

    public static void main(String[] args) {
        long t1 = System.nanoTime();
        Test1 test1 = new Test1();
        test1.test();
        System.out.println(System.nanoTime() - t1);
    }
}
