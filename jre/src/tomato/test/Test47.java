package tomato.test;

public class Test47 {
    public int test(){
        int i = 2 << 10;
        i >>= 4;
        return i == 128 ? Result.SUCCESS : Result.FAILED;
    }

    public static void main(String[] args) {
        Test47 test47 = new Test47();
        System.out.println(test47.test());
    }
}
