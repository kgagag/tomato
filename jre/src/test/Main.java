package test;

import java.util.*;
public class Main{
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        int m = sc.nextInt();
        int[] nums1 = new int[n];
        int[] nums2 = new int[m];
        for(int i = 0 ; i < n; i ++){
            nums1[i] = sc.nextInt();
        }
        for(int i = 0 ; i < m; i ++){
            nums2[i] = sc.nextInt();
        }

        int l = 0, r = 0;
        while (l < nums1.length  && r < nums2.length ){
            if(nums1[l] == nums2[r]){
                l ++;
            }
        }

        if(l == nums1.length){
            System.out.println("Yes");
        }else {
            System.out.println("No");
        }
    }
}