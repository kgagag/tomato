package test;

public class MyTestUnit {
    public static boolean assertEquals(int[] arr,int[] brr){
        if(arr.length != brr.length){
            return false;
        }
        for(int i = 0; i < arr.length; i ++){
            if(arr[i] != brr[i]){
                return false;
            }
        }
        return true;
    }
}
