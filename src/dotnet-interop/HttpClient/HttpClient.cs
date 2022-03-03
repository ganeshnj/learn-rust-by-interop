using System.Runtime.InteropServices;

namespace HttpClientInterop;

public class HttpClient
{

    [DllImport("http_client.dll")]
    private static extern CHttpResponse http_client_get(CHttpRequest request);

    public HttpResponse Get(HttpRequest request)
    {
        var cRequest = new CHttpRequest
        {
            host = request.Host,
            path = request.Path,
        };

        var cResponse = http_client_get(cRequest);
        var response = new HttpResponse
        {
            Status = Marshal.PtrToStringUTF8(cResponse.status),
            Body = Marshal.PtrToStringUTF8(cResponse.body),
        };
        return response;
    }
}