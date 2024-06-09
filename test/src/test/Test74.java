package test;

public class Test74 {
    public int test() {
        Test74 t = new Test74();
        if (t instanceof Test74) {
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }
}
