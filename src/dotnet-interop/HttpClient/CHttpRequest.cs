using System.Runtime.InteropServices;

namespace HttpClientInterop;

[StructLayout(LayoutKind.Sequential)]

public struct CHttpRequest
{
    [MarshalAs(UnmanagedType.LPUTF8Str)]
    public string host;

    [MarshalAs(UnmanagedType.LPUTF8Str)]
    public string path;

}