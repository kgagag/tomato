package test;

import java.io.IOException;
import java.io.OutputStream;
import java.net.ServerSocket;
import java.net.Socket;
import java.io.InputStream;
import java.io.BufferedReader;
import java.io.InputStreamReader;

public class MyHttpServer2 {

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

    public static void main(String[] args) {
        MyHttpServer2 myHttpServer2 = new MyHttpServer2();
        int port = 8080;
        try (ServerSocket serverSocket = new ServerSocket(port)) {
            System.out.println("Server is listening on port " + port);

            while (true) {
                try (Socket socket = serverSocket.accept()) {
                    myHttpServer2.handleClient(socket);
                } catch (IOException e) {
                    System.out.println("Server exception: " + e.getMessage());
                    e.printStackTrace();
                }
            }
        } catch (IOException e) {
            System.out.println("Could not listen on port " + port);
            e.printStackTrace();
        }
    }

    private  void handleClient(Socket socket) throws IOException {
        InputStream input = socket.getInputStream();
        BufferedReader reader = new BufferedReader(new InputStreamReader(input));
        OutputStream output = socket.getOutputStream();

        // Read the request line (e.g., "GET / HTTP/1.1")
        String requestLine = reader.readLine();
        System.out.println(requestLine);

        // Read and discard the rest of the request headers
        while (true) {
            String headerLine = reader.readLine();
            if (headerLine == null || headerLine.isEmpty()) {
                break;
            }
        }

        // Prepare the response
        String httpResponse = "HTTP/1.1 200 OK\r\n" +
                "Content-Type: text/html\r\n" +
                "Content-Length: " + response.length() + "\r\n" +
                "\r\n" +
                response;

        // Send the response
        output.write(httpResponse.getBytes());
        output.flush();
    }
}
