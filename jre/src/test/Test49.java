package test;

public class Test49 extends Test48 {
    public int test(){
        Test49 test49 = new Test49();
        test49.id = 100;
        if(test49.id == 100 && Test48.id == 100){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }
    public static void main(String[] args) {

    }
}

