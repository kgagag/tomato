public class Test5 {
    public int add(){
        int a = 0;
        int i = 0;
        while(i < 100){
            a += i;
            i++;
        }
        return a;
    }
    public int test(){
        if(add() == 4950){
            return 20240325;
        }
        return 20240324;
    }
}
