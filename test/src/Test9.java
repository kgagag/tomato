public class Test8 {

    private int add(int a,int b){
        return  a + b;
    }

    public int test(){
        int a = 0;
        for(int i = 0;i < 100;i++){
            a = add(i, i + 1);
        }
        if( a== 199){
            return 20240325;
        }
        return 20240324;
    }

    public static void main(String[] args) {
        Test8 test8 = new Test8();
        System.out.println(test8.test());
    }

}
