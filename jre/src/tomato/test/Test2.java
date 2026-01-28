package tomato.test;

public class Test2 {
    public float add(float a , float b){
        return a + b;
    }

    public int test(){
        float a = 1000.001f;
        float b = 666f;
        if(add(a, b) == 1666.001f){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }
}
