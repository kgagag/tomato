package test;

import java.util.Arrays;

class Solution {
    public int minSetSize(int[] arr) {
        Integer[] arr2 = new Integer[100000];
        for(int i = 0 ; i < arr2.length;i++){
            if(arr2[i] == null){
                arr2[i] = 0;
            }
        }
        for(int i = 0 ; i < arr.length;i++){
            arr2[arr[i]] ++;
        }
        Arrays.sort(arr2,(a, b) -> b - a );
        int ans = 0;
        int k = 0;
        for(int i = 0; i < arr2.length;i++){
            k += arr2[i];
            ans ++;
            if(k >= arr.length / 2){
                return ans;
            }
        }
        return ans;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        solution.minSetSize(new int[]{3,3,3,3,5,5,5,2,2,7});
    }
}
