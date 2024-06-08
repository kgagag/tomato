public class Test71 {
    public void test(){
        try {
            throw new RuntimeException("异常");
        }catch (Exception e){
          fix(111);
        }finally {
            fix(200);
            int a = 1 / 0;
        }
    }

    public void  fix(int a){
        System.out.println(a);
    }
}
