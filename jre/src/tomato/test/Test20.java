package tomato.test;

public class Test20 {
    public int test(){
        int[][] arr = new int[200][200];
        for(int i = 0; i < arr.length ; i++){
            for(int j = 0; j < arr[i].length;j ++){
                arr[i][j] = j;
            }
        }
        return arr[99][99] == 99 ? Result.SUCCESS : Result.FAILED ;
    }

    public static void main(String[] args) {
        //long t1 =  System.nanoTime();
        Test20 test20 = new Test20();
        test20.test();
        //System.out.println(System.nanoTime() - t1);
    }

}
