public class Test6 {
    int a = 0;

    public void add(int i){
        if(i >= 100){
            return;
        }
        a += i;
        add( i + 1);
    }
    public int test(){
        add(0);
        if( a== 4950){
            return 20240325;
        }
        return 20240324;
    }
}
