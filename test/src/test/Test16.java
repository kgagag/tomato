package test;

public class Test16 {
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
//        int[][] arr = new int[2][3];
//        arr[1][2] = 100;
        System.out.println(new Test6().test());
    }
}
