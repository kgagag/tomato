public class Test1 {
    public int add(int a , int b){
        return a + b;
    }

    public int test(){
        int a = 1000;
        int b = 666;
        if(add(a, b) == 1666){
            return 20240325;
        }
        return 20240324;
        // return add(a, b);
    }
}
