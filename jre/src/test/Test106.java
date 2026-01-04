package test;

public class Test106 {
    public int test(){
        int[][][] arr = new int[10][10][10];
        int a = 0;
        for(int i = 0 ;i <arr.length;i++){
            for(int j = 0 ;j < arr[i].length;j++){
                for(int k = 0 ;k < arr[i][j].length;k++){
                    arr[i][j][k] = a ++;
                }
            }
        }
        return arr[9][9][9] == 999 ? Result.SUCCESS : Result.FAILED ;
    }

    public static void main(String[] args) {
        Test106 test106 = new Test106();
        test106.test();
    }
}
