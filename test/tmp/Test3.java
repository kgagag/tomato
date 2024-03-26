public class Test3{
   public int add (int[] a ,int[] b){
      int c = 0;
      for(int i : a){
         c += i;
      }

      for(int i : b){
         c += i;
      }

      return c;
   }

   public static void main(String[] args) {
      int[] arr = new int[]{1,2,3,4,5,6};
      int[] brr = new int[]{7,8,9,10};
      Test3 test3 = new Test3();
      int c = test3.add(arr, brr);
      int d = c + 100;
   }
}