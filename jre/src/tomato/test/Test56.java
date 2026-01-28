package tomato.test;

public class Test56 {
    public int test(){
        float f = Float.valueOf(0.1f);
//        double d = Double.valueOf(0.1d);
//        int i = Integer.valueOf(1);
        return f == 0.1f  ? Result.SUCCESS : Result.FAILED;
    }
}
