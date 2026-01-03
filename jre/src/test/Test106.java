package test;

public class Test106 {
    public int test(){
      int[][] array = new int[3][5];
      array[2][2] = 1;
        if(array[2][2] == 1){
            return Result.SUCCESS;
        }else {
            return Result.FAILED;
        }
    }

    public static void main(String[] args) {
        Test106 test106 = new Test106();
        test106.test();
    }
}
