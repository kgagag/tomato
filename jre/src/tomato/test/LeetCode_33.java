package tomato.test;

public class LeetCode_33 {
    public int  test(){
         int a = search(new int[]{4,5,6,7,0,1,2},0);
         return a == 4 ? Result.SUCCESS : Result.FAILED;
    }
    public int search(int[] nums, int target) {
        int left = 0;
        int right = nums.length -1;
        if(nums.length == 1 && nums[0]==target) return 0;
        if(nums.length == 1 && nums[0]!=target) return -1;

        while(left <= right){
            int mid = ( left + right)/2;
            if(nums[mid]==target){
                return mid;
            }
            if(nums[0]<=nums[mid]){
                if(nums[0] <= target && target < nums[mid]){
                    right = mid -1;
                }else{
                    left = mid +1;
                }
            }else{
                if(nums[nums.length -1 ] >= target && target > nums[mid]){
                    left = mid + 1;
                }else{
                    right = mid - 1;
                }
            }
        }
        return -1;
    }

    public static void main(String[] args) {
       //long t1 =  System.nanoTime();
        LeetCode_33 leetCode33 = new LeetCode_33();
        leetCode33.test();
      //  System.out.println(System.nanoTime() - t1);
    }
}