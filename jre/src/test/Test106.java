package test;

public class Test106 {
    public int test(){
      int[][] array = new int[10][8];
      array[4][5] = 1;
//        if(array[4][5] == 1){
//            return Result.SUCCESS;
//        }else {
//            return Result.FAILED;
//        }
        return 0;
    }

    public static void main(String[] args) {
        Test106 test106 = new Test106();
        test106.test();
    }
}
