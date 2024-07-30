package test;

public class Test20 {
    public int test(){
        int[][] arr = new int[100][100];
        for(int i = 0; i < arr.length ; i++){
            for(int j = 0; j < arr[i].length;j ++){
                arr[i][j] = j;
            }
        }
        return arr[99][99] == 99 ? Result.SUCCESS : Result.FAILED ;
    }

}
