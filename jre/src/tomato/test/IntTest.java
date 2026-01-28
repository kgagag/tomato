package tomato.test;

public class IntTest {
    public int test(){
        int i = 1000363634;
        StringHelper.print20240503((i * i) +"");
        return i;
    }

    public static void main(String[] args) {
        new IntTest().test();
    }
}
