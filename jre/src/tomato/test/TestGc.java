package tomato.test;

public class TestGc {
    public int test(){
        createObject();
        for(int i = 0; i < 12000;i++){
            TestGc testGc = new TestGc();
        }
        return Result.SUCCESS ;
    }

    private void createObject(){
        for(int i = 0; i < 9000; i ++){
            TestGc testGc = new TestGc();
        }
    }
}
