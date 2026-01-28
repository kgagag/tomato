package tomato.test;

public class Test107 extends AbstractTest2{

    public int test(){
        Float f = 100f;
        int a = 100;
        Float c = a + f;
        return c == 200 ? Result.SUCCESS : Result.FAILED;
    }
    public static void main(String[] args) {
        Test107 test107 = new Test107();
        //System.out.println(test107.tomato.test());
        test107.test();
    }
}
