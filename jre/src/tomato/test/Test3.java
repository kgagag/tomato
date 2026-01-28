package tomato.test;

public class Test3 {
    public double add(double a , double b){
        double c = a + b;
        return a + b;

    }

    public int test(){
        double a = 10000000000000000000000000000000000000000000000000000000000000000000000.001d;
        float b = 666f;
        if(add(a, b) == 10000000000000000000000000000000000000000000000000000000000000000000666.001d){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }

}
