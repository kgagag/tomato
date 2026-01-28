package tomato.test;

public class Test92 {
    public int test(){
        long t1 = Long.MAX_VALUE;
        long t2 = 10L;
        long t3 = t1 << t2;
        return t3 == -1024 ? Result.SUCCESS : Result.FAILED;
    }

    public static void main(String[] args) {
        Test92 test92 = new Test92();
        test92.test();
    }
}
