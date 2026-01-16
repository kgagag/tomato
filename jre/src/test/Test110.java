package test;

public class Test110 extends Test109{

    public int test(){
        Test110 test110 = new Test110();
        test110.value = 10;
        Test108.value = 20;
        if (Test110.value == 20 && Test108.value == 20){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    };
    public static void main(String[] args) {
        Test110 test110 = new Test110();
        //System.out.println(test110.test());
        test110.test();
    }
}
