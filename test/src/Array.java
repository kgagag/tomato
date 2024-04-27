import java.util.ArrayList;
import java.util.List;

public class Array {
    public static void main(String[] args) {
        List<Object> arr = new ArrayList<>();
        createArray(arr,3);
        System.out.println(1);
    }



   static void createArray(List<Object> arr,int dept){
       while (dept == 0) {
           return;
       }
       List l = new ArrayList();
       arr.add(l);
       createArray(l,dept -1);
   }
}
