package test;

public class TryCatchTest01 {
    public int test() {
        int a = 100 ;
        try {
            int i = 1 / 0;
        } catch (Exception e) {
            a = 200;
        }
        if (a == 200) {
           return Result.SUCCESS;
        }
        return Result.FAILED;
    }
    public static void main(String[] args) {
        TryCatchTest01 t = new TryCatchTest01();
        t.test();
    }
}
