package tomato.test;

public class Test35 {
    public int test(char a, int c , long d , float e , double f){
        double g = a  + c + d + e+ f;

        return  67.0 == g ? Result.SUCCESS : Result.FAILED;
    }

    public static void main(String[] args) {
        System.out.println( new Test35().test('1',3,4,5.0f,6d) );
    }
}
