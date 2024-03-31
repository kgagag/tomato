public class Test2 {
    public float add(float a , float b){
        return a + b;
    }

    public int test(){
        float a = 1000.001f;
        float b = 666f;
        if(add(a, b) == 1666.001f){
            return 20240325;
        }
        return 20240324;
    }
}
