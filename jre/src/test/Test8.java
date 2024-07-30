package test;

public class Test8 {

    private int add(int a,int b){
        return  a + b;
    }

    public int test(){
        int a = 0;
        for(int i = 0;i < 1000;i++){
            a = add(i, i + 1);
        }
       // System.out.println(a);
        if( a== 1999){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }

    public static void main(String[] args) {
        Test8 test8 = new Test8();
        System.out.println(test8.test());
    }

}
