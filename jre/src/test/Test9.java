package test;

public class Test9 {

    public int add_1(int a,int b){
        return  a + b;
    }
    public int add_2(int a,int b){
        return  add_1(a,b);
    }
    public int add_3(int a,int b){
        return  add_2(a,b);
    }

    public int add_4(int a,int b){
        return  add_3(a,b);
    }

    public int add_5(int a,int b){
        return  add_4(a,b);
    }

    public int add_6(int a,int b){
        return  add_5(a,b);
    }

    public int add_7(int a,int b){
        return  add_6(a,b);
    }

    public int test(){
        int a = 0;
        for(int i = 0;i < 100;i++){
            a = add_3(i, i + 1);
        }
        if( a== 199){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }

    public static void main(String[] args) {
        Test9 test8 = new Test9();
        test8.test();
    }

}
