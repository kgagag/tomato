public class TestGc {
    public void test(){
        createObject();
        for(int i = 0; i < 12000;i++){
            TestGc testGc = new TestGc();
        }
    }

    private void createObject(){
        for(int i = 0; i < 9000; i ++){
            TestGc testGc = new TestGc();
        }
    }
}
