package tomato.test;

import java.io.IOException;
import tomato.test.net.ServerSocket;
import tomato.test.net.Socket;

public class SimpleHttpServer {

    String response =  "<!DOCTYPE html>" +
            "<html>" +
            "<head>" +
            "<meta charset = 'utf8'>"+
            "<title>我的主页</title>" +
            "<style>" +
            "body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; background-color: #f7f7f7; margin: 0; padding: 0; color: #333; }" +
            "header { background-color: #4CAF50; color: white; padding: 20px 0; text-align: center; font-size: 24px; }" +
            "nav ul { list-style: none; padding: 0; text-align: center; background-color: #333; margin: 0; }" +
            "nav ul li { display: inline; margin: 0; }" +
            "nav ul li a { color: white; text-decoration: none; padding: 14px 20px; display: inline-block; }" +
            "nav ul li a:hover { background-color: #4CAF50; }" +
            "main { padding: 20px; text-align: center; }" +
            "footer { background-color: #333; color: white; text-align: center; padding: 10px 0; position: relative; bottom: 0; width: 100%; margin-top: 20px; }" +
            "</style>" +
            "</head>" +
            "<body>" +
            "<header><h1>欢迎来到我的主页!</h1></header>" +
            "<nav>" +
            "<ul>" +
            "<li><a href='/about'>关于我们</a></li>" +
            "<li><a href='/contact'>联系我们</a></li>" +
            "</ul>" +
            "</nav>" +
            "<main>" +
            "<p>这是一个简单的 HTTP 服务器示例。</p>" +
            "</main>" +
            "<footer>" +
            "<p>&copy; 2024 我的 HTTP 服务器</p>" +
            "</footer>" +
            "</body>" +
            "</html>";

    public void test() {
        int port = 8080;
        try{
            ServerSocket serverSocket = new ServerSocket(port);
            StringHelper.print20240503("HTTP 服务器正在运行，端口：" + port);
            while (true) {
                try  {
                    Socket clientSocket = serverSocket.accept();
                    handleClientRequest(clientSocket);
                } catch (Exception e) {
                    StringHelper.print20240503("客户端请求处理失败: " + e.getMessage());
                }
            }
        } catch (Exception e) {
            StringHelper.print20240503("服务器启动失败: " + e.getMessage());
        }
    }

    private  void handleClientRequest(Socket clientSocket) {
        // 构建 HTTP 响应
        try {
            clientSocket.getOutputStream().write(getHomePage().getBytes());
        } catch (IOException e) {
            throw new RuntimeException(e);
        }finally {
            clientSocket.close();
        }
    }

    private  String getHomePage() {
       // String response = "<html><body><h1>Welcome to the Home Page</h1></body></html>";
        String httpResponse = "HTTP/1.1 200 OK\r\n" +
                "Content-Type: text/html\r\n" +
                "Content-Length: " + response.getBytes().length + "\r\n" +
                "\r\n" +
                response;
        return httpResponse;
    }

    public static void main(String[] args) {
//        SimpleHttpServer simpleHttpServer = new SimpleHttpServer();
//        simpleHttpServer.tomato.test();
        System.out.println(0x0100 & 258);
        System.out.println(Integer.toBinaryString(258));
    }
}
